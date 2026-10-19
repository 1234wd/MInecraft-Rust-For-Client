//! Port of: net/minecraft/util/Graph.java
//! Java class(es): net.minecraft.util.Graph
//! Status: VERIFIED
//!
//! Session 24. `public final class Graph`: one static helper, a recursive depth-first search that
//! emits nodes in reverse topological order and reports a cycle. `FeatureSorter` uses it to order
//! placed features.
//!
//! ```java
//! if (discovered.contains(current)) return false;
//! if (currentlyVisiting.contains(current)) return true;          // a cycle
//! currentlyVisiting.add(current);
//! for (T next : edges.getOrDefault(current, ImmutableSet.of()))
//!     if (depthFirstSearch(edges, discovered, currentlyVisiting, reverseTopologicalOrder, next)) return true;
//! currentlyVisiting.remove(current);
//! discovered.add(current);
//! reverseTopologicalOrder.accept(current);
//! return false;
//! ```
//!
//! # What the port takes
//!
//! * `Map<T, Set<T>> edges` is a closure from a node to its successors IN THE ITERATION ORDER OF
//!   ITS SET -- the order decides both the emitted order and which cycle is found first, and it
//!   belongs to whatever map the caller built. A node with no entry yields nothing
//!   (`getOrDefault(current, ImmutableSet.of())`);
//! * `discovered` and `currentlyVisiting` are only asked `contains`/`add`/`remove`, so their own
//!   order never shows: `HashSet`s;
//! * `Consumer<T>` is `FnMut(&T)`.
//!
//! On a cycle the search returns `true` at once, leaving `currentlyVisiting` holding the path that
//! led there (nothing is removed on the way out), exactly as Java does.
//!
//! The class is `final` with a private constructor: only the function is ported.
//!
//! Verified by the `GRAPH` group of `sweep_probe.txt` (`tests/session24.rs`).

use std::collections::HashSet;
use std::hash::Hash;

/// Port of `public static <T> boolean depthFirstSearch(Map<T, Set<T>> edges, Set<T> discovered,
/// Set<T> currentlyVisiting, Consumer<T> reverseTopologicalOrder, T current)`.
pub fn depth_first_search<T, F, I>(
    edges: &F,
    discovered: &mut HashSet<T>,
    currently_visiting: &mut HashSet<T>,
    reverse_topological_order: &mut dyn FnMut(&T),
    current: &T,
) -> bool
where
    T: Eq + Hash + Clone,
    F: Fn(&T) -> I,
    I: IntoIterator<Item = T>,
{
    if discovered.contains(current) {
        return false;
    }
    if currently_visiting.contains(current) {
        return true;
    }
    currently_visiting.insert(current.clone());
    for next in edges(current) {
        if depth_first_search(
            edges,
            discovered,
            currently_visiting,
            reverse_topological_order,
            &next,
        ) {
            return true;
        }
    }
    currently_visiting.remove(current);
    discovered.insert(current.clone());
    reverse_topological_order(current);
    false
}
