//! Port of: net/minecraft/world/entity/boss/enderdragon/DragonFlightHistory.java
//! Java class(es): net.minecraft.world.entity.boss.enderdragon.DragonFlightHistory, net.minecraft.world.entity.boss.enderdragon.Sample
//! Status: VERIFIED
//!
//! A 64-slot ring buffer of the dragon's past `(y, yRot)` used to pose its neck and tail
//! segments. Every member is ported, including the nested record `Sample` (equals / hashCode /
//! toString). `LENGTH` is checked against the constant dump; the ring-buffer behaviour,
//! interpolation and the record methods against a run of the 26.2 jar
//! (`_porting/tests/session19_e.rs`).
//!
//! # Ring-buffer details that matter
//!
//! - `head` starts at **-1** and the slots start as `Sample(0.0, 0.0F)`. The *first* `record`
//!   floods all 64 slots with the new sample before advancing, so a freshly spawned dragon has
//!   no fake "trail from the origin".
//! - `get(delay)` indexes `head - delay & 63` -- Java precedence makes that `(head - delay) & 63`,
//!   so any `int` delay (negative, or >= 64) wraps instead of throwing, and on a fresh history
//!   (`head == -1`) it reads slot 63.
//! - `get(delay, partialTicks)` lerps `y` in **double** (`Mth.lerp(double, ...)`, the float
//!   `partialTicks` widened) and `yRot` with `Mth.rotLerp(float, ...)`, which takes the short way
//!   round: 170 -> -170 at 0.5 gives 180, not 0.
//! - Java shares one immutable `Sample` instance between slots; `Sample` is `Copy` here, which
//!   is observably identical.

use crate::javacompat::java_lang::{compare_f32, compare_f64};
use crate::javacompat::java_lang::float_to_string::{double_to_string, float_to_string};
use crate::net::minecraft::util::Mth::Mth;

/// Java: `DragonFlightHistory.LENGTH` = `64`.
pub const LENGTH: i32 = 64;
/// Java: `private static final int MASK` = `63`.
const MASK: i32 = 63;

/// Port of `DragonFlightHistory`.
#[derive(Clone, Debug)]
pub struct DragonFlightHistory {
    samples: [Sample; LENGTH as usize],
    head: i32,
}

impl Default for DragonFlightHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl DragonFlightHistory {
    /// Port of `DragonFlightHistory()`: every slot `Sample(0.0, 0.0F)`, `head = -1`.
    pub fn new() -> Self {
        DragonFlightHistory { samples: [Sample::new(0.0, 0.0); LENGTH as usize], head: -1 }
    }

    /// Port of `DragonFlightHistory#copyFrom(DragonFlightHistory)`: all 64 slots and `head`.
    pub fn copy_from(&mut self, history: &DragonFlightHistory) {
        self.samples = history.samples;
        self.head = history.head;
    }

    /// Port of `DragonFlightHistory#record(double, float)`.
    pub fn record(&mut self, y: f64, y_rot: f32) {
        let sample = Sample::new(y, y_rot);
        if self.head < 0 {
            self.samples = [sample; LENGTH as usize];
        }
        self.head += 1;
        if self.head == LENGTH {
            self.head = 0;
        }
        self.samples[self.head as usize] = sample;
    }

    /// Port of `DragonFlightHistory#get(int)`: the sample `delay` records ago.
    pub fn get_i32(&self, delay: i32) -> Sample {
        // Java: `this.samples[this.head - delay & 63]` == `[(head - delay) & 63]`, never negative.
        self.samples[(self.head.wrapping_sub(delay) & MASK) as usize]
    }

    /// Port of `DragonFlightHistory#get(int, float)`: interpolates from `get(delay + 1)` to
    /// `get(delay)`.
    pub fn get_i32_f32(&self, delay: i32, partial_ticks: f32) -> Sample {
        let sample = self.get_i32(delay);
        let sample_old = self.get_i32(delay.wrapping_add(1));
        Sample::new(
            Mth::lerp_f64(partial_ticks as f64, sample_old.y, sample.y),
            Mth::rot_lerp_f32(partial_ticks, sample_old.y_rot, sample.y_rot),
        )
    }
}

/// Port of the record `DragonFlightHistory.Sample(double y, float yRot)`.
///
/// Record semantics: `equals` compares components with `Double.compare` / `Float.compare`
/// (so `NaN == NaN` and `0.0 != -0.0`), `hashCode` is `31 * Double.hashCode(y) +
/// Float.hashCode(yRot)`, `toString` is `Sample[y=..., yRot=...]` with Java's number formatting.
#[derive(Clone, Copy, Debug)]
pub struct Sample {
    y: f64,
    y_rot: f32,
}

impl Sample {
    /// Port of the canonical constructor `Sample(double, float)`.
    pub const fn new(y: f64, y_rot: f32) -> Self {
        Sample { y, y_rot }
    }

    /// Port of the record accessor `Sample#y()`.
    pub const fn y(&self) -> f64 {
        self.y
    }

    /// Port of the record accessor `Sample#yRot()`.
    pub const fn y_rot(&self) -> f32 {
        self.y_rot
    }

    /// Port of the record's `hashCode()`.
    pub fn java_hash_code(&self) -> i32 {
        // Double.hashCode: doubleToLongBits (canonical NaN), folded; Float.hashCode: floatToIntBits.
        let y_bits = if self.y.is_nan() { 0x7ff8000000000000_i64 } else { self.y.to_bits() as i64 };
        let y_hash = (y_bits ^ ((y_bits as u64) >> 32) as i64) as i32;
        let r_hash = if self.y_rot.is_nan() { 0x7fc00000_i32 } else { self.y_rot.to_bits() as i32 };
        y_hash.wrapping_mul(31).wrapping_add(r_hash)
    }
}

/// Record `equals`: `Double.compare(y, o.y) == 0 && Float.compare(yRot, o.yRot) == 0`.
impl PartialEq for Sample {
    fn eq(&self, other: &Self) -> bool {
        compare_f64(self.y, other.y).is_eq() && compare_f32(self.y_rot, other.y_rot).is_eq()
    }
}

impl Eq for Sample {}

impl std::hash::Hash for Sample {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.java_hash_code().hash(state);
    }
}

/// Record `toString`: `Sample[y=<Double.toString>, yRot=<Float.toString>]`.
impl std::fmt::Display for Sample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sample[y={}, yRot={}]", double_to_string(self.y), float_to_string(self.y_rot))
    }
}
