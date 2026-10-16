//! Port of: net/minecraft/world/entity/Relative.java
//! Java class(es): net.minecraft.world.entity.Relative
//! Status: PARTIAL
//!
//! Session 19 batch d. Ported: the nine constants with `bit`, the `ALL`, `ROTATION` and `DELTA`
//! sets, `union`, `rotation`, `position`, `direction`, `pack`, `unpack`, and the private
//! `getMask` / `isSet`. Constants and `bit` are checked against the jar, and the pack/unpack
//! layout is pinned bit by bit, in `_porting/tests/session19_d.rs`.
//!
//! PORT-BLOCKED: `SET_STREAM_CODEC` (`ByteBufCodecs.INT.map(Relative::unpack, Relative::pack)`)
//! on netty `ByteBuf`.
//!
//! This class has no vector math in 26.2 (applying relative flags to a position lives in
//! `PositionMoveRotation`, not here).
//!
//! # THE WIRE FORMAT
//!
//! The teleport packets send a `Set<Relative>` as one plain 4-byte big-endian `int`
//! (`ByteBufCodecs.INT`, not a VarInt) whose bit `n` is the constant with `bit == n`:
//!
//! | bit | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
//! |---|---|---|---|---|---|---|---|---|---|
//! | constant | `X` | `Y` | `Z` | `Y_ROT` | `X_ROT` | `DELTA_X` | `DELTA_Y` | `DELTA_Z` | `ROTATE_DELTA` |
//!
//! Note `Y_ROT` is bit 3 and `X_ROT` bit 4 (yaw before pitch). `unpack` ignores bits 9..31, so
//! `unpack(-1)` is `ALL` and `pack(unpack(v)) == v & 0x1FF`.
//!
//! # SETS
//!
//! Java mixes `Set.of` (immutable, unspecified order), `HashSet` (from `union`) and `EnumSet`
//! (ordinal order). All compare by content, which is the only thing callers observe; the port
//! uses `BTreeSet<Relative>` throughout, ordered by ordinal like `EnumSet` (`Relative` derives
//! `Ord` in declaration order, which is Java's `Enum#compareTo`).

use std::collections::BTreeSet;

/// Port of `net.minecraft.world.entity.Relative`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Relative {
    /// Port of `Relative#X` -- ordinal 0, bit 0.
    X = 0,
    /// Port of `Relative#Y` -- ordinal 1, bit 1.
    Y = 1,
    /// Port of `Relative#Z` -- ordinal 2, bit 2.
    Z = 2,
    /// Port of `Relative#Y_ROT` -- ordinal 3, bit 3.
    YRot = 3,
    /// Port of `Relative#X_ROT` -- ordinal 4, bit 4.
    XRot = 4,
    /// Port of `Relative#DELTA_X` -- ordinal 5, bit 5.
    DeltaX = 5,
    /// Port of `Relative#DELTA_Y` -- ordinal 6, bit 6.
    DeltaY = 6,
    /// Port of `Relative#DELTA_Z` -- ordinal 7, bit 7.
    DeltaZ = 7,
    /// Port of `Relative#ROTATE_DELTA` -- ordinal 8, bit 8.
    RotateDelta = 8,
}

/// Java: `Relative.values()` -- declaration order.
pub const VALUES: [Relative; 9] = [
    Relative::X,
    Relative::Y,
    Relative::Z,
    Relative::YRot,
    Relative::XRot,
    Relative::DeltaX,
    Relative::DeltaY,
    Relative::DeltaZ,
    Relative::RotateDelta,
];

/// Java: `public static final Set<Relative> ALL = Set.of(values())`.
pub static ALL: std::sync::LazyLock<BTreeSet<Relative>> = std::sync::LazyLock::new(|| VALUES.into_iter().collect());

/// Java: `public static final Set<Relative> ROTATION = Set.of(X_ROT, Y_ROT)`.
pub static ROTATION: std::sync::LazyLock<BTreeSet<Relative>> =
    std::sync::LazyLock::new(|| [Relative::XRot, Relative::YRot].into_iter().collect());

/// Java: `public static final Set<Relative> DELTA = Set.of(DELTA_X, DELTA_Y, DELTA_Z, ROTATE_DELTA)`.
pub static DELTA: std::sync::LazyLock<BTreeSet<Relative>> = std::sync::LazyLock::new(|| {
    [Relative::DeltaX, Relative::DeltaY, Relative::DeltaZ, Relative::RotateDelta]
        .into_iter()
        .collect()
});

impl Relative {
    /// Java: `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            Relative::X => "X",
            Relative::Y => "Y",
            Relative::Z => "Z",
            Relative::YRot => "Y_ROT",
            Relative::XRot => "X_ROT",
            Relative::DeltaX => "DELTA_X",
            Relative::DeltaY => "DELTA_Y",
            Relative::DeltaZ => "DELTA_Z",
            Relative::RotateDelta => "ROTATE_DELTA",
        }
    }

    /// Java: `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of the private `bit` field.
    pub const fn bit(&self) -> i32 {
        match self {
            Relative::X => 0,
            Relative::Y => 1,
            Relative::Z => 2,
            Relative::YRot => 3,
            Relative::XRot => 4,
            Relative::DeltaX => 5,
            Relative::DeltaY => 6,
            Relative::DeltaZ => 7,
            Relative::RotateDelta => 8,
        }
    }

    /// Port of the private `Relative#getMask()`: `1 << bit` in Java `int` arithmetic.
    pub const fn get_mask(&self) -> i32 {
        1i32.wrapping_shl(self.bit() as u32)
    }

    /// Port of the private `Relative#isSet(int)`: `(value & mask) == mask`.
    pub const fn is_set(&self, value: i32) -> bool {
        (value & self.get_mask()) == self.get_mask()
    }
}

impl std::fmt::Display for Relative {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Port of `Relative#union(Set<Relative>...)`: a new set holding every member of every argument.
pub fn union(sets: &[&BTreeSet<Relative>]) -> BTreeSet<Relative> {
    let mut set = BTreeSet::new();
    for s in sets {
        set.extend(s.iter().copied());
    }
    set
}

/// Port of `Relative#rotation(boolean relativeYRot, boolean relativeXRot)`.
///
/// Note the parameter order: yaw (`Y_ROT`) first, then pitch (`X_ROT`).
pub fn rotation(relative_y_rot: bool, relative_x_rot: bool) -> BTreeSet<Relative> {
    let mut relatives = BTreeSet::new();
    if relative_y_rot {
        relatives.insert(Relative::YRot);
    }
    if relative_x_rot {
        relatives.insert(Relative::XRot);
    }
    relatives
}

/// Port of `Relative#position(boolean, boolean, boolean)`: `X`, `Y`, `Z`.
pub fn position(relative_x: bool, relative_y: bool, relative_z: bool) -> BTreeSet<Relative> {
    let mut relatives = BTreeSet::new();
    if relative_x {
        relatives.insert(Relative::X);
    }
    if relative_y {
        relatives.insert(Relative::Y);
    }
    if relative_z {
        relatives.insert(Relative::Z);
    }
    relatives
}

/// Port of `Relative#direction(boolean, boolean, boolean)`: `DELTA_X`, `DELTA_Y`, `DELTA_Z`
/// (never `ROTATE_DELTA`, unlike the `DELTA` set).
pub fn direction(relative_x: bool, relative_y: bool, relative_z: bool) -> BTreeSet<Relative> {
    let mut relatives = BTreeSet::new();
    if relative_x {
        relatives.insert(Relative::DeltaX);
    }
    if relative_y {
        relatives.insert(Relative::DeltaY);
    }
    if relative_z {
        relatives.insert(Relative::DeltaZ);
    }
    relatives
}

/// Port of `Relative#unpack(int)`: every constant whose bit is set; higher bits are ignored.
pub fn unpack(value: i32) -> BTreeSet<Relative> {
    let mut result = BTreeSet::new();
    for argument in VALUES {
        if argument.is_set(value) {
            result.insert(argument);
        }
    }
    result
}

/// Port of `Relative#pack(Set<Relative>)`: the OR of every member's mask.
pub fn pack(set: &BTreeSet<Relative>) -> i32 {
    let mut result = 0;
    for argument in set {
        result |= argument.get_mask();
    }
    result
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final StreamCodec<ByteBuf, Set<Relative>> SET_STREAM_CODEC =
/// ByteBufCodecs.INT.map(Relative::unpack, Relative::pack)`.
///
/// PORT-BLOCKED on `io.netty.buffer.ByteBuf`. Decoding is [`unpack`], encoding is [`pack`].
pub fn SET_STREAM_CODEC() {
    todo!("PORT-BLOCKED: io.netty.buffer.ByteBuf is not ported")
}
