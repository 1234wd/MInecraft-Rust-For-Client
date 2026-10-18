//! Port of: net/minecraft/client/gui/components/SelectableEntry.java
//! Java class(es): net.minecraft.client.gui.components.SelectableEntry
//! Status: VERIFIED
//!
//! Session 22. `public interface SelectableEntry`: seven default hit-tests for a square icon of side
//! `size` at relative mouse coordinates -- the whole icon, its left/right halves and its four
//! quarters.
//!
//! The half point is `size / 2` in `int` arithmetic, truncating toward zero, so for an odd size the
//! LEFT half is the smaller one, and every range is half-open (`>= start && < end`). A negative or
//! zero size matches nothing. Rust's `i32` `/` truncates the same way.
//!
//! Verified by the `SELENTRY` group of `small_probe.txt` (`tests/session22.rs`): six sizes (odd,
//! zero, one, negative, `MAX_VALUE`) x a 10 x 10 grid of coordinates, all seven tests.

/// Port of `net.minecraft.client.gui.components.SelectableEntry`.
pub trait SelectableEntry {
    /// Port of `default boolean mouseOverIcon(int relX, int relY, int size)`.
    fn mouse_over_icon(&self, rel_x: i32, rel_y: i32, size: i32) -> bool {
        rel_x >= 0 && rel_x < size && rel_y >= 0 && rel_y < size
    }

    /// Port of `default boolean mouseOverLeftHalf(int relX, int relY, int size)`.
    fn mouse_over_left_half(&self, rel_x: i32, rel_y: i32, size: i32) -> bool {
        rel_x >= 0 && rel_x < size / 2 && rel_y >= 0 && rel_y < size
    }

    /// Port of `default boolean mouseOverRightHalf(int relX, int relY, int size)`.
    fn mouse_over_right_half(&self, rel_x: i32, rel_y: i32, size: i32) -> bool {
        rel_x >= size / 2 && rel_x < size && rel_y >= 0 && rel_y < size
    }

    /// Port of `default boolean mouseOverTopRightQuarter(int relX, int relY, int size)`.
    fn mouse_over_top_right_quarter(&self, rel_x: i32, rel_y: i32, size: i32) -> bool {
        rel_x >= size / 2 && rel_x < size && rel_y >= 0 && rel_y < size / 2
    }

    /// Port of `default boolean mouseOverBottomRightQuarter(int relX, int relY, int size)`.
    fn mouse_over_bottom_right_quarter(&self, rel_x: i32, rel_y: i32, size: i32) -> bool {
        rel_x >= size / 2 && rel_x < size && rel_y >= size / 2 && rel_y < size
    }

    /// Port of `default boolean mouseOverTopLeftQuarter(int relX, int relY, int size)`.
    fn mouse_over_top_left_quarter(&self, rel_x: i32, rel_y: i32, size: i32) -> bool {
        rel_x >= 0 && rel_x < size / 2 && rel_y >= 0 && rel_y < size / 2
    }

    /// Port of `default boolean mouseOverBottomLeftQuarter(int relX, int relY, int size)`.
    fn mouse_over_bottom_left_quarter(&self, rel_x: i32, rel_y: i32, size: i32) -> bool {
        rel_x >= 0 && rel_x < size / 2 && rel_y >= size / 2 && rel_y < size
    }
}
