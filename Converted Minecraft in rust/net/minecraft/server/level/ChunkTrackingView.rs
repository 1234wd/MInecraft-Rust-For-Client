//! Port of: net/minecraft/server/level/ChunkTrackingView.java
//! Java class(es): net.minecraft.server.level.ChunkTrackingView, net.minecraft.server.level.Positioned, ChunkTrackingView::<1 anonymous class(es)>
//! Status: VERIFIED
//!
//! Session 19. Every member is ported: `EMPTY`, `of`, `difference`, the default and abstract
//! `contains` overloads, `forEach`, both `isInViewDistance`, `isWithinDistance`, and the
//! `Positioned` record (`minX/minZ/maxX/maxZ`, `squareIntersects`). Verified against the real
//! 26.2 jar by `_porting/tests/session19_consts.rs` replaying the `view.*` rows of
//! `_porting/test-data/graph_probe.txt` (membership, `forEach` order, and the enter/leave
//! sequences `difference` emits).
//!
//! # A CLOSED SET OF TWO IMPLEMENTATIONS
//!
//! Vanilla has exactly two implementations -- the anonymous `EMPTY` and the `Positioned` record --
//! so the port is an enum. `difference` compares them with Java `equals`: `EMPTY` is equal only to
//! itself, `Positioned` by `(center, viewDistance)`; the derived `PartialEq` is the same relation.
//!
//! # THE DISTANCE TEST IS CIRCULAR, WITH A BUFFER
//!
//! `isWithinDistance` subtracts a buffer of 1 (2 with neighbours) from each axis distance BEFORE
//! squaring, in `long` arithmetic, and compares with a STRICT `<` against `viewDistance^2`. So the
//! tracked area is a rounded square, not a circle and not a square.

use crate::net::minecraft::world::level::ChunkPos::ChunkPos;

/// Port of `ChunkTrackingView.Positioned(ChunkPos center, int viewDistance)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Positioned {
    /// Record component `center`.
    pub center: ChunkPos,
    /// Record component `viewDistance`.
    pub view_distance: i32,
}

impl Positioned {
    fn min_x(&self) -> i32 {
        self.center.x.wrapping_sub(self.view_distance).wrapping_sub(1)
    }

    fn min_z(&self) -> i32 {
        self.center.z.wrapping_sub(self.view_distance).wrapping_sub(1)
    }

    fn max_x(&self) -> i32 {
        self.center.x.wrapping_add(self.view_distance).wrapping_add(1)
    }

    fn max_z(&self) -> i32 {
        self.center.z.wrapping_add(self.view_distance).wrapping_add(1)
    }

    /// Port of `Positioned#squareIntersects(Positioned)` (`@VisibleForTesting`).
    pub fn square_intersects(&self, other: &Positioned) -> bool {
        self.min_x() <= other.max_x() && self.max_x() >= other.min_x() && self.min_z() <= other.max_z() && self.max_z() >= other.min_z()
    }
}

/// Port of the `net.minecraft.server.level.ChunkTrackingView` interface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChunkTrackingView {
    /// Java: `ChunkTrackingView.EMPTY`.
    Empty,
    /// Java: `ChunkTrackingView.Positioned`.
    Positioned(Positioned),
}

/// Java: `ChunkTrackingView EMPTY`.
pub const EMPTY: ChunkTrackingView = ChunkTrackingView::Empty;

/// Port of `ChunkTrackingView#of(ChunkPos, int)`.
pub fn of(center: ChunkPos, radius: i32) -> ChunkTrackingView {
    ChunkTrackingView::Positioned(Positioned { center, view_distance: radius })
}

/// Port of `ChunkTrackingView#difference(from, to, onEnter, onLeave)`.
///
/// For two overlapping `Positioned` views it walks their joint bounding square x-major and
/// reports each changed chunk; otherwise it reports every chunk of `from` as leaving, then every
/// chunk of `to` as entering.
pub fn difference(from: &ChunkTrackingView, to: &ChunkTrackingView, mut on_enter: impl FnMut(ChunkPos), mut on_leave: impl FnMut(ChunkPos)) {
    if from == to {
        return;
    }
    if let (ChunkTrackingView::Positioned(last), ChunkTrackingView::Positioned(next)) = (from, to) {
        if last.square_intersects(next) {
            let min_x = last.min_x().min(next.min_x());
            let min_z = last.min_z().min(next.min_z());
            let max_x = last.max_x().max(next.max_x());
            let max_z = last.max_z().max(next.max_z());
            let mut x = min_x;
            while x <= max_x {
                let mut z = min_z;
                while z <= max_z {
                    let saw = from.contains(x, z);
                    let sees = to.contains(x, z);
                    if saw != sees {
                        if sees {
                            on_enter(ChunkPos::new(x, z));
                        } else {
                            on_leave(ChunkPos::new(x, z));
                        }
                    }
                    if z == i32::MAX {
                        break;
                    }
                    z += 1;
                }
                if x == i32::MAX {
                    break;
                }
                x += 1;
            }
            return;
        }
    }
    from.for_each(&mut on_leave);
    to.for_each(&mut on_enter);
}

/// Port of the static `ChunkTrackingView#isInViewDistance(int, int, int, int, int)`.
pub fn is_in_view_distance_static(center_x: i32, center_z: i32, view_distance: i32, chunk_x: i32, chunk_z: i32) -> bool {
    is_within_distance(center_x, center_z, view_distance, chunk_x, chunk_z, false)
}

/// Port of `ChunkTrackingView#isWithinDistance(int, int, int, int, int, boolean)`.
pub fn is_within_distance(center_x: i32, center_z: i32, view_distance: i32, chunk_x: i32, chunk_z: i32, include_neighbors: bool) -> bool {
    let buffer_range = if include_neighbors { 2 } else { 1 };
    // `Math.abs(int)` on the int difference (wrapping, and abs(MIN) == MIN), then max with 0,
    // then widened to long.
    let delta_x = 0i32.max(chunk_x.wrapping_sub(center_x).wrapping_abs().wrapping_sub(buffer_range)) as i64;
    let delta_z = 0i32.max(chunk_z.wrapping_sub(center_z).wrapping_abs().wrapping_sub(buffer_range)) as i64;
    let distance_squared = delta_x.wrapping_mul(delta_x).wrapping_add(delta_z.wrapping_mul(delta_z));
    let radius_squared = view_distance.wrapping_mul(view_distance);
    distance_squared < radius_squared as i64
}

impl ChunkTrackingView {
    /// Port of the abstract `contains(int chunkX, int chunkZ, boolean includeNeighbors)`.
    pub fn contains_with(&self, chunk_x: i32, chunk_z: i32, include_neighbors: bool) -> bool {
        match self {
            ChunkTrackingView::Empty => false,
            ChunkTrackingView::Positioned(p) => {
                is_within_distance(p.center.x, p.center.z, p.view_distance, chunk_x, chunk_z, include_neighbors)
            }
        }
    }

    /// Port of the default `contains(ChunkPos)`.
    pub fn contains_pos(&self, pos: &ChunkPos) -> bool {
        self.contains(pos.x, pos.z)
    }

    /// Port of the default `contains(int, int)`: neighbours INCLUDED.
    pub fn contains(&self, x: i32, z: i32) -> bool {
        self.contains_with(x, z, true)
    }

    /// Port of the default `isInViewDistance(int, int)`: neighbours EXCLUDED.
    pub fn is_in_view_distance(&self, chunk_x: i32, chunk_z: i32) -> bool {
        self.contains_with(chunk_x, chunk_z, false)
    }

    /// Port of `forEach(Consumer<ChunkPos>)`: x-major over the bounding square, keeping chunks
    /// that `contains(x, z)` (neighbours included).
    pub fn for_each(&self, consumer: &mut impl FnMut(ChunkPos)) {
        if let ChunkTrackingView::Positioned(p) = self {
            let mut x = p.min_x();
            while x <= p.max_x() {
                let mut z = p.min_z();
                while z <= p.max_z() {
                    if self.contains(x, z) {
                        consumer(ChunkPos::new(x, z));
                    }
                    if z == i32::MAX {
                        break;
                    }
                    z += 1;
                }
                if x == i32::MAX {
                    break;
                }
                x += 1;
            }
        }
    }
}
