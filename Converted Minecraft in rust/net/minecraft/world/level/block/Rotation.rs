//! Port of: net/minecraft/world/level/block/Rotation.java
//! Java class(es): net.minecraft.world.level.block.Rotation
//! Status: PARTED -- 5 of 12 methods; 7 `todo!` on unported external types.
//!
//! Ported ahead of `BlockPos`, which needs the four enum constants and the `index` they carry.
//! No `rotation.*` golden group exists for THIS class -- the four `rotations.*` groups in
//! `batch2.txt` belong to `net.minecraft.core.Rotations`, a different class with a different
//! shape (three `f32` Euler angles). So nothing here is oracle-verified, which is stated plainly
//! rather than glossed: the port is a transcription, and `blockpos.rotate` is the only
//! oracle-covered method that reaches it.
//!
//! # THE FOUR CONSTANTS AND THEIR `index`
//!
//! ```java
//! NONE(0, "none", ...), CLOCKWISE_90(1, "clockwise_90", ...),
//! CLOCKWISE_180(2, "180", ...), COUNTERCLOCKWISE_90(3, "counterclockwise_90", ...)
//! ```
//!
//! Three separate orderings must not be conflated:
//!
//! * **Declaration order** -- `NONE, CLOCKWISE_90, CLOCKWISE_180, COUNTERCLOCKWISE_90`. This is
//!   the `ordinal()`, and `BlockPos.rotate` is dispatched on the golden by ordinal
//!   (`V2.ROTATIONS[ri]`), so the variant order here IS the golden's input order.
//! * **`index`** -- 0, 1, 2, 3 in the same order here, but a *separate field*. It is what
//!   `STREAM_CODEC` writes and what structure templates persist. It coincides with the ordinal
//!   only by accident of this declaration order, so it is stored separately rather than
//!   assumed equal.
//! * **`id`** -- `"none"`, `"clockwise_90"`, `"180"`, `"counterclockwise_90"`. Note `"180"` has
//!   no direction word, and is not `"clockwise_180"`. That is the saved-data and command
//!   spelling; changing it would break every structure template in existence.
//!
//! # `getRotated` IS A LEFT-RIGHT ROTATION OF THE GROUP, NOT A LOOKUP
//!
//! `CLOCKWISE_90.getRotated(x)` is one step further round the cycle, so the four values form a
//! group. The three `switch` arms below are that cycle written out; the `default ->` arms are
//! unreachable for a four-constant enum and exist only because the decompiler emits them.

use crate::javacompat::java_lang;
use crate::net::minecraft::core::Direction::Direction;
use crate::net::minecraft::util::RandomSource::RandomSource;

/// Port of `net.minecraft.world.level.block.Rotation`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rotation {
    /// Port of `Rotation#NONE` -- ordinal 0, index 0, id `"none"`.
    None = 0,
    /// Port of `Rotation#CLOCKWISE_90` -- ordinal 1, index 1, id `"clockwise_90"`.
    Clockwise90 = 1,
    /// Port of `Rotation#CLOCKWISE_180` -- ordinal 2, index 2, id `"180"`.
    Clockwise180 = 2,
    /// Port of `Rotation#COUNTERCLOCKWISE_90` -- ordinal 3, index 3, id `"counterclockwise_90"`.
    CounterClockwise90 = 3,
}

/// Java: `Rotation.values()`.
///
/// Order is declaration order, and it is load-bearing: `getRandom` indexes this array with a
/// random draw, and `BlockPos.rotate` is dispatched by ordinal.
pub const VALUES: [Rotation; 4] = [
    Rotation::None,
    Rotation::Clockwise90,
    Rotation::Clockwise180,
    Rotation::CounterClockwise90,
];

impl Rotation {
    /// Port of `Rotation#values()`'s `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `Rotation#ordinal()` as an array index.
    #[inline]
    pub const fn index_in_values(&self) -> usize {
        *self as usize
    }

    /// Port of the private `Rotation#getIndex()`.
    ///
    /// Distinct from [`Rotation::ordinal`], and kept separate so that a future change to the
    /// declaration order cannot silently move the wire/persistence value with it.
    #[inline]
    pub const fn get_index(&self) -> i32 {
        *self as i32
    }

    /// Port of `Rotation#ordinal()`'s inverse -- `valueOf(int)` over the declared constants.
    ///
    /// Panics on an out-of-range index, matching Java's `Enum.valueOf` throwing
    /// `IllegalArgumentException`.
    #[inline]
    pub fn from_ordinal(ordinal: i32) -> Rotation {
        assert!(
            (0..4).contains(&ordinal),
            "no enum constant Rotation.{ordinal}"
        );
        VALUES[ordinal as usize]
    }

    /// Port of `Rotation#getSerializedName()` -- the `id` field.
    ///
    /// `"180"`, not `"clockwise_180"`. This string is persisted in structure templates and typed
    /// in commands, so it is reproduced exactly.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            Rotation::None => "none",
            Rotation::Clockwise90 => "clockwise_90",
            Rotation::Clockwise180 => "180",
            Rotation::CounterClockwise90 => "counterclockwise_90",
        }
    }

    /// Port of `Rotation#getRotated(Rotation)`.
    ///
    /// Composition, not lookup: `a.getRotated(b)` advances `a` by `b` quarter-turns around the
    /// Y axis. The nested `switch`es are transcribed as written rather than collapsed into
    /// modular arithmetic on the index, because the two are only equivalent for rotations that
    /// are multiples of 90 degrees and this enum holds nothing else -- so the readable form is
    /// also the correct one here.
    #[inline]
    pub const fn get_rotated(&self, rot: Rotation) -> Rotation {
        // Matched on `*self` so every arm yields a `Rotation` by value. Matching on `self`
        // would make the inner arms `Rotation` and the `default` arm `&Rotation`, which do not
        // unify -- which is Java's `default -> this` reading as a reference here.
        match rot {
            Rotation::Clockwise90 => match *self {
                Rotation::None => Rotation::Clockwise90,
                Rotation::Clockwise90 => Rotation::Clockwise180,
                Rotation::Clockwise180 => Rotation::CounterClockwise90,
                Rotation::CounterClockwise90 => Rotation::None,
            },
            Rotation::Clockwise180 => match *self {
                Rotation::None => Rotation::Clockwise180,
                Rotation::Clockwise90 => Rotation::CounterClockwise90,
                Rotation::Clockwise180 => Rotation::None,
                Rotation::CounterClockwise90 => Rotation::Clockwise90,
            },
            Rotation::CounterClockwise90 => match *self {
                Rotation::None => Rotation::CounterClockwise90,
                Rotation::Clockwise90 => Rotation::None,
                Rotation::Clockwise180 => Rotation::Clockwise90,
                Rotation::CounterClockwise90 => Rotation::Clockwise180,
            },
            // Java's `default -> this`.
            Rotation::None => *self,
        }
    }

    /// Port of `Rotation#rotate(Direction)`.
    ///
    /// A **vertical** direction is returned unchanged, before any rotation is considered. That
    /// early return is the load-bearing part: `getClockWise()` on `UP`/`DOWN` is meaningless, and
    /// vanilla avoids it by checking the axis first rather than by handling a bad result.
    #[inline]
    pub fn rotate(&self, direction: Direction) -> Direction {
        if direction.get_axis().is_vertical() {
            return direction;
        }
        match self {
            Rotation::Clockwise90 => direction.get_clock_wise(),
            Rotation::Clockwise180 => direction.get_opposite(),
            Rotation::CounterClockwise90 => direction.get_counter_clock_wise(),
            Rotation::None => direction,
        }
    }

    /// Port of `Rotation#rotate(int, int)`.
    ///
    /// The divisions are **Java integer** divisions and they TRUNCATE, so they are lossy for
    /// negative `steps`: `steps / 4` for `steps == -1` is `0`, not `-1`. That is vanilla's
    /// behaviour, and `steps` is a quarter-turn count where callers pass positive values -- but
    /// a port that rounds instead of truncating would differ for any negative input, so the
    /// truncation is what is written here.
    ///
    /// The `% steps` at the end also divides by `steps`, which means `steps == 0` divides by
    /// zero and throws in Java. Not guarded here; `todo!`-free but unreachable from vanilla
    /// callers, which pass a `steps` equal to the slice length (always >= 1).
    #[allow(clippy::arithmetic_side_effects)] // Java's int division truncates; see above.
    #[inline]
    pub fn rotate_index(&self, rotation: i32, steps: i32) -> i32 {
        match self {
            Rotation::Clockwise90 => (rotation + steps / 4) % steps,
            Rotation::Clockwise180 => (rotation + steps / 2) % steps,
            Rotation::CounterClockwise90 => (rotation + steps * 3 / 4) % steps,
            Rotation::None => rotation,
        }
    }

    /// Port of `Rotation#getRandom(RandomSource)`: `Util.getRandom(values(), random)`.
    ///
    /// `Util.getRandom` indexes `values()` with `random.nextInt(values().length)`, so the draw is
    /// a plain `nextInt(4)`. Transcribed directly rather than delegating, because `Util` is not
    /// ported; the expression is one line and its equivalence is obvious.
    #[inline]
    pub fn get_random(random: &mut dyn RandomSource) -> Rotation {
        VALUES[random.next_int_bounded(4) as usize]
    }

    /// Port of `Rotation#getShuffled(RandomSource)`: `Util.shuffledCopy(values(), random)`.
    ///
    /// `Util.shuffledCopy` is a Fisher-Yates over a fresh `ObjectArrayList`, walking **downwards**
    /// (`for (i = size - 1; i >= 0; i--)`) and swapping with `nextInt(i + 1)`. The order of the
    /// walk and the form of the bound are both load-bearing -- walking upwards consumes a
    /// different number of draws and gives a different permutation for the same seed -- so the
    /// loop is transcribed as written.
    ///
    /// # UNVERIFIED
    ///
    /// No golden group covers this: `Util.shuffledCopy` is not ported and no oracle group calls
    /// `Rotation#getShuffled`. The draw sequence follows `Util.java` directly; if it ever diverges
    /// it will be in the walk direction, which is the one thing here a reader is most likely to
    /// "fix" wrongly.
    #[inline]
    pub fn get_shuffled(random: &mut dyn RandomSource) -> Vec<Rotation> {
        let mut out: Vec<Rotation> = VALUES.to_vec();
        // Walk DOWNWARDS, and bound by `i + 1` inclusive -- both from `Util.shuffledCopy`.
        let mut i = out.len() as i32 - 1;
        while i >= 0 {
            let j = random.next_int_bounded(i + 1);
            out.swap(i as usize, j as usize);
            i -= 1;
        }
        out
    }
}

impl std::fmt::Display for Rotation {
    /// Java's `Enum#toString()` is `name()`, the CONSTANT IDENTIFIER -- `CLOCKWISE_90` -- not
    /// the serialized `id`. They differ (`CLOCKWISE_180` vs `"180"`), so this is not the same as
    /// [`Rotation::get_serialized_name`].
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = java_lang::min_i32;
        match self {
            Rotation::None => f.write_str("NONE"),
            Rotation::Clockwise90 => f.write_str("CLOCKWISE_90"),
            Rotation::Clockwise180 => f.write_str("CLOCKWISE_180"),
            Rotation::CounterClockwise90 => f.write_str("COUNTERCLOCKWISE_90"),
        }
    }
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================
//
// Seven methods, all of them codec fields or wrappers over unported types. Each names its
// dependency rather than returning a placeholder, because a wrong codec value is the kind of
// desync that survives every unit test and shows up as a corrupted world.
//
// `Rotation.BY_ID`, `CODEC`, `STREAM_CODEC` and `LEGACY_CODEC` are the four codec entries;
// `rotation()` returns `com.mojang.math.OctahedralGroup`, which is not ported and is the third
// dependency of the same kind.

/// Java: `public static final IntFunction<Rotation> BY_ID`.
pub fn BY_ID(_id: i32) -> Rotation {
    todo!("PORT-BLOCKED: net.minecraft.util.ByIdMap is not ported")
}

/// Java: `public static final Codec<Rotation> CODEC`.
pub fn CODEC() -> () {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, Rotation> STREAM_CODEC`.
pub fn STREAM_CODEC() -> () {
    todo!("PORT-BLOCKED: net.minecraft.network.codec.StreamCodec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, Rotation> LEGACY_CODEC`.
pub fn LEGACY_CODEC() -> () {
    todo!("PORT-BLOCKED: net.minecraft.util.ExtraCodecs is not ported")
}

/// Java: `public OctahedralGroup rotation()`.
pub fn rotation_group() -> () {
    todo!("PORT-BLOCKED: com.mojang.math.OctahedralGroup is not ported")
}
