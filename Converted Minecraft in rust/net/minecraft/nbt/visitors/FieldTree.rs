//! Port of: net/minecraft/nbt/visitors/FieldTree.java
//! Java class(es): net.minecraft.nbt.visitors.FieldTree
//! Status: VERIFIED
//!
//! Session 23. `public record FieldTree(int depth, Map<String, TagType<?>> selectedFields,
//! Map<String, FieldTree> fieldsToRecurse)`: the path tree `CollectFields`/`SkipFields` build from
//! their `FieldSelector`s. A selector with path `[a, b]` hangs its `(name, type)` off the
//! depth-3 node reached through `a` then `b`; one with an empty path selects at the root (depth 1).
//!
//! * `addEntry(field)`: while `depth <= path.size()`, descend into
//!   `fieldsToRecurse.computeIfAbsent(path[depth - 1], new FieldTree(depth + 1))`; at the end of the
//!   path, `selectedFields.put(name, type)` -- so two selectors for one name at one place keep the
//!   LAST type.
//! * `isSelected(type, id)`: `type.equals(selectedFields.get(id))` -- the type must match too.
//!
//! # Port shape
//!
//! Both maps are [`JavaHashMap`]s, as in Java, but nothing reads their ORDER: `CollectFields` and
//! `SkipFields` only `get`/`remove` by key, and the record's `toString`/`hashCode` -- the only
//! members that would iterate -- fold in `TagType`'s identity `hashCode`/`toString` and are not
//! ported (OPEN_QUESTIONS #30). That is also why `computeIfAbsent` is spelled `contains`+`put`
//! here: the JDK's `computeIfAbsent` links a new node at the HEAD of its bucket where `put`
//! appends, a difference only iteration order could show. `equals` (derived, map content) is
//! portable and ported.
//!
//! The private `FieldTree(int depth)` constructor is [`FieldTree::with_depth`], private too.
//!
//! Verified by the `FTREE` group of `_porting/test-data/nbt_probe.txt` (the tree each of the
//! probe's selector sets builds, and `isSelected` over it) and, through `CollectFields` /
//! `SkipFields`, by the `CF<n>`/`SF<n>` rows of every `GEN`/`CASE` tree (`tests/session23.rs`).

use crate::javacompat::java_hash_map::JavaHashMap;
use crate::net::minecraft::nbt::visitors::FieldSelector::FieldSelector;
use crate::net::minecraft::nbt::TagType::TagTypeRef;

/// Port of `net.minecraft.nbt.visitors.FieldTree`.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldTree {
    depth: i32,
    selected_fields: JavaHashMap<String, TagTypeRef>,
    fields_to_recurse: JavaHashMap<String, FieldTree>,
}

impl FieldTree {
    /// Port of the canonical constructor.
    pub fn new(
        depth: i32,
        selected_fields: JavaHashMap<String, TagTypeRef>,
        fields_to_recurse: JavaHashMap<String, FieldTree>,
    ) -> Self {
        FieldTree {
            depth,
            selected_fields,
            fields_to_recurse,
        }
    }

    /// Port of the private `FieldTree(int depth)`: two empty `HashMap`s.
    fn with_depth(depth: i32) -> Self {
        FieldTree::new(depth, JavaHashMap::new(), JavaHashMap::new())
    }

    /// Port of `FieldTree.createRoot()`: depth 1.
    pub fn create_root() -> Self {
        FieldTree::with_depth(1)
    }

    /// Port of `addEntry(FieldSelector)`.
    pub fn add_entry(&mut self, field: &FieldSelector) {
        if self.depth as usize <= field.path().len() {
            let step = &field.path()[(self.depth - 1) as usize];
            if !self.fields_to_recurse.contains_key(step.as_str()) {
                self.fields_to_recurse
                    .put(step.clone(), FieldTree::with_depth(self.depth + 1));
            }
            self.fields_to_recurse
                .get_mut(step.as_str())
                .expect("just ensured")
                .add_entry(field);
        } else {
            self.selected_fields
                .put(field.name().to_string(), field.tag_type());
        }
    }

    /// Port of `isSelected(TagType, String)`: the id is selected WITH this type.
    pub fn is_selected(&self, tag_type: &TagTypeRef, id: &str) -> bool {
        self.selected_fields.get(id) == Some(tag_type)
    }

    /// Port of the accessor `depth()`.
    pub fn depth(&self) -> i32 {
        self.depth
    }

    /// Port of the accessor `selectedFields()`.
    pub fn selected_fields(&self) -> &JavaHashMap<String, TagTypeRef> {
        &self.selected_fields
    }

    /// `selectedFields()`, for `CollectFields`' `selectedFields().remove(id, type)`: Java hands
    /// out the record's live map.
    pub fn selected_fields_mut(&mut self) -> &mut JavaHashMap<String, TagTypeRef> {
        &mut self.selected_fields
    }

    /// Port of the accessor `fieldsToRecurse()`.
    pub fn fields_to_recurse(&self) -> &JavaHashMap<String, FieldTree> {
        &self.fields_to_recurse
    }

    /// `fieldsToRecurse()`, mutably -- see [`FieldTree::selected_fields_mut`].
    pub fn fields_to_recurse_mut(&mut self) -> &mut JavaHashMap<String, FieldTree> {
        &mut self.fields_to_recurse
    }

    /// The node reached from this one through `path` (`fieldsToRecurse().get(..)` per step).
    ///
    /// Not a Java method: `CollectFields`/`SkipFields` keep a `Deque` of references into the tree,
    /// and the port keeps the paths to those nodes instead (see their module docs).
    pub fn descend(&self, path: &[String]) -> &FieldTree {
        let mut node = self;
        for step in path {
            node = node
                .fields_to_recurse
                .get(step.as_str())
                .expect("a path recorded from this tree");
        }
        node
    }

    /// [`FieldTree::descend`], mutably.
    pub fn descend_mut(&mut self, path: &[String]) -> &mut FieldTree {
        let mut node = self;
        for step in path {
            node = node
                .fields_to_recurse
                .get_mut(step.as_str())
                .expect("a path recorded from this tree");
        }
        node
    }
}
