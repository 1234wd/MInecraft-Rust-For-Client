//! Port of: net/minecraft/core/Direction.java
//! Java class(es): Direction, Direction.Axis, Direction.AxisDirection, Direction.Plane
//! Status: PARTIAL
//!
//! Everything except the DataFixerUpper / netty codec fields, which are `todo!()` per
//! the batch rule. Those are:
//!
//! * `Direction.CODEC`, `Direction.VERTICAL_CODEC`
//! * `Direction.STREAM_CODEC`, `Direction.LEGACY_ID_CODEC`, `Direction.LEGACY_ID_CODEC_2D`
//!
//! They are on no parity-tested path. `Direction.BY_ID` IS ported, because it is the
//! lookup the rules ask to verify and it depends only on `get3DDataValue`.
//! See DESIGN_DECISIONS.md (#dfu-proposal).
//!
//! `Direction.Plane` is NOT ported -- see the `todo!` below.
//!
//! Everything that IS implemented here is oracle-verified against `core.txt`
//! (48 groups, 320 604 rows) -- see `_porting/tests/parity_core.rs`.

use crate::javacompat::java_lang;
use crate::net::minecraft::core::Vec3i::Vec3i;

/// Port of `net.minecraft.core.Direction`.
///
/// # ORDINAL ORDER IS PERSISTED DATA
///
/// The declaration order below is
/// `DOWN, UP, NORTH, SOUTH, WEST, EAST` and it is **not** alphabetical, not
/// x/y/z-grouped, and not the `data3d` order (which is coincidentally the same, but by
/// design: `data3d` is assigned 0..5 in declaration order). Vanilla writes several of
/// these numbers to disk and over the wire:
///
/// * `Direction.ordinal()` reaches `level.dat` and, via `ByteBufCodecs.idMapper`,
///   the network protocol as `data3d`.
/// * `Direction#from3DDataValue(int)` is what a save file is read back through.
///
/// So the `Discriminant` values below are load-bearing. Reordering the variants would
/// silently reinterpret every saved world and every packet. `ordinal_is_stable` pins
/// them.
///
/// # `data3d` vs ordinal
///
/// In 26.2 they coincide, but they are conceptually different: `data3d` is the protocol
/// id and `oppositeIndex` is a *separate* hand-written index. `getOpposite()` does NOT
/// use `data3d ^ 1` -- it looks up `from3DDataValue(oppositeIndex)`, because the two
/// pairings are not the same permutation:
///
/// ```text
/// id:  DOWN UP NORTH SOUTH WEST EAST
/// opp:  1   0    3     2     5    4     <- XOR 1, works
/// axis: Y   Y    Z     Z     X    X     <- Y pairs 0<->1, Z pairs 2<->3, X pairs 4<->5
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    /// Port of `Direction#DOWN` -- ordinal 0, `data3d` 0, `data2d` -1, axis Y/NEGATIVE.
    Down = 0,
    /// Port of `Direction#UP` -- ordinal 1, `data3d` 1, `data2d` -1, axis Y/POSITIVE.
    Up = 1,
    /// Port of `Direction#NORTH` -- ordinal 2, `data3d` 2, `data2d` 2, axis Z/NEGATIVE.
    North = 2,
    /// Port of `Direction#SOUTH` -- ordinal 3, `data3d` 3, `data2d` 0, axis Z/POSITIVE.
    South = 3,
    /// Port of `Direction#WEST` -- ordinal 4, `data3d` 4, `data2d` 1, axis X/NEGATIVE.
    West = 4,
    /// Port of `Direction#EAST` -- ordinal 5, `data3d` 5, `data2d` 3, axis X/POSITIVE.
    East = 5,
}

/// Port of `Direction#values()` -- the shared array.
///
/// Java caches this in `private static final Direction[] VALUES = values();` and every
/// lookup table below is built from it. Order is declaration order.
pub const VALUES: [Direction; 6] = [
    Direction::Down,
    Direction::Up,
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

impl Direction {
    /// Port of `Direction#values().length`.
    pub const COUNT: usize = 6;

    /// Port of `Direction#CODEC`.
    pub fn CODEC() -> () {
        todo!("PORT: codec — Direction#CODEC")
    }

    /// Port of `Direction#VERTICAL_CODEC`.
    ///
    /// Java: `CODEC.validate(Direction::verifyVertical)`, whose error text is
    /// `"Expected a vertical direction"` -- player-visible, so carry it over verbatim.
    pub fn VERTICAL_CODEC() -> () {
        todo!("PORT: codec — Direction#VERTICAL_CODEC")
    }

    /// Port of `Direction#STREAM_CODEC`.
    ///
    /// `ByteBufCodecs.idMapper(BY_ID, Direction::get3DDataValue)` -- netty, batch 10.
    pub fn STREAM_CODEC() -> () {
        todo!("PORT: codec — Direction#STREAM_CODEC")
    }

    /// Port of `Direction#LEGACY_ID_CODEC`.
    pub fn LEGACY_ID_CODEC() -> () {
        todo!("PORT: codec — Direction#LEGACY_ID_CODEC")
    }

    /// Port of `Direction#LEGACY_ID_CODEC_2D`.
    pub fn LEGACY_ID_CODEC_2D() -> () {
        todo!("PORT: codec — Direction#LEGACY_ID_CODEC_2D")
    }

    /// Port of `Direction#BY_ID`.
    ///
    /// ```java
    /// public static final IntFunction<Direction> BY_ID =
    ///     ByIdMap.continuous(Direction::get3DDataValue, values(), ByIdMap.OutOfBoundsStrategy.WRAP);
    /// ```
    ///
    /// # `BY_ID` AND `from3DDataValue` DISAGREE ON NEGATIVE IDS
    ///
    /// I assumed these were the same function. They are not, and the difference is
    /// silent:
    ///
    /// | input | `from3DDataValue` | `BY_ID` |
    /// |---|---|---|
    /// | `0..5` | identity | identity |
    /// | `6`, `7` | `0`, `1` | `0`, `1` |
    /// | `-1` | `1` (from `Mth.abs(-1 % 6)`) | **`5`** |
    /// | `-2` | `2` | **`4`** |
    /// | `-100` | `4` | **`2`** |
    ///
    /// `from3DDataValue` is `BY_3D_DATA[Mth.abs(data % len)]` -- an `abs` of Java's
    /// *truncating* remainder, so `-1` becomes `+1`. `ByIdMap`'s `WRAP` strategy is a
    /// true wrap, i.e. a **floor** modulo, so `-1` becomes `5`.
    ///
    /// They agree on every non-negative input, which is exactly why this survives a
    /// casual reading and would only bite on a negative id from a malformed packet or a
    /// mod. So they are separate functions here; one must not delegate to the other.
    #[inline]
    pub fn by_id(data: i32) -> Direction {
        VALUES[java_lang::floor_mod_i32(data, VALUES.len() as i32) as usize]
    }

    /// Port of `Direction#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `Direction#valueOf(int)` -- Java throws for an out-of-range ordinal;
    /// we panic, which is the same crash (`DESIGN_DECISIONS.md` #panic-not-Result).
    #[inline]
    pub fn from_ordinal(ordinal: i32) -> Direction {
        assert!(
            (0..6).contains(&ordinal),
            "no enum constant Direction.{ordinal}"
        );
        VALUES[ordinal as usize]
    }

    /// Port of `Direction#get3DDataValue()`.
    ///
    /// The protocol id. In 26.2 it equals the ordinal, but it is stored and transmitted
    /// separately, so it gets its own accessor.
    #[inline]
    pub const fn get_3d_data_value(&self) -> i32 {
        *self as i32
    }

    /// Port of `Direction#get2DDataValue()`.
    ///
    /// Only meaningful for the horizontal directions. Note the values are NOT the
    /// `data3d` values: `DOWN` and `UP` both use -1, and the horizontals run
    /// SOUTH=0, WEST=1, NORTH=2, EAST=3 -- a different permutation.
    #[inline]
    pub const fn get_2d_data_value(&self) -> i32 {
        match self {
            Direction::Down | Direction::Up => -1,
            Direction::South => 0,
            Direction::West => 1,
            Direction::North => 2,
            Direction::East => 3,
        }
    }

    /// Port of the `oppositeIndex` constructor argument.
    #[inline]
    const fn opposite_index(&self) -> i32 {
        match self {
            Direction::Down => 1,
            Direction::Up => 0,
            Direction::North => 3,
            Direction::South => 2,
            Direction::West => 5,
            Direction::East => 4,
        }
    }

    /// Port of `Direction#getName()` / `toString()` / `getSerializedName()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Direction::Down => "down",
            Direction::Up => "up",
            Direction::North => "north",
            Direction::South => "south",
            Direction::West => "west",
            Direction::East => "east",
        }
    }

    /// Port of `Direction#getAxis()`.
    #[inline]
    pub const fn get_axis(&self) -> Axis {
        match self {
            Direction::Down | Direction::Up => Axis::Y,
            Direction::North | Direction::South => Axis::Z,
            Direction::West | Direction::East => Axis::X,
        }
    }

    /// Port of `Direction#getAxisDirection()`.
    #[inline]
    pub const fn get_axis_direction(&self) -> AxisDirection {
        match self {
            Direction::Down | Direction::North | Direction::West => AxisDirection::Negative,
            Direction::Up | Direction::South | Direction::East => AxisDirection::Positive,
        }
    }

    /// Port of `Direction#getUnitVec3i()`.
    #[inline]
    pub const fn get_unit_vec3i(&self) -> Vec3i {
        match self {
            Direction::Down => Vec3i::new(0, -1, 0),
            Direction::Up => Vec3i::new(0, 1, 0),
            Direction::North => Vec3i::new(0, 0, -1),
            Direction::South => Vec3i::new(0, 0, 1),
            Direction::West => Vec3i::new(-1, 0, 0),
            Direction::East => Vec3i::new(1, 0, 0),
        }
    }

    /// Port of `Direction#getStepX()`.
    #[inline]
    pub const fn get_step_x(&self) -> i32 {
        self.get_unit_vec3i().get_x()
    }

    /// Port of `Direction#getStepY()`.
    #[inline]
    pub const fn get_step_y(&self) -> i32 {
        self.get_unit_vec3i().get_y()
    }

    /// Port of `Direction#getStepZ()`.
    #[inline]
    pub const fn get_step_z(&self) -> i32 {
        self.get_unit_vec3i().get_z()
    }

    /// Port of `Direction#getOpposite()`.
    ///
    /// Goes through `from3DDataValue(oppositeIndex)`, i.e. a table lookup, rather than
    /// the tempting `data3d ^ 1`. They agree today, but only because the constructor
    /// arguments were written that way; matching the mechanism keeps it correct if a
    /// future version breaks the coincidence.
    #[inline]
    pub fn get_opposite(&self) -> Direction {
        Direction::from_3d_data_value(self.opposite_index())
    }

    /// Port of `Direction#isVertical()`.
    #[inline]
    pub const fn is_vertical(&self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    /// Port of `Direction#isHorizontal()`.
    #[inline]
    pub const fn is_horizontal(&self) -> bool {
        matches!(self, Direction::North | Direction::South | Direction::West | Direction::East)
    }

    // -----------------------------------------------------------------------
    // rotation
    // -----------------------------------------------------------------------

    /// Port of `Direction#getClockWise()` -- rotation about Y only.
    ///
    /// # Panics
    ///
    /// Java throws `IllegalStateException` for the vertical directions. We panic,
    /// which is the same crash.
    #[inline]
    pub fn get_clock_wise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
            other => panic!("Unable to get Y-rotated facing of {other}"),
        }
    }

    /// Port of `Direction#getCounterClockWise()`.
    #[inline]
    pub fn get_counter_clock_wise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
            other => panic!("Unable to get CCW facing of {other}"),
        }
    }

    /// Port of `Direction#getClockWiseX()`.
    ///
    /// ```java
    /// case DOWN -> SOUTH; case UP -> NORTH; case NORTH -> DOWN; case SOUTH -> UP;
    /// default -> throw new IllegalStateException(...)
    /// ```
    ///
    /// # CW AND CCW ARE TRANSPOSED, AND SWAPPING THEM IS EASY
    ///
    /// `getClockWiseX` and `getCounterClockWiseX` are exact mirrors:
    /// CW is `DOWN->SOUTH, UP->NORTH, NORTH->DOWN, SOUTH->UP` while CCW is
    /// `DOWN->NORTH, UP->SOUTH, NORTH->UP, SOUTH->DOWN`. Same for Z. Writing CCW's
    /// table under CW's name compiles fine, throws on the same inputs, and only shows
    /// up as blocks facing the wrong way -- so the tables are transcribed, not derived.
    #[inline]
    pub fn get_clock_wise_x(&self) -> Direction {
        match self {
            Direction::Down => Direction::South,
            Direction::Up => Direction::North,
            Direction::North => Direction::Down,
            Direction::South => Direction::Up,
            other => panic!("Unable to get X-rotated facing of {other}"),
        }
    }

    /// Port of `Direction#getCounterClockWiseX()`.
    #[inline]
    pub fn get_counter_clock_wise_x(&self) -> Direction {
        match self {
            Direction::Down => Direction::North,
            Direction::Up => Direction::South,
            Direction::North => Direction::Up,
            Direction::South => Direction::Down,
            other => panic!("Unable to get CCW X-rotated facing of {other}"),
        }
    }

    /// Port of `Direction#getClockWiseZ()`.
    #[inline]
    pub fn get_clock_wise_z(&self) -> Direction {
        match self {
            Direction::Down => Direction::West,
            Direction::Up => Direction::East,
            Direction::West => Direction::Up,
            Direction::East => Direction::Down,
            other => panic!("Unable to get Z-rotated facing of {other}"),
        }
    }

    /// Port of `Direction#getCounterClockWiseZ()`.
    #[inline]
    pub fn get_counter_clock_wise_z(&self) -> Direction {
        match self {
            Direction::Down => Direction::East,
            Direction::Up => Direction::West,
            Direction::West => Direction::Down,
            Direction::East => Direction::Up,
            other => panic!("Unable to get CCW Z-rotated facing of {other}"),
        }
    }

    /// Port of `Direction#getClockWise(Axis)`.
    ///
    /// Note the guard: asking for a CW rotation about an axis this direction *lies on*
    /// returns `this`, it does not rotate. `UP.getClockWise(Y)` is therefore `UP`.
    #[inline]
    pub fn get_clock_wise_axis(&self, axis: Axis) -> Direction {
        match axis {
            Axis::X => {
                if *self != Direction::West && *self != Direction::East {
                    self.get_clock_wise_x()
                } else {
                    *self
                }
            }
            Axis::Y => {
                if *self != Direction::Up && *self != Direction::Down {
                    self.get_clock_wise()
                } else {
                    *self
                }
            }
            Axis::Z => {
                if *self != Direction::North && *self != Direction::South {
                    self.get_clock_wise_z()
                } else {
                    *self
                }
            }
        }
    }

    /// Port of `Direction#getCounterClockWise(Axis)`.
    #[inline]
    pub fn get_counter_clock_wise_axis(&self, axis: Axis) -> Direction {
        match axis {
            Axis::X => {
                if *self != Direction::West && *self != Direction::East {
                    self.get_counter_clock_wise_x()
                } else {
                    *self
                }
            }
            Axis::Y => {
                if *self != Direction::Up && *self != Direction::Down {
                    self.get_counter_clock_wise()
                } else {
                    *self
                }
            }
            Axis::Z => {
                if *self != Direction::North && *self != Direction::South {
                    self.get_counter_clock_wise_z()
                } else {
                    *self
                }
            }
        }
    }

    // -----------------------------------------------------------------------
    // lookup
    // -----------------------------------------------------------------------

    /// Port of `Direction#from3DDataValue(int)`.
    ///
    /// `BY_3D_DATA` is `values()` sorted by `data3d`. Since `data3d` currently equals the
    /// ordinal, the sort is the identity -- but it is specified as a sort, so the
    /// implementation here also sorts rather than assuming.
    ///
    /// The `Math.abs(data % len)` wrapper is what makes negative and wildly out-of-range
    /// inputs wrap rather than throw. `-1 % 6 == -1`, `abs(-1) == 1` -> `UP`.
    #[inline]
    pub fn from_3d_data_value(data: i32) -> Direction {
        let mut sorted = VALUES;
        sorted.sort_by_key(|d| d.get_3d_data_value());
        let idx = java_lang::abs_i32(data % sorted.len() as i32) as usize;
        sorted[idx]
    }

    /// Port of `Direction#from2DDataValue(int)`.
    ///
    /// `BY_2D_DATA` is `values()` **filtered to horizontal** then sorted by `data2d`, so
    /// the array is `SOUTH, WEST, NORTH, EAST` (2d ids 0,1,2,3) with a length of 4 -- not
    /// 6. Getting that filter wrong makes `data2d` lookups wrap at the wrong modulus.
    #[inline]
    pub fn from_2d_data_value(data: i32) -> Direction {
        let mut sorted: [Direction; 4] = [
            Direction::South,
            Direction::West,
            Direction::North,
            Direction::East,
        ];
        sorted.sort_by_key(|d| d.get_2d_data_value());
        let idx = java_lang::abs_i32(data % sorted.len() as i32) as usize;
        sorted[idx]
    }

    /// Port of `Direction#fromYRot(double)`.
    #[inline]
    pub fn from_y_rot(y_rot: f64) -> Direction {
        Direction::from_2d_data_value(Mth_floor(y_rot / 90.0 + 0.5) & 3)
    }

    /// Port of `Direction#get(AxisDirection, Axis)`.
    ///
    /// Scans `VALUES` in declaration order and returns the first match.
    ///
    /// # Panics
    ///
    /// Java throws `IllegalArgumentException("No such direction: ...")`. We panic, and
    /// keep the message so the failure is diagnosable.
    pub fn get(axis_direction: AxisDirection, axis: Axis) -> Direction {
        for d in VALUES {
            if d.get_axis_direction() == axis_direction && d.get_axis() == axis {
                return d;
            }
        }
        panic!("No such direction: {axis_direction:?} {axis:?}");
    }

    /// Port of `Direction#byName(String)`.
    ///
    /// Java delegates to a codec and gets `Optional`-like behaviour; the `Option` is the
    /// faithful translation.
    pub fn by_name(name: &str) -> Option<Direction> {
        VALUES.iter().copied().find(|d| d.name() == name)
    }

    // -----------------------------------------------------------------------
    // nearest
    // -----------------------------------------------------------------------

    /// Port of `Direction#getNearest(int,int,int,Direction)`.
    ///
    /// `orElse` is Java's `@Nullable` fallback, returned when two axes tie. Note the
    /// comparison order: X, then Z, then Y -- and each branch requires a STRICT `>`, so
    /// a tie falls through to the next test and, ultimately, to `or_else`.
    #[inline]
    pub fn get_nearest(x: i32, y: i32, z: i32, or_else: Option<Direction>) -> Option<Direction> {
        let abs_x = java_lang::abs_i32(x);
        let abs_y = java_lang::abs_i32(y);
        let abs_z = java_lang::abs_i32(z);
        if abs_x > abs_z && abs_x > abs_y {
            Some(if x < 0 { Direction::West } else { Direction::East })
        } else if abs_z > abs_x && abs_z > abs_y {
            Some(if z < 0 { Direction::North } else { Direction::South })
        } else if abs_y > abs_x && abs_y > abs_z {
            Some(if y < 0 { Direction::Down } else { Direction::Up })
        } else {
            or_else
        }
    }

    /// Port of `Direction#getNearest(Vec3i, Direction)`.
    #[inline]
    pub fn get_nearest_vec3i(pos: &Vec3i, or_else: Option<Direction>) -> Option<Direction> {
        Direction::get_nearest(pos.get_x(), pos.get_y(), pos.get_z(), or_else)
    }

    /// Port of `Direction#getApproximateNearest(float,float,float)`.
    ///
    /// # THE INITIAL DOT PRODUCT IS `Float.MIN_VALUE`, NOT NEGATIVE INFINITY
    ///
    /// ```java
    /// Direction result = NORTH;
    /// float highestDot = Float.MIN_VALUE;
    /// ```
    ///
    /// `Float.MIN_VALUE` is the smallest POSITIVE denormal, `1.4e-45`. Starting from a
    /// positive value rather than `-Infinity` means a direction whose dot product is
    /// exactly zero, or a small negative, does NOT win -- `NORTH` stays the result
    /// because the initial `highestDot` was already positive. Using `-f32::MAX` or
    /// `-f32::INFINITY` as the seed would pick a different direction for near-zero
    /// vectors.
    ///
    /// The dot product is also assembled in plain `float` left-to-right
    /// (`dx*nx + dy*ny + dz*nz`), NOT via `Mth#dot`, so it does not use JOML's FMA.
    #[inline]
    pub fn get_approximate_nearest_f32(dx: f32, dy: f32, dz: f32) -> Direction {
        let mut result = Direction::North;
        let mut highest_dot = f32::MIN_POSITIVE;
        for direction in VALUES {
            let normal = direction.get_unit_vec3i();
            let dot = java_lang::add_f32(
                java_lang::add_f32(
                    java_lang::mul_f32(dx, normal.get_x() as f32),
                    java_lang::mul_f32(dy, normal.get_y() as f32),
                ),
                java_lang::mul_f32(dz, normal.get_z() as f32),
            );
            if dot > highest_dot {
                highest_dot = dot;
                result = direction;
            }
        }
        result
    }

    /// Port of `Direction#getApproximateNearest(double,double,double)`.
    ///
    /// Java NARROWS each argument to `float` first, then calls the `float` overload. The
    /// `(float)` casts are load-bearing: passing a double that is not representable in
    /// f32 changes which direction wins.
    #[inline]
    pub fn get_approximate_nearest_f64(dx: f64, dy: f64, dz: f64) -> Direction {
        Direction::get_approximate_nearest_f32(dx as f32, dy as f32, dz as f32)
    }

    /// Port of `Vec3i#relative(Direction, int)`, exposed on `Direction` because the
    /// oracle and many call sites read more naturally this way.
    #[inline]
    pub fn relative_to(&self, from: Vec3i, steps: i32) -> Vec3i {
        if steps == 0 {
            from
        } else {
            from.offset(
                self.get_step_x().wrapping_mul(steps),
                self.get_step_y().wrapping_mul(steps),
                self.get_step_z().wrapping_mul(steps),
            )
        }
    }
}

/// Port of `Direction#toString()`.
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Port of `Direction.Axis`.
///
/// Declaration order `X, Y, Z` matches the Java enum and therefore its ordinals.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Axis {
    /// Port of `Direction.Axis#X` -- ordinal 0.
    X = 0,
    /// Port of `Direction.Axis#Y` -- ordinal 1.
    Y = 1,
    /// Port of `Direction.Axis#Z` -- ordinal 2.
    Z = 2,
}

/// Port of `Direction.Axis#values()`.
pub const AXIS_VALUES: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];

impl Axis {
    /// Port of `Direction.Axis#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `Direction.Axis#valueOf(int)`.
    #[inline]
    pub fn from_ordinal(ordinal: i32) -> Axis {
        assert!((0..3).contains(&ordinal), "no enum constant Axis.{ordinal}");
        AXIS_VALUES[ordinal as usize]
    }

    /// Port of `Direction.Axis#getName()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
        }
    }

    /// Port of `Direction.Axis#byName(String)`.
    pub fn by_name(name: &str) -> Option<Axis> {
        AXIS_VALUES.iter().copied().find(|a| a.name() == name)
    }

    /// Port of `Direction.Axis#isVertical()`.
    #[inline]
    pub const fn is_vertical(&self) -> bool {
        matches!(self, Axis::Y)
    }

    /// Port of `Direction.Axis#isHorizontal()`.
    #[inline]
    pub const fn is_horizontal(&self) -> bool {
        matches!(self, Axis::X | Axis::Z)
    }

    /// Port of `Direction.Axis#getPositive()`.
    #[inline]
    pub const fn get_positive(&self) -> Direction {
        match self {
            Axis::X => Direction::East,
            Axis::Y => Direction::Up,
            Axis::Z => Direction::South,
        }
    }

    /// Port of `Direction.Axis#getNegative()`.
    #[inline]
    pub const fn get_negative(&self) -> Direction {
        match self {
            Axis::X => Direction::West,
            Axis::Y => Direction::Down,
            Axis::Z => Direction::North,
        }
    }

    /// Port of `Direction.Axis#getDirections()` -- positive first, then negative.
    #[inline]
    pub const fn get_directions(&self) -> [Direction; 2] {
        [self.get_positive(), self.get_negative()]
    }

    /// Port of `Direction.Axis#test(Direction)` -- the `Predicate<Direction>` method.
    #[inline]
    pub const fn test(&self, input: Option<Direction>) -> bool {
        match input {
            Some(d) => d.get_axis() as u8 == *self as u8,
            None => false,
        }
    }

    /// Port of `Direction.Axis#choose(double,double,double)`.
    ///
    /// Added while porting `AABB`, whose `min(Axis)`/`max(Axis)` are the only callers in the
    /// ported set. Java's version is a ternary chain; this is the same dispatch, and it stays a
    /// total function so `AABB::min`/`max` need no out-of-range branch.
    ///
    /// The three arguments are **eagerly evaluated** at every call site. That is not a
    /// behavioural difference here -- `AABB` passes three plain field reads, so there is nothing
    /// to elide -- but it is why this is written as a `match` on the axis rather than something
    /// lazy.
    #[inline]
    pub fn choose(&self, x: f64, y: f64, z: f64) -> f64 {
        match self {
            Axis::X => x,
            Axis::Y => y,
            Axis::Z => z,
        }
    }

    /// Port of `Vec3i#relative(Axis, int)`.
    #[inline]
    pub fn relative_to(&self, from: Vec3i, steps: i32) -> Vec3i {
        if steps == 0 {
            return from;
        }
        let x_step = if *self == Axis::X { steps } else { 0 };
        let y_step = if *self == Axis::Y { steps } else { 0 };
        let z_step = if *self == Axis::Z { steps } else { 0 };
        from.offset(x_step, y_step, z_step)
    }
}

/// Port of `Direction.Plane`.
///
/// Not ported. It exists for `Direction#getAxisDirection`-style flattening and is only
/// reachable from code we have not reached yet; it is a plain enum of (axis, axisDir)
/// pairs with a lookup, so it is a short port when something needs it. Recorded here as
/// a `todo!` rather than left silently missing.
pub fn PLANE() -> () {
    todo!("PORT: Direction.Plane")
}

/// Port of `Direction.Axis#toString()`.
impl std::fmt::Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Port of `Direction.AxisDirection`.
///
/// Note the declaration order is `POSITIVE, NEGATIVE` -- POSITIVE is ordinal 0. The
/// `data` field is `+1` / `-1` respectively.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AxisDirection {
    /// Port of `Direction.AxisDirection#POSITIVE` -- ordinal 0, data +1.
    Positive = 0,
    /// Port of `Direction.AxisDirection#NEGATIVE` -- ordinal 1, data -1.
    Negative = 1,
}

impl AxisDirection {
    /// Port of `Direction.AxisDirection#values()`.
    pub const VALUES: [AxisDirection; 2] = [AxisDirection::Positive, AxisDirection::Negative];

    /// Port of `Direction.AxisDirection#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `Direction.AxisDirection#getStep()` -- the `data` field.
    #[inline]
    pub const fn get_step(&self) -> i32 {
        match self {
            AxisDirection::Positive => 1,
            AxisDirection::Negative => -1,
        }
    }

    /// Port of `Direction.AxisDirection#getDescription()`.
    #[inline]
    pub const fn get_description(&self) -> &'static str {
        match self {
            AxisDirection::Positive => "Towards positive",
            AxisDirection::Negative => "Towards negative",
        }
    }
}

/// Port of `Direction.AxisDirection#toString()` -- note it is the DESCRIPTION, not the
/// enum constant name.
impl std::fmt::Display for AxisDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_description())
    }
}

/// `Math.floor` as used by `Direction#fromYRot`.
///
/// Local rather than reaching into `Mth` to keep this module free of the
/// `Mth`-imports-`Vec3i`-imports-`Direction` cycle that the mirror's nesting creates.
#[inline]
fn Mth_floor(x: f64) -> i32 {
    let f = x.floor();
    if f.is_nan() || f >= i32::MAX as f64 {
        i32::MAX
    } else if f <= i32::MIN as f64 {
        i32::MIN
    } else {
        f as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinal_is_stable_and_matches_data3d() {
        // Persisted in level.dat and sent over the wire. Reordering breaks both.
        assert_eq!(Direction::Down.ordinal(), 0);
        assert_eq!(Direction::Up.ordinal(), 1);
        assert_eq!(Direction::North.ordinal(), 2);
        assert_eq!(Direction::South.ordinal(), 3);
        assert_eq!(Direction::West.ordinal(), 4);
        assert_eq!(Direction::East.ordinal(), 5);
        for d in VALUES {
            assert_eq!(d.ordinal(), d.get_3d_data_value());
        }
    }

    #[test]
    fn axis_ordinals_are_x_y_z() {
        assert_eq!(Axis::X.ordinal(), 0);
        assert_eq!(Axis::Y.ordinal(), 1);
        assert_eq!(Axis::Z.ordinal(), 2);
    }

    #[test]
    fn axis_direction_ordinals_are_positive_first() {
        // POSITIVE is ordinal 0 -- the opposite of the obvious guess.
        assert_eq!(AxisDirection::Positive.ordinal(), 0);
        assert_eq!(AxisDirection::Negative.ordinal(), 1);
    }

    #[test]
    fn data2d_is_a_different_permutation_than_data3d() {
        assert_eq!(Direction::Down.get_2d_data_value(), -1);
        assert_eq!(Direction::Up.get_2d_data_value(), -1);
        assert_eq!(Direction::South.get_2d_data_value(), 0);
        assert_eq!(Direction::West.get_2d_data_value(), 1);
        assert_eq!(Direction::North.get_2d_data_value(), 2);
        assert_eq!(Direction::East.get_2d_data_value(), 3);
    }

    #[test]
    fn from_2d_data_value_wraps_at_four_not_six() {
        // The horizontal-only filter means the modulus is 4. If this were 6 the
        // results for 4 and 5 would be wrong.
        assert_eq!(Direction::from_2d_data_value(0), Direction::South);
        assert_eq!(Direction::from_2d_data_value(1), Direction::West);
        assert_eq!(Direction::from_2d_data_value(2), Direction::North);
        assert_eq!(Direction::from_2d_data_value(3), Direction::East);
        assert_eq!(Direction::from_2d_data_value(4), Direction::South);
        // Negative input: `abs(-1 % 4)` = 1 -> WEST, `abs(-2 % 4)` = 2 -> NORTH.
        assert_eq!(Direction::from_2d_data_value(-1), Direction::West);
        assert_eq!(Direction::from_2d_data_value(-2), Direction::North);
        assert_eq!(Direction::from_2d_data_value(-4), Direction::South);
    }

    #[test]
    fn from_3d_data_value_wraps_and_absorbs_negatives() {
        assert_eq!(Direction::from_3d_data_value(0), Direction::Down);
        assert_eq!(Direction::from_3d_data_value(5), Direction::East);
        assert_eq!(Direction::from_3d_data_value(6), Direction::Down);
        assert_eq!(Direction::from_3d_data_value(-1), Direction::Up);
    }

    #[test]
    fn opposite_is_an_involution() {
        for d in VALUES {
            assert_eq!(d.get_opposite().get_opposite(), d);
            assert_ne!(d.get_opposite(), d);
        }
        assert_eq!(Direction::North.get_opposite(), Direction::South);
        assert_eq!(Direction::West.get_opposite(), Direction::East);
        assert_eq!(Direction::Up.get_opposite(), Direction::Down);
    }

    #[test]
    fn clockwise_about_y_is_a_four_cycle() {
        let mut d = Direction::North;
        for _ in 0..4 {
            d = d.get_clock_wise();
        }
        assert_eq!(d, Direction::North);
        assert_eq!(Direction::North.get_clock_wise(), Direction::East);
        assert_eq!(Direction::East.get_clock_wise(), Direction::South);
        assert_eq!(Direction::South.get_clock_wise(), Direction::West);
        assert_eq!(Direction::West.get_clock_wise(), Direction::North);
    }

    #[test]
    fn clockwise_about_the_axis_you_lie_on_is_identity() {
        assert_eq!(Direction::East.get_clock_wise_axis(Axis::X), Direction::East);
        assert_eq!(Direction::Up.get_clock_wise_axis(Axis::Y), Direction::Up);
        assert_eq!(Direction::North.get_clock_wise_axis(Axis::Z), Direction::North);
        // ...but off-axis it still rotates.
        assert_eq!(Direction::East.get_clock_wise_axis(Axis::Y), Direction::South);
    }

    #[test]
    #[should_panic(expected = "Unable to get Y-rotated facing")]
    fn clockwise_rejects_vertical() {
        let _ = Direction::Up.get_clock_wise();
    }

    #[test]
    fn get_nearest_breaks_ties_towards_the_fallback() {
        // All zero: every strict comparison fails, so the fallback comes back.
        assert_eq!(Direction::get_nearest(0, 0, 0, None), None);
        assert_eq!(
            Direction::get_nearest(0, 0, 0, Some(Direction::West)),
            Some(Direction::West)
        );
        assert_eq!(Direction::get_nearest(1, 0, 0, None), Some(Direction::East));
        assert_eq!(Direction::get_nearest(-1, 0, 0, None), Some(Direction::West));
        assert_eq!(Direction::get_nearest(0, 1, 0, None), Some(Direction::Up));
        assert_eq!(Direction::get_nearest(0, 0, 1, None), Some(Direction::South));
        // x and z tie at 5, y is smaller: neither branch fires -> fallback.
        assert_eq!(
            Direction::get_nearest(5, 1, 5, Some(Direction::Down)),
            Some(Direction::Down)
        );
    }

    #[test]
    fn approximate_nearest_seeds_from_float_min_value() {
        // A vector pointing straight DOWN has dot 1.0 with DOWN.
        assert_eq!(Direction::get_approximate_nearest_f32(0.0, -1.0, 0.0), Direction::Down);
        assert_eq!(Direction::get_approximate_nearest_f32(0.0, 1.0, 0.0), Direction::Up);
        assert_eq!(Direction::get_approximate_nearest_f32(1.0, 0.0, 0.0), Direction::East);
        assert_eq!(Direction::get_approximate_nearest_f32(0.0, 0.0, -1.0), Direction::North);
        // An all-zero vector: every dot is 0, and 0 > Float.MIN_VALUE is false, so
        // nothing ever wins and NORTH (the initial `result`) is returned.
        assert_eq!(
            Direction::get_approximate_nearest_f32(0.0, 0.0, 0.0),
            Direction::North,
            "a zero vector must not win against a Float.MIN_VALUE seed"
        );
    }

    #[test]
    fn approximate_nearest_narrows_doubles_first() {
        // 1e40 is not representable in f32 -> becomes +inf, and inf*0 is NaN, so the
        // comparison is against NaN and NORTH wins. Passing the f64 values directly
        // would give a finite dot and a different answer.
        assert_eq!(Direction::get_approximate_nearest_f64(1e40, 1e40, 1e40), Direction::North);
    }

    #[test]
    fn relative_is_identity_at_zero_steps() {
        let p = Vec3i::new(1, 2, 3);
        assert_eq!(Direction::Up.relative_to(p, 0), p);
        assert_eq!(Direction::Up.relative_to(p, 5), Vec3i::new(1, 7, 3));
        assert_eq!(Direction::West.relative_to(p, 2), Vec3i::new(-1, 2, 3));
    }

    #[test]
    fn axis_relative_only_moves_its_own_component() {
        let p = Vec3i::new(1, 2, 3);
        assert_eq!(Axis::X.relative_to(p, 10), Vec3i::new(11, 2, 3));
        assert_eq!(Axis::Y.relative_to(p, 10), Vec3i::new(1, 12, 3));
        assert_eq!(Axis::Z.relative_to(p, 10), Vec3i::new(1, 2, 13));
        assert_eq!(Axis::Z.relative_to(p, 0), p);
    }

    #[test]
    fn get_finds_the_matching_direction() {
        assert_eq!(Direction::get(AxisDirection::Positive, Axis::X), Direction::East);
        assert_eq!(Direction::get(AxisDirection::Negative, Axis::X), Direction::West);
        assert_eq!(Direction::get(AxisDirection::Positive, Axis::Y), Direction::Up);
        assert_eq!(Direction::get(AxisDirection::Negative, Axis::Y), Direction::Down);
        assert_eq!(Direction::get(AxisDirection::Positive, Axis::Z), Direction::South);
        assert_eq!(Direction::get(AxisDirection::Negative, Axis::Z), Direction::North);
    }

    #[test]
    fn names_match_serialised_names() {
        assert_eq!(Direction::Down.name(), "down");
        assert_eq!(Direction::by_name("north"), Some(Direction::North));
        assert_eq!(Direction::by_name("Northwest"), None);
        assert_eq!(Direction::East.to_string(), "east");
        assert_eq!(Axis::Y.to_string(), "y");
        // AxisDirection's toString is its description, not its constant name.
        assert_eq!(AxisDirection::Positive.to_string(), "Towards positive");
    }

    #[test]
    fn axis_test_matches_java_predicate() {
        assert!(Axis::X.test(Some(Direction::East)));
        assert!(!Axis::X.test(Some(Direction::Up)));
        assert!(!Axis::X.test(None));
    }
}
