//! Port of: net/minecraft/world/DifficultyInstance.java
//! Java class(es): net.minecraft.world.DifficultyInstance
//! Status: VERIFIED
//!
//! Session 24. `@Immutable public class DifficultyInstance`: the "regional difficulty" of a place
//! and time -- a base [`Difficulty`] scaled by how long the world and the chunk have existed and by
//! the moon. Computed once in the constructor; five getters read it.
//!
//! ```java
//! if (base == PEACEFUL) return 0.0F;
//! float scale = 0.75F;
//! float globalScale = Mth.clamp(((float)totalGameTime + -72000.0F) / 1440000.0F, 0.0F, 1.0F) * 0.25F;
//! scale += globalScale;
//! float localScale = 0.0F;
//! localScale += Mth.clamp((float)localGameTime / 3600000.0F, 0.0F, 1.0F) * (isHard ? 1.0F : 0.75F);
//! localScale += Mth.clamp(moonBrightness * 0.25F, 0.0F, globalScale);
//! if (base == EASY) localScale *= 0.5F;
//! scale += localScale;
//! return base.getId() * scale;
//! ```
//!
//! Every step is `float` arithmetic, in that order, and the port keeps it: `long -> float` is
//! Rust's `as f32` (round to nearest, as `l2f`), and `int * float` widens the id to `float` first.
//! The three private constants are inlined in the Java bytecode exactly as above.
//!
//! `isHard()` compares with `Difficulty.HARD.ordinal()` (3, widened to `float`), not with
//! `getId()`; the two agree for every constant, and the port uses the discriminant, which is the
//! ordinal.
//!
//! Verified by the `DIFFINST` group of `sweep_probe.txt` (`tests/session24.rs`): every
//! difficulty over a grid of total/local times (negative, zero, the clamp corners, `Long`
//! extremes) and moon brightnesses (negative, zero, the clamp corner, NaN, infinities).
//! Computed NaNs are compared after `nan_canon` (OPEN_QUESTIONS #33).

use crate::net::minecraft::util::Mth::Mth;
use crate::net::minecraft::world::Difficulty::Difficulty;

/// Port of `private static final float DIFFICULTY_TIME_GLOBAL_OFFSET = -72000.0F`.
const DIFFICULTY_TIME_GLOBAL_OFFSET: f32 = -72000.0;
/// Port of `private static final float MAX_DIFFICULTY_TIME_GLOBAL = 1440000.0F`.
const MAX_DIFFICULTY_TIME_GLOBAL: f32 = 1_440_000.0;
/// Port of `private static final float MAX_DIFFICULTY_TIME_LOCAL = 3600000.0F`.
const MAX_DIFFICULTY_TIME_LOCAL: f32 = 3_600_000.0;

/// Port of `net.minecraft.world.DifficultyInstance`.
#[derive(Debug, Clone, Copy)]
pub struct DifficultyInstance {
    base: Difficulty,
    effective_difficulty: f32,
}

impl DifficultyInstance {
    /// Port of `public DifficultyInstance(Difficulty base, long totalGameTime, long localGameTime,
    /// float moonBrightness)`.
    pub fn new(
        base: Difficulty,
        total_game_time: i64,
        local_game_time: i64,
        moon_brightness: f32,
    ) -> Self {
        DifficultyInstance {
            base,
            effective_difficulty: Self::calculate_difficulty(
                base,
                total_game_time,
                local_game_time,
                moon_brightness,
            ),
        }
    }

    /// Port of `public Difficulty getDifficulty()`.
    pub fn get_difficulty(&self) -> Difficulty {
        self.base
    }

    /// Port of `public float getEffectiveDifficulty()`.
    pub fn get_effective_difficulty(&self) -> f32 {
        self.effective_difficulty
    }

    /// Port of `public boolean isHard()`: `effectiveDifficulty >= Difficulty.HARD.ordinal()`.
    pub fn is_hard(&self) -> bool {
        self.effective_difficulty >= Difficulty::Hard as i32 as f32
    }

    /// Port of `public boolean isHarderThan(float requiredDifficulty)`.
    pub fn is_harder_than(&self, required_difficulty: f32) -> bool {
        self.effective_difficulty > required_difficulty
    }

    /// Port of `public float getSpecialMultiplier()`.
    pub fn get_special_multiplier(&self) -> f32 {
        if self.effective_difficulty < 2.0 {
            0.0
        } else if self.effective_difficulty > 4.0 {
            1.0
        } else {
            (self.effective_difficulty - 2.0) / 2.0
        }
    }

    /// Port of `private float calculateDifficulty(Difficulty, long, long, float)`.
    fn calculate_difficulty(
        base: Difficulty,
        total_game_time: i64,
        local_game_time: i64,
        moon_brightness: f32,
    ) -> f32 {
        if base == Difficulty::Peaceful {
            return 0.0;
        }
        let is_hard = base == Difficulty::Hard;
        let mut scale = 0.75f32;
        let global_scale = Mth::clamp_f32(
            (total_game_time as f32 + DIFFICULTY_TIME_GLOBAL_OFFSET) / MAX_DIFFICULTY_TIME_GLOBAL,
            0.0,
            1.0,
        ) * 0.25;
        scale += global_scale;
        let mut local_scale = 0.0f32;
        local_scale += Mth::clamp_f32(local_game_time as f32 / MAX_DIFFICULTY_TIME_LOCAL, 0.0, 1.0)
            * if is_hard { 1.0 } else { 0.75 };
        local_scale += Mth::clamp_f32(moon_brightness * 0.25, 0.0, global_scale);
        if base == Difficulty::Easy {
            local_scale *= 0.5;
        }
        scale += local_scale;
        base.get_id() as f32 * scale
    }
}
