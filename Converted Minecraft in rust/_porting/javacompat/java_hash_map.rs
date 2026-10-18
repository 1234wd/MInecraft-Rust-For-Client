//! Port of: (no Java counterpart -- see module docs)
//! Java class(es): `java.util.HashMap` (JDK 25; the subset `CompoundTag` uses), `java.util.AbstractMap#equals`/`#hashCode`
//! Status: VERIFIED
//!
//! Session 23. A map that reproduces `java.util.HashMap`'s **iteration order**, because
//! `CompoundTag.write` serialises in map order and `CompoundTag.copy()` re-inserts in it: two
//! maps with the same entries but a different history iterate differently, and that difference
//! is in the bytes of every world save. A Rust `HashMap` (random order) or `BTreeMap` (sorted)
//! would produce valid NBT with the wrong byte layout.
//!
//! # What is ported
//!
//! The algorithm, from the JDK 25 source (`java.base/java/util/HashMap.java`), operation for
//! operation:
//!
//! * `hash(key)`: `h ^ (h >>> 16)`, and the bucket index `(n - 1) & hash`.
//! * the lazily-allocated table, the 0.75 load factor, `resize()`'s doubling and its
//!   order-preserving lo/hi split of each bucket;
//! * `putVal`, `getNode`, `removeNode`, `clear()` (which keeps the table length);
//! * the copy constructor `HashMap(Map)` -- `putMapEntries` presizes with
//!   `ceil(s / loadFactor)`, the JDK 19+ formula;
//! * **tree bins**: a bucket that reaches 9 entries in a table of at least 64 becomes a
//!   red-black tree (`treeifyBin`, `TreeNode.treeify`), and smaller tables resize instead. A tree
//!   bin is still iterated through its `next` list, but `putTreeVal` links a new node right after
//!   its TREE PARENT and `moveRootToFront` moves the root to the head, so iteration order inside
//!   such a bucket depends on the tree's shape. So the tree code is ported in full -- `find`,
//!   `putTreeVal`, `removeTreeNode` (successor swap included), `split` on resize with its
//!   untreeify-at-6 rule, both rotations, `balanceInsertion` and `balanceDeletion`;
//! * `AbstractMap.equals` (same size, every entry present and equal -- order-independent) and
//!   `AbstractMap.hashCode` (the wrapping sum of `key.hashCode() ^ value.hashCode()`).
//!
//! The node graph lives in an index arena (`Vec` slots plus a free list) instead of object
//! pointers; each `TreeNode` field (`parent`, `left`, `right`, `prev`, `red`) is a field of the
//! slot, live only while the slot is a tree node. `untreeify`/`replacementNode` keep the slot
//! and clear the tree fields, which is the same order and the same data.
//!
//! # What is not
//!
//! * Null keys and null values: none of the NBT call sites store either (`CompoundTag.write`
//!   would throw on one), and a Rust `K`/`V` cannot be null.
//! * `putAll`, `putIfAbsent`, `compute*`, `merge`, `replaceAll`, the views' own mutators,
//!   `clone()`, serialisation, and the constructors taking an initial capacity or load factor.
//!   `CompoundTag` calls none of them.
//! * `modCount` / fail-fast iterators: Rust's borrow checker already forbids mutating a map
//!   while it is being iterated, which is the only thing `ConcurrentModificationException`
//!   reports.
//! * `TreeNode.tieBreakOrder`: it is reached only when two keys with equal hashes compare as
//!   EQUAL but are not `equals`. For `String` and `Integer` keys that cannot happen, and for a key
//!   that could it would consult `System.identityHashCode`, which is not portable. The port
//!   panics there rather than invent an order. Every [`JavaHashKey`] in this crate is consistent.
//!
//! Rust's `Clone` duplicates the table as it stands -- the same order -- and is NOT Java's
//! `HashMap.clone()` (which re-inserts). [`JavaHashMap::copy_of`] is the copy constructor.
//! [`JavaHashMap::table_length`] exposes the private table length, which the oracle reads by
//! reflection; nothing in vanilla can see it.
//!
//! Checked against JDK 25.0.4.1 by the `HMOP`/`HMRES`/`HMCOPY`/`HMINT`/`HMINTRES` and
//! `HMSTEP`/`HMEQ` groups of `_porting/test-data/nbt_probe.txt` (`tests/session23.rs`): every
//! step's table length, size, return value and full iteration order, through colliding keys,
//! treeification, tree splits, tree removals down to untreeify, clears and copies.

use std::borrow::Borrow;

use crate::javacompat::java_lang;

/// Port of `HashMap.DEFAULT_INITIAL_CAPACITY`.
const DEFAULT_INITIAL_CAPACITY: i32 = 1 << 4;
/// Port of `HashMap.MAXIMUM_CAPACITY`.
const MAXIMUM_CAPACITY: i32 = 1 << 30;
/// Port of `HashMap.DEFAULT_LOAD_FACTOR`.
const DEFAULT_LOAD_FACTOR: f32 = 0.75;
/// Port of `HashMap.TREEIFY_THRESHOLD`.
const TREEIFY_THRESHOLD: i32 = 8;
/// Port of `HashMap.UNTREEIFY_THRESHOLD`.
const UNTREEIFY_THRESHOLD: i32 = 6;
/// Port of `HashMap.MIN_TREEIFY_CAPACITY`.
const MIN_TREEIFY_CAPACITY: i32 = 64;

/// A key type that a [`JavaHashMap`] can hold: Java's `hashCode()` and, because every key type
/// the NBT code uses is `Comparable` to itself, `compareTo`.
///
/// `java_compare_to` must return 0 only for keys that are `==`; see the module docs on
/// `tieBreakOrder`.
pub trait JavaHashKey {
    /// Port of `hashCode()`.
    fn java_hash_code(&self) -> i32;
    /// Port of `compareTo(other)`. Only the sign is used.
    fn java_compare_to(&self, other: &Self) -> i32;
}

impl JavaHashKey for str {
    /// `String.hashCode()`, over UTF-16 code units.
    fn java_hash_code(&self) -> i32 {
        java_lang::string_hash_code(self)
    }

    /// `String.compareTo`, over UTF-16 code units.
    fn java_compare_to(&self, other: &Self) -> i32 {
        java_lang::string_compare_to(self, other)
    }
}

impl JavaHashKey for String {
    fn java_hash_code(&self) -> i32 {
        self.as_str().java_hash_code()
    }

    fn java_compare_to(&self, other: &Self) -> i32 {
        self.as_str().java_compare_to(other.as_str())
    }
}

impl JavaHashKey for i32 {
    /// `Integer.hashCode()` is the value itself.
    fn java_hash_code(&self) -> i32 {
        *self
    }

    /// `Integer.compareTo`: `Integer.compare(x, y)`.
    fn java_compare_to(&self, other: &Self) -> i32 {
        match self.cmp(other) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

/// Port of `HashMap.hash(Object)` for a non-null key: `h ^ (h >>> 16)`.
fn spread(h: i32) -> i32 {
    h ^ ((h as u32) >> 16) as i32
}

/// Port of `HashMap.tableSizeFor(int)`.
///
/// `-1 >>> numberOfLeadingZeros(cap - 1)`: Java masks a shift count to five bits, so for
/// `cap <= 1` the shift is by 32 and leaves `-1`, which the `n < 0` arm maps to 1.
fn table_size_for(cap: i32) -> i32 {
    let n = (u32::MAX.wrapping_shr(cap.wrapping_sub(1).leading_zeros())) as i32;
    if n < 0 {
        1
    } else if n >= MAXIMUM_CAPACITY {
        MAXIMUM_CAPACITY
    } else {
        n + 1
    }
}

/// `TreeNode.tieBreakOrder`: unreachable for a consistent [`JavaHashKey`] -- see the module docs.
fn tie_break_order() -> i32 {
    panic!(
        "java.util.HashMap.TreeNode.tieBreakOrder: two unequal keys with equal hashes compared as \
         equal; Java would order them by System.identityHashCode, which is not portable"
    )
}

/// One `HashMap.Node` / `HashMap.TreeNode`, by arena index.
#[derive(Clone, Debug)]
struct Node<K, V> {
    hash: i32,
    key: K,
    value: V,
    next: Option<usize>,
    /// `instanceof TreeNode`.
    tree: bool,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    prev: Option<usize>,
    red: bool,
}

/// Port of `java.util.HashMap<K, V>`, reproducing its iteration order. See the module docs.
#[derive(Clone)]
pub struct JavaHashMap<K, V> {
    slots: Vec<Option<Node<K, V>>>,
    free: Vec<usize>,
    /// `transient Node<K,V>[] table`: `None` until the first insertion, as in Java.
    table: Option<Vec<Option<usize>>>,
    size: i32,
    threshold: i32,
    load_factor: f32,
}

impl<K, V> Default for JavaHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> JavaHashMap<K, V> {
    /// Port of `new HashMap<>()`: no table yet, load factor 0.75.
    pub fn new() -> Self {
        JavaHashMap {
            slots: Vec::new(),
            free: Vec::new(),
            table: None,
            size: 0,
            threshold: 0,
            load_factor: DEFAULT_LOAD_FACTOR,
        }
    }

    /// Port of `size()`.
    pub fn size(&self) -> i32 {
        self.size
    }

    /// Port of `isEmpty()`.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// The length of the private `table` array -- 0 before the first insertion.
    ///
    /// Not a Java API: the oracle reads it by reflection, and it is what decides iteration order.
    pub fn table_length(&self) -> i32 {
        self.table.as_ref().map_or(0, |t| t.len() as i32)
    }

    /// Port of `clear()`: empties every bucket but keeps the table, so a cleared map that is
    /// refilled iterates in its old capacity's order. Does nothing to an empty map.
    pub fn clear(&mut self) {
        if let Some(tab) = self.table.as_mut() {
            if self.size > 0 {
                self.size = 0;
                for slot in tab.iter_mut() {
                    *slot = None;
                }
                self.slots.clear();
                self.free.clear();
            }
        }
    }

    /// The entries in `HashMap`'s iteration order: bucket by bucket, each bucket along `next`.
    ///
    /// The order of `keySet()`, `values()`, `entrySet()` and `forEach` alike.
    pub fn iter(&self) -> Iter<'_, K, V> {
        let mut it = Iter {
            map: self,
            index: 0,
            next: None,
        };
        if self.size > 0 {
            it.advance();
        }
        it
    }

    /// Port of `keySet()`'s iteration.
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.iter().map(|(k, _)| k)
    }

    /// Port of `values()`'s iteration.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.iter().map(|(_, v)| v)
    }

    /// Port of `AbstractMap.hashCode()`: the wrapping sum of `key.hashCode() ^ value.hashCode()`.
    pub fn java_hash_code(&self, value_hash: impl Fn(&V) -> i32) -> i32
    where
        K: JavaHashKey,
    {
        self.iter().fold(0i32, |h, (k, v)| {
            h.wrapping_add(k.java_hash_code() ^ value_hash(v))
        })
    }

    // ---------------------------------------------------------------- arena

    fn n(&self, i: usize) -> &Node<K, V> {
        self.slots[i].as_ref().expect("live HashMap node")
    }

    fn nm(&mut self, i: usize) -> &mut Node<K, V> {
        self.slots[i].as_mut().expect("live HashMap node")
    }

    fn alloc(&mut self, node: Node<K, V>) -> usize {
        match self.free.pop() {
            Some(i) => {
                self.slots[i] = Some(node);
                i
            }
            None => {
                self.slots.push(Some(node));
                self.slots.len() - 1
            }
        }
    }

    fn release(&mut self, i: usize) -> Node<K, V> {
        let node = self.slots[i].take().expect("live HashMap node");
        self.free.push(i);
        node
    }

    fn plain_node(hash: i32, key: K, value: V, next: Option<usize>) -> Node<K, V> {
        Node {
            hash,
            key,
            value,
            next,
            tree: false,
            parent: None,
            left: None,
            right: None,
            prev: None,
            red: false,
        }
    }

    fn tab(&self) -> &Vec<Option<usize>> {
        self.table.as_ref().expect("HashMap table")
    }

    fn tab_mut(&mut self) -> &mut Vec<Option<usize>> {
        self.table.as_mut().expect("HashMap table")
    }

    fn bucket(&self, hash: i32) -> usize {
        ((self.tab().len() as i32 - 1) & hash) as usize
    }

    // ---------------------------------------------------------------- tree bins

    /// Port of `TreeNode.root()`.
    fn root_of(&self, mut r: usize) -> usize {
        while let Some(p) = self.n(r).parent {
            r = p;
        }
        r
    }

    /// Port of `TreeNode.moveRootToFront(tab, root)`.
    fn move_root_to_front(&mut self, root: usize) {
        if self.table_length() == 0 {
            return;
        }
        let index = self.bucket(self.n(root).hash);
        let first = self.tab()[index];
        if first != Some(root) {
            self.tab_mut()[index] = Some(root);
            let rp = self.n(root).prev;
            let rn = self.n(root).next;
            if let Some(rn) = rn {
                self.nm(rn).prev = rp;
            }
            if let Some(rp) = rp {
                self.nm(rp).next = rn;
            }
            if let Some(first) = first {
                self.nm(first).prev = Some(root);
            }
            let r = self.nm(root);
            r.next = first;
            r.prev = None;
        }
    }

    /// Port of `TreeNode.rotateLeft(root, p)`.
    fn rotate_left(&mut self, mut root: usize, p: usize) -> usize {
        if let Some(r) = self.n(p).right {
            let rl = self.n(r).left;
            self.nm(p).right = rl;
            if let Some(rl) = rl {
                self.nm(rl).parent = Some(p);
            }
            let pp = self.n(p).parent;
            self.nm(r).parent = pp;
            match pp {
                None => {
                    root = r;
                    self.nm(r).red = false;
                }
                Some(pp) => {
                    if self.n(pp).left == Some(p) {
                        self.nm(pp).left = Some(r);
                    } else {
                        self.nm(pp).right = Some(r);
                    }
                }
            }
            self.nm(r).left = Some(p);
            self.nm(p).parent = Some(r);
        }
        root
    }

    /// Port of `TreeNode.rotateRight(root, p)`.
    fn rotate_right(&mut self, mut root: usize, p: usize) -> usize {
        if let Some(l) = self.n(p).left {
            let lr = self.n(l).right;
            self.nm(p).left = lr;
            if let Some(lr) = lr {
                self.nm(lr).parent = Some(p);
            }
            let pp = self.n(p).parent;
            self.nm(l).parent = pp;
            match pp {
                None => {
                    root = l;
                    self.nm(l).red = false;
                }
                Some(pp) => {
                    if self.n(pp).right == Some(p) {
                        self.nm(pp).right = Some(l);
                    } else {
                        self.nm(pp).left = Some(l);
                    }
                }
            }
            self.nm(l).right = Some(p);
            self.nm(p).parent = Some(l);
        }
        root
    }

    fn is_red(&self, x: Option<usize>) -> bool {
        x.is_some_and(|x| self.n(x).red)
    }

    fn parent_of(&self, x: Option<usize>) -> Option<usize> {
        x.and_then(|x| self.n(x).parent)
    }

    /// Port of `TreeNode.balanceInsertion(root, x)`.
    fn balance_insertion(&mut self, mut root: usize, mut x: usize) -> usize {
        self.nm(x).red = true;
        loop {
            let Some(mut xp) = self.n(x).parent else {
                self.nm(x).red = false;
                return x;
            };
            if !self.n(xp).red {
                return root;
            }
            let Some(mut xpp) = self.n(xp).parent else {
                return root;
            };
            let xppl = self.n(xpp).left;
            if Some(xp) == xppl {
                let xppr = self.n(xpp).right;
                if self.is_red(xppr) {
                    self.nm(xppr.unwrap()).red = false;
                    self.nm(xp).red = false;
                    self.nm(xpp).red = true;
                    x = xpp;
                } else {
                    let mut xpp_opt = Some(xpp);
                    let mut xp_opt = Some(xp);
                    if Some(x) == self.n(xp).right {
                        x = xp;
                        root = self.rotate_left(root, x);
                        xp_opt = self.n(x).parent;
                        xpp_opt = self.parent_of(xp_opt);
                    }
                    if let Some(p) = xp_opt {
                        xp = p;
                        self.nm(xp).red = false;
                        if let Some(pp) = xpp_opt {
                            xpp = pp;
                            self.nm(xpp).red = true;
                            root = self.rotate_right(root, xpp);
                        }
                    }
                }
            } else if self.is_red(xppl) {
                self.nm(xppl.unwrap()).red = false;
                self.nm(xp).red = false;
                self.nm(xpp).red = true;
                x = xpp;
            } else {
                let mut xpp_opt = Some(xpp);
                let mut xp_opt = Some(xp);
                if Some(x) == self.n(xp).left {
                    x = xp;
                    root = self.rotate_right(root, x);
                    xp_opt = self.n(x).parent;
                    xpp_opt = self.parent_of(xp_opt);
                }
                if let Some(p) = xp_opt {
                    xp = p;
                    self.nm(xp).red = false;
                    if let Some(pp) = xpp_opt {
                        xpp = pp;
                        self.nm(xpp).red = true;
                        root = self.rotate_left(root, xpp);
                    }
                }
            }
        }
    }

    /// Port of `TreeNode.balanceDeletion(root, x)`.
    fn balance_deletion(&mut self, mut root: usize, x0: usize) -> usize {
        let mut x = Some(x0);
        loop {
            let Some(xi) = x else {
                return root;
            };
            if xi == root {
                return root;
            }
            let Some(xp0) = self.n(xi).parent else {
                self.nm(xi).red = false;
                return xi;
            };
            if self.n(xi).red {
                self.nm(xi).red = false;
                return root;
            }
            let mut xp = Some(xp0);
            let xpl = self.n(xp0).left;
            if xpl == Some(xi) {
                let mut xpr = self.n(xp0).right;
                if self.is_red(xpr) {
                    self.nm(xpr.unwrap()).red = false;
                    self.nm(xp0).red = true;
                    root = self.rotate_left(root, xp0);
                    xp = self.n(xi).parent;
                    xpr = xp.and_then(|p| self.n(p).right);
                }
                match xpr {
                    None => x = xp,
                    Some(xpr_i) => {
                        let sl = self.n(xpr_i).left;
                        let sr = self.n(xpr_i).right;
                        if !self.is_red(sr) && !self.is_red(sl) {
                            self.nm(xpr_i).red = true;
                            x = xp;
                        } else {
                            let mut xpr = Some(xpr_i);
                            if !self.is_red(sr) {
                                if let Some(sl) = sl {
                                    self.nm(sl).red = false;
                                }
                                self.nm(xpr_i).red = true;
                                root = self.rotate_right(root, xpr_i);
                                xp = self.n(xi).parent;
                                xpr = xp.and_then(|p| self.n(p).right);
                            }
                            if let Some(xpr) = xpr {
                                let red = xp.is_some_and(|p| self.n(p).red);
                                self.nm(xpr).red = red;
                                if let Some(sr) = self.n(xpr).right {
                                    self.nm(sr).red = false;
                                }
                            }
                            if let Some(p) = xp {
                                self.nm(p).red = false;
                                root = self.rotate_left(root, p);
                            }
                            x = Some(root);
                        }
                    }
                }
            } else {
                // Symmetric.
                let mut xpl = xpl;
                if self.is_red(xpl) {
                    self.nm(xpl.unwrap()).red = false;
                    self.nm(xp0).red = true;
                    root = self.rotate_right(root, xp0);
                    xp = self.n(xi).parent;
                    xpl = xp.and_then(|p| self.n(p).left);
                }
                match xpl {
                    None => x = xp,
                    Some(xpl_i) => {
                        let sl = self.n(xpl_i).left;
                        let sr = self.n(xpl_i).right;
                        if !self.is_red(sl) && !self.is_red(sr) {
                            self.nm(xpl_i).red = true;
                            x = xp;
                        } else {
                            let mut xpl = Some(xpl_i);
                            if !self.is_red(sl) {
                                if let Some(sr) = sr {
                                    self.nm(sr).red = false;
                                }
                                self.nm(xpl_i).red = true;
                                root = self.rotate_left(root, xpl_i);
                                xp = self.n(xi).parent;
                                xpl = xp.and_then(|p| self.n(p).left);
                            }
                            if let Some(xpl) = xpl {
                                let red = xp.is_some_and(|p| self.n(p).red);
                                self.nm(xpl).red = red;
                                if let Some(sl) = self.n(xpl).left {
                                    self.nm(sl).red = false;
                                }
                            }
                            if let Some(p) = xp {
                                self.nm(p).red = false;
                                root = self.rotate_right(root, p);
                            }
                            x = Some(root);
                        }
                    }
                }
            }
        }
    }

    /// Port of `TreeNode.untreeify(map)`: the same nodes, in `next` order, as plain nodes.
    fn untreeify(&mut self, head: usize) -> usize {
        let mut q = Some(head);
        while let Some(qi) = q {
            let n = self.nm(qi);
            n.tree = false;
            n.parent = None;
            n.left = None;
            n.right = None;
            n.prev = None;
            n.red = false;
            q = n.next;
        }
        head
    }
}

impl<K: JavaHashKey + Eq, V> JavaHashMap<K, V> {
    // ---------------------------------------------------------------- resize

    /// Port of `resize()`.
    fn resize(&mut self) {
        let old_cap = self.table_length();
        let old_thr = self.threshold;
        let mut new_cap;
        let mut new_thr = 0;
        if old_cap > 0 {
            if old_cap >= MAXIMUM_CAPACITY {
                self.threshold = i32::MAX;
                return;
            }
            new_cap = old_cap << 1;
            if new_cap < MAXIMUM_CAPACITY && old_cap >= DEFAULT_INITIAL_CAPACITY {
                new_thr = old_thr << 1;
            }
        } else if old_thr > 0 {
            new_cap = old_thr;
        } else {
            new_cap = DEFAULT_INITIAL_CAPACITY;
            new_thr = (DEFAULT_LOAD_FACTOR * DEFAULT_INITIAL_CAPACITY as f32) as i32;
        }
        if new_thr == 0 {
            let ft = new_cap as f32 * self.load_factor;
            new_thr = if new_cap < MAXIMUM_CAPACITY && ft < MAXIMUM_CAPACITY as f32 {
                ft as i32
            } else {
                i32::MAX
            };
        }
        self.threshold = new_thr;
        // Java assigns `table = newTab` before moving anything, and `split`/`treeify` work on it.
        let old_tab = self.table.replace(vec![None; new_cap as usize]);
        let Some(mut old_tab) = old_tab else {
            return;
        };
        for j in 0..old_cap as usize {
            let Some(e) = old_tab[j].take() else {
                continue;
            };
            if self.n(e).next.is_none() {
                let idx = (self.n(e).hash & (new_cap - 1)) as usize;
                self.tab_mut()[idx] = Some(e);
            } else if self.n(e).tree {
                self.split(e, j, old_cap);
            } else {
                // Preserve order.
                let (mut lo_head, mut lo_tail) = (None, None::<usize>);
                let (mut hi_head, mut hi_tail) = (None, None::<usize>);
                let mut cur = Some(e);
                while let Some(c) = cur {
                    let next = self.n(c).next;
                    if self.n(c).hash & old_cap == 0 {
                        match lo_tail {
                            None => lo_head = Some(c),
                            Some(t) => self.nm(t).next = Some(c),
                        }
                        lo_tail = Some(c);
                    } else {
                        match hi_tail {
                            None => hi_head = Some(c),
                            Some(t) => self.nm(t).next = Some(c),
                        }
                        hi_tail = Some(c);
                    }
                    cur = next;
                }
                if let Some(t) = lo_tail {
                    self.nm(t).next = None;
                    self.tab_mut()[j] = lo_head;
                }
                if let Some(t) = hi_tail {
                    self.nm(t).next = None;
                    self.tab_mut()[j + old_cap as usize] = hi_head;
                }
            }
        }
    }

    /// Port of `TreeNode.find(h, k, kc)`.
    fn find<Q>(&self, start: usize, h: i32, k: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: JavaHashKey + Eq + ?Sized,
    {
        let mut p = Some(start);
        while let Some(pi) = p {
            let node = self.n(pi);
            let (pl, pr) = (node.left, node.right);
            let ph = node.hash;
            if ph > h {
                p = pl;
            } else if ph < h {
                p = pr;
            } else if node.key.borrow() == k {
                return Some(pi);
            } else if pl.is_none() {
                p = pr;
            } else if pr.is_none() {
                p = pl;
            } else {
                let dir = k.java_compare_to(node.key.borrow());
                if dir != 0 {
                    p = if dir < 0 { pl } else { pr };
                } else if let Some(q) = self.find(pr.unwrap(), h, k) {
                    return Some(q);
                } else {
                    p = pl;
                }
            }
        }
        None
    }

    /// Port of `TreeNode.getTreeNode(h, k)`, called on a bucket's first node.
    fn get_tree_node<Q>(&self, first: usize, h: i32, k: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: JavaHashKey + Eq + ?Sized,
    {
        let start = if self.n(first).parent.is_some() {
            self.root_of(first)
        } else {
            first
        };
        self.find(start, h, k)
    }

    /// Port of `TreeNode.treeify(tab)`, called on the head of a bucket of tree nodes.
    fn treeify(&mut self, head: usize) {
        let mut root: Option<usize> = None;
        let mut x = Some(head);
        while let Some(xi) = x {
            let next = self.n(xi).next;
            {
                let n = self.nm(xi);
                n.left = None;
                n.right = None;
            }
            match root {
                None => {
                    let n = self.nm(xi);
                    n.parent = None;
                    n.red = false;
                    root = Some(xi);
                }
                Some(r) => {
                    let h = self.n(xi).hash;
                    let mut p = r;
                    loop {
                        let ph = self.n(p).hash;
                        let dir = if ph > h {
                            -1
                        } else if ph < h {
                            1
                        } else {
                            match self.n(xi).key.java_compare_to(&self.n(p).key) {
                                0 => tie_break_order(),
                                d => d,
                            }
                        };
                        let xp = p;
                        let child = if dir <= 0 {
                            self.n(p).left
                        } else {
                            self.n(p).right
                        };
                        match child {
                            Some(c) => p = c,
                            None => {
                                self.nm(xi).parent = Some(xp);
                                if dir <= 0 {
                                    self.nm(xp).left = Some(xi);
                                } else {
                                    self.nm(xp).right = Some(xi);
                                }
                                root = Some(self.balance_insertion(r, xi));
                                break;
                            }
                        }
                    }
                }
            }
            x = next;
        }
        if let Some(root) = root {
            self.move_root_to_front(root);
        }
    }

    /// Port of `TreeNode.putTreeVal(map, tab, h, k, v)`, called on a bucket's first node.
    ///
    /// Returns the existing node and hands the value back when the key is already present;
    /// `None` when a new node was linked in.
    fn put_tree_val(&mut self, first: usize, h: i32, key: K, value: V) -> Option<(usize, V)> {
        let mut searched = false;
        let root = if self.n(first).parent.is_some() {
            self.root_of(first)
        } else {
            first
        };
        let mut p = root;
        loop {
            let ph = self.n(p).hash;
            let dir;
            if ph > h {
                dir = -1;
            } else if ph < h {
                dir = 1;
            } else if self.n(p).key == key {
                return Some((p, value));
            } else {
                let d = key.java_compare_to(&self.n(p).key);
                if d == 0 {
                    if !searched {
                        searched = true;
                        if let Some(ch) = self.n(p).left {
                            if let Some(q) = self.find(ch, h, &key) {
                                return Some((q, value));
                            }
                        }
                        if let Some(ch) = self.n(p).right {
                            if let Some(q) = self.find(ch, h, &key) {
                                return Some((q, value));
                            }
                        }
                    }
                    dir = tie_break_order();
                } else {
                    dir = d;
                }
            }
            let xp = p;
            let child = if dir <= 0 {
                self.n(xp).left
            } else {
                self.n(xp).right
            };
            match child {
                Some(c) => p = c,
                None => {
                    let xpn = self.n(xp).next;
                    let x = self.alloc(Node {
                        hash: h,
                        key,
                        value,
                        next: xpn,
                        tree: true,
                        parent: Some(xp),
                        left: None,
                        right: None,
                        prev: Some(xp),
                        red: false,
                    });
                    if dir <= 0 {
                        self.nm(xp).left = Some(x);
                    } else {
                        self.nm(xp).right = Some(x);
                    }
                    self.nm(xp).next = Some(x);
                    if let Some(xpn) = xpn {
                        self.nm(xpn).prev = Some(x);
                    }
                    let r = self.balance_insertion(root, x);
                    self.move_root_to_front(r);
                    return None;
                }
            }
        }
    }

    /// Port of `TreeNode.removeTreeNode(map, tab, movable)` with `movable = true` (the only
    /// value `HashMap.remove` passes). Unlinks `node` from its bucket; the caller frees it.
    fn remove_tree_node(&mut self, node: usize) {
        let movable = true;
        if self.table_length() == 0 {
            return;
        }
        let index = self.bucket(self.n(node).hash);
        let mut first = self.tab()[index];
        // `root = first`: the OLD head, which may be `node` itself.
        let mut root = first;
        let succ = self.n(node).next;
        let pred = self.n(node).prev;
        match pred {
            None => {
                self.tab_mut()[index] = succ;
                first = succ;
            }
            Some(pred) => self.nm(pred).next = succ,
        }
        if let Some(succ) = succ {
            self.nm(succ).prev = pred;
        }
        let Some(first) = first else {
            return;
        };
        if let Some(r) = root {
            if self.n(r).parent.is_some() {
                root = Some(self.root_of(r));
            }
        }
        let too_small = match root {
            None => true,
            Some(r) => {
                movable
                    && (self.n(r).right.is_none()
                        || match self.n(r).left {
                            None => true,
                            Some(rl) => self.n(rl).left.is_none(),
                        })
            }
        };
        if too_small {
            let head = self.untreeify(first);
            self.tab_mut()[index] = Some(head);
            return;
        }
        let mut root = root.unwrap();
        let p = node;
        let pl = self.n(p).left;
        let pr = self.n(p).right;
        let replacement;
        if let (Some(pl), Some(pr)) = (pl, pr) {
            let mut s = pr;
            while let Some(sl) = self.n(s).left {
                s = sl;
            }
            // Swap colours.
            let c = self.n(s).red;
            let p_red = self.n(p).red;
            self.nm(s).red = p_red;
            self.nm(p).red = c;
            let sr = self.n(s).right;
            let pp = self.n(p).parent;
            if s == pr {
                // `p` was `s`'s direct parent.
                self.nm(p).parent = Some(s);
                self.nm(s).right = Some(p);
            } else {
                let sp = self.n(s).parent;
                self.nm(p).parent = sp;
                if let Some(sp) = sp {
                    if self.n(sp).left == Some(s) {
                        self.nm(sp).left = Some(p);
                    } else {
                        self.nm(sp).right = Some(p);
                    }
                }
                self.nm(s).right = Some(pr);
                self.nm(pr).parent = Some(s);
            }
            self.nm(p).left = None;
            self.nm(p).right = sr;
            if let Some(sr) = sr {
                self.nm(sr).parent = Some(p);
            }
            self.nm(s).left = Some(pl);
            self.nm(pl).parent = Some(s);
            self.nm(s).parent = pp;
            match pp {
                None => root = s,
                Some(pp) => {
                    if self.n(pp).left == Some(p) {
                        self.nm(pp).left = Some(s);
                    } else {
                        self.nm(pp).right = Some(s);
                    }
                }
            }
            replacement = sr.unwrap_or(p);
        } else if let Some(pl) = pl {
            replacement = pl;
        } else if let Some(pr) = pr {
            replacement = pr;
        } else {
            replacement = p;
        }
        if replacement != p {
            let pp = self.n(p).parent;
            self.nm(replacement).parent = pp;
            match pp {
                None => {
                    root = replacement;
                    self.nm(replacement).red = false;
                }
                Some(pp) => {
                    if self.n(pp).left == Some(p) {
                        self.nm(pp).left = Some(replacement);
                    } else {
                        self.nm(pp).right = Some(replacement);
                    }
                }
            }
            let pn = self.nm(p);
            pn.left = None;
            pn.right = None;
            pn.parent = None;
        }
        let r = if self.n(p).red {
            root
        } else {
            self.balance_deletion(root, replacement)
        };
        if replacement == p {
            // Detach.
            let pp = self.n(p).parent;
            self.nm(p).parent = None;
            if let Some(pp) = pp {
                if self.n(pp).left == Some(p) {
                    self.nm(pp).left = None;
                } else if self.n(pp).right == Some(p) {
                    self.nm(pp).right = None;
                }
            }
        }
        if movable {
            self.move_root_to_front(r);
        }
    }

    /// Port of `TreeNode.split(map, tab, index, bit)`, run by `resize` on a tree bucket.
    fn split(&mut self, b: usize, index: usize, bit: i32) {
        let (mut lo_head, mut lo_tail) = (None::<usize>, None::<usize>);
        let (mut hi_head, mut hi_tail) = (None::<usize>, None::<usize>);
        let (mut lc, mut hc) = (0, 0);
        let mut e = Some(b);
        while let Some(ei) = e {
            let next = self.n(ei).next;
            self.nm(ei).next = None;
            if self.n(ei).hash & bit == 0 {
                self.nm(ei).prev = lo_tail;
                match lo_tail {
                    None => lo_head = Some(ei),
                    Some(t) => self.nm(t).next = Some(ei),
                }
                lo_tail = Some(ei);
                lc += 1;
            } else {
                self.nm(ei).prev = hi_tail;
                match hi_tail {
                    None => hi_head = Some(ei),
                    Some(t) => self.nm(t).next = Some(ei),
                }
                hi_tail = Some(ei);
                hc += 1;
            }
            e = next;
        }
        if let Some(lo) = lo_head {
            if lc <= UNTREEIFY_THRESHOLD {
                let head = self.untreeify(lo);
                self.tab_mut()[index] = Some(head);
            } else {
                self.tab_mut()[index] = Some(lo);
                // Else it is already treeified.
                if hi_head.is_some() {
                    self.treeify(lo);
                }
            }
        }
        if let Some(hi) = hi_head {
            let at = index + bit as usize;
            if hc <= UNTREEIFY_THRESHOLD {
                let head = self.untreeify(hi);
                self.tab_mut()[at] = Some(head);
            } else {
                self.tab_mut()[at] = Some(hi);
                if lo_head.is_some() {
                    self.treeify(hi);
                }
            }
        }
    }

    /// Port of `treeifyBin(tab, hash)`: resize while the table is under 64, else turn the bucket
    /// into tree nodes (in `next` order, with `prev` links) and build the tree.
    fn treeify_bin(&mut self, hash: i32) {
        let n = self.table_length();
        if n < MIN_TREEIFY_CAPACITY {
            self.resize();
            return;
        }
        let index = self.bucket(hash);
        let Some(hd) = self.tab()[index] else {
            return;
        };
        let mut tl: Option<usize> = None;
        let mut e = Some(hd);
        while let Some(ei) = e {
            let next = self.n(ei).next;
            let node = self.nm(ei);
            node.tree = true;
            node.parent = None;
            node.left = None;
            node.right = None;
            node.red = false;
            node.prev = tl;
            tl = Some(ei);
            e = next;
        }
        self.treeify(hd);
    }

    // ---------------------------------------------------------------- the public operations

    /// Port of `getNode(key)`.
    fn get_node<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: JavaHashKey + Eq + ?Sized,
    {
        let tab = self.table.as_ref()?;
        if tab.is_empty() {
            return None;
        }
        let hash = spread(key.java_hash_code());
        let first = tab[self.bucket(hash)]?;
        let f = self.n(first);
        if f.hash == hash && f.key.borrow() == key {
            return Some(first);
        }
        let mut e = f.next?;
        if f.tree {
            return self.get_tree_node(first, hash, key);
        }
        loop {
            let n = self.n(e);
            if n.hash == hash && n.key.borrow() == key {
                return Some(e);
            }
            e = n.next?;
        }
    }

    /// Port of `get(key)`.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: JavaHashKey + Eq + ?Sized,
    {
        self.get_node(key).map(|i| &self.n(i).value)
    }

    /// `get(key)`, for mutating the value in place. Java hands out the live object either way.
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: JavaHashKey + Eq + ?Sized,
    {
        let i = self.get_node(key)?;
        Some(&mut self.nm(i).value)
    }

    /// Port of `containsKey(key)`.
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: JavaHashKey + Eq + ?Sized,
    {
        self.get_node(key).is_some()
    }

    /// Port of `put(key, value)` (`putVal(hash(key), key, value, false, true)`).
    ///
    /// Returns the previous value. When the key is already present the value is replaced in its
    /// node -- iteration order does not change -- and the map keeps its OLD key object, as Java's
    /// does.
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        let hash = spread(key.java_hash_code());
        if self.table_length() == 0 {
            self.resize();
        }
        let i = self.bucket(hash);
        match self.tab()[i] {
            None => {
                let node = self.alloc(Self::plain_node(hash, key, value, None));
                self.tab_mut()[i] = Some(node);
            }
            Some(p0) => {
                let first = self.n(p0);
                if first.hash == hash && first.key == key {
                    return Some(std::mem::replace(&mut self.nm(p0).value, value));
                } else if first.tree {
                    if let Some((e, value)) = self.put_tree_val(p0, hash, key, value) {
                        return Some(std::mem::replace(&mut self.nm(e).value, value));
                    }
                } else {
                    let mut p = p0;
                    let mut bin_count = 0;
                    loop {
                        match self.n(p).next {
                            None => {
                                let node = self.alloc(Self::plain_node(hash, key, value, None));
                                self.nm(p).next = Some(node);
                                if bin_count >= TREEIFY_THRESHOLD - 1 {
                                    self.treeify_bin(hash);
                                }
                                break;
                            }
                            Some(e) => {
                                let n = self.n(e);
                                if n.hash == hash && n.key == key {
                                    return Some(std::mem::replace(&mut self.nm(e).value, value));
                                }
                                p = e;
                            }
                        }
                        bin_count += 1;
                    }
                }
            }
        }
        self.size += 1;
        if self.size > self.threshold {
            self.resize();
        }
        None
    }

    /// Port of `remove(key)` (`removeNode(hash(key), key, null, false, true)`).
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: JavaHashKey + Eq + ?Sized,
    {
        if self.table_length() == 0 {
            return None;
        }
        let hash = spread(key.java_hash_code());
        let index = self.bucket(hash);
        let p0 = self.tab()[index]?;
        let mut p = p0;
        let mut node = None;
        let first = self.n(p0);
        if first.hash == hash && first.key.borrow() == key {
            node = Some(p0);
        } else if let Some(e0) = first.next {
            if first.tree {
                node = self.get_tree_node(p0, hash, key);
            } else {
                let mut e = Some(e0);
                while let Some(ei) = e {
                    let n = self.n(ei);
                    if n.hash == hash && n.key.borrow() == key {
                        node = Some(ei);
                        break;
                    }
                    p = ei;
                    e = n.next;
                }
            }
        }
        let node = node?;
        if self.n(node).tree {
            self.remove_tree_node(node);
        } else if node == p {
            let next = self.n(node).next;
            self.tab_mut()[index] = next;
        } else {
            let next = self.n(node).next;
            self.nm(p).next = next;
        }
        self.size -= 1;
        Some(self.release(node).value)
    }

    /// Port of the copy constructor `new HashMap<>(m)`: `putMapEntries(m, false)` presizes the
    /// threshold to `tableSizeFor(ceil(s / 0.75))`, then puts every entry in `m`'s order.
    pub fn copy_of(m: &JavaHashMap<K, V>) -> Self
    where
        K: Clone,
        V: Clone,
    {
        let mut map = JavaHashMap::new();
        let s = m.size();
        if s > 0 {
            let dt = (s as f64 / map.load_factor as f64).ceil();
            let t = if dt < MAXIMUM_CAPACITY as f64 {
                dt as i32
            } else {
                MAXIMUM_CAPACITY
            };
            if t > map.threshold {
                map.threshold = table_size_for(t);
            }
            for (k, v) in m.iter() {
                map.put(k.clone(), v.clone());
            }
        }
        map
    }
}

/// Port of `AbstractMap.equals(Object)`: same size, and every entry of this map has an equal
/// value under the same key in the other. Order and capacity are irrelevant.
impl<K: JavaHashKey + Eq, V: PartialEq> PartialEq for JavaHashMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }
        self.iter().all(|(k, v)| other.get(k) == Some(v))
    }
}

impl<K: std::fmt::Debug, V: std::fmt::Debug> std::fmt::Debug for JavaHashMap<K, V> {
    /// The entries in iteration order.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

/// Port of `HashMap.HashIterator`: buckets in index order, each along `next`.
pub struct Iter<'a, K, V> {
    map: &'a JavaHashMap<K, V>,
    /// The next bucket to look at.
    index: usize,
    next: Option<usize>,
}

impl<K, V> Iter<'_, K, V> {
    /// `do {} while (index < t.length && (next = t[index++]) == null);`
    fn advance(&mut self) {
        if let Some(t) = self.map.table.as_ref() {
            while self.index < t.len() {
                self.next = t[self.index];
                self.index += 1;
                if self.next.is_some() {
                    return;
                }
            }
            self.next = None;
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        let e = self.next?;
        let map = self.map;
        let node = map.n(e);
        self.next = node.next;
        if self.next.is_none() {
            self.advance();
        }
        Some((&node.key, &node.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_size_for_matches_the_jdk() {
        // tableSizeFor(c) is the smallest power of two >= c, with 0 and 1 both mapping to 1.
        for (cap, want) in [
            (0, 1),
            (1, 1),
            (2, 2),
            (3, 4),
            (16, 16),
            (17, 32),
            (1 << 30, 1 << 30),
        ] {
            assert_eq!(table_size_for(cap), want, "tableSizeFor({cap})");
        }
        assert_eq!(table_size_for(i32::MAX), MAXIMUM_CAPACITY);
    }

    #[test]
    fn spread_mixes_the_high_half_into_the_low() {
        assert_eq!(spread(0x1234_0000), 0x1234_1234);
        assert_eq!(spread(-1), -1 ^ 0xffff);
    }
}
