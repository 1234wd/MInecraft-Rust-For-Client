//! Port of: net/minecraft/client/renderer/Rect2i.java
//! Java class(es): net.minecraft.client.renderer.Rect2i
//! Status: PORTED
//!
//! Session 19 batch h. Every member: constructor, `intersect`, the getters/setters,
//! `setPosition`, `contains`. Nothing is PORT-BLOCKED; the class has no constants, so there is no
//! golden data (hence `PORTED`).
//!
//! * `int` arithmetic wraps (`xPos + width` etc.), so the port uses `wrapping_*`.
//! * `intersect` MUTATES `this` and returns it; the port returns `&mut Self`.
//! * `contains` is inclusive on BOTH edges (`x <= xPos + width`), so a 10-wide rect at 0
//!   "contains" x = 10.
//! * No `equals`/`hashCode`/`toString` override: Java compares by identity, so the port does not
//!   derive `PartialEq`.

/// Port of `net.minecraft.client.renderer.Rect2i`.
#[derive(Clone, Debug)]
pub struct Rect2i {
    x_pos: i32,
    y_pos: i32,
    width: i32,
    height: i32,
}

impl Rect2i {
    /// Port of `Rect2i(int x, int y, int width, int height)`.
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x_pos: x,
            y_pos: y,
            width,
            height,
        }
    }

    /// Port of `Rect2i#intersect(Rect2i)`: shrinks `self` to the overlap (width/height clamped
    /// at 0) and returns `self`.
    pub fn intersect(&mut self, other: &Rect2i) -> &mut Self {
        let x0 = self.x_pos;
        let y0 = self.y_pos;
        let x1 = self.x_pos.wrapping_add(self.width);
        let y1 = self.y_pos.wrapping_add(self.height);
        let x2 = other.get_x();
        let y2 = other.get_y();
        let x3 = x2.wrapping_add(other.get_width());
        let y3 = y2.wrapping_add(other.get_height());
        self.x_pos = std::cmp::max(x0, x2);
        self.y_pos = std::cmp::max(y0, y2);
        self.width = std::cmp::max(0, std::cmp::min(x1, x3).wrapping_sub(self.x_pos));
        self.height = std::cmp::max(0, std::cmp::min(y1, y3).wrapping_sub(self.y_pos));
        self
    }

    /// Port of `Rect2i#getX()`.
    pub fn get_x(&self) -> i32 {
        self.x_pos
    }

    /// Port of `Rect2i#getY()`.
    pub fn get_y(&self) -> i32 {
        self.y_pos
    }

    /// Port of `Rect2i#setX(int)`.
    pub fn set_x(&mut self, x: i32) {
        self.x_pos = x;
    }

    /// Port of `Rect2i#setY(int)`.
    pub fn set_y(&mut self, y: i32) {
        self.y_pos = y;
    }

    /// Port of `Rect2i#getWidth()`.
    pub fn get_width(&self) -> i32 {
        self.width
    }

    /// Port of `Rect2i#getHeight()`.
    pub fn get_height(&self) -> i32 {
        self.height
    }

    /// Port of `Rect2i#setWidth(int)`.
    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    /// Port of `Rect2i#setHeight(int)`.
    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    /// Port of `Rect2i#setPosition(int, int)`.
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x_pos = x;
        self.y_pos = y;
    }

    /// Port of `Rect2i#contains(int, int)` -- inclusive on all four edges.
    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x_pos
            && x <= self.x_pos.wrapping_add(self.width)
            && y >= self.y_pos
            && y <= self.y_pos.wrapping_add(self.height)
    }
}
