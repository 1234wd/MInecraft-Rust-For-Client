//! Port of: net/minecraft/client/renderer/EndFlashState.java
//! Java class(es): net.minecraft.client.renderer.EndFlashState
//! Status: VERIFIED
//!
//! Session 19 batch h. Every member: the five constants (the public `SOUND_DELAY_IN_TICKS` is
//! checked against the jar's dump, which does not record private fields; the four private ones
//! are module-private here and used where javac inlined their literals), `tick`, the private
//! `calculateFlashParameters`/`calculateIntensity`, and the getters. Nothing is PORT-BLOCKED: the
//! per-interval random source is `RandomSource.createThreadLocalInstance(seed)`, i.e. a seeded
//! `SingleThreadedRandomSource`, which is ported.
//!
//! # Vanilla quirks preserved
//!
//! * `flashSeed` starts at `0`, and the parameters are only recomputed when
//!   `clockTime / 600` differs from it. So for every `clockTime` in `-599..=599` (Java's `/`
//!   truncates toward zero) the parameters are never computed: `offset`, `duration` and both
//!   angles stay `0`, and the only "flash" is the degenerate `sin(0 * PI / 0)` = `sin(NaN)` at
//!   `clockTime % 600 == 0`, which `Mth.sin` maps to table index 0, i.e. `0.0`.
//! * `clockTime % 600` keeps the dividend's sign, so negative clock times never flash.
//! * The phase is `(float)(t - offset) * (float)Math.PI / duration` in `float`, then widened to
//!   `double` for `Mth.sin(double)`.

use crate::net::minecraft::util::Mth::Mth;
use crate::net::minecraft::util::RandomSource;

/// Port of `EndFlashState#SOUND_DELAY_IN_TICKS`.
pub const SOUND_DELAY_IN_TICKS: i32 = 30;
/// Port of the private `EndFlashState#FLASH_INTERVAL_IN_TICKS`.
const FLASH_INTERVAL_IN_TICKS: i32 = 600;
/// Port of the private `EndFlashState#MAX_FLASH_OFFSET_IN_TICKS`.
const MAX_FLASH_OFFSET_IN_TICKS: i32 = 200;
/// Port of the private `EndFlashState#MIN_FLASH_DURATION_IN_TICKS`.
const MIN_FLASH_DURATION_IN_TICKS: i32 = 100;
/// Port of the private `EndFlashState#MAX_FLASH_DURATION_IN_TICKS`.
const MAX_FLASH_DURATION_IN_TICKS: i32 = 380;

/// Port of `net.minecraft.client.renderer.EndFlashState`. `Default` is Java's implicit
/// constructor (every field zero).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EndFlashState {
    flash_seed: i64,
    offset: i32,
    duration: i32,
    intensity: f32,
    old_intensity: f32,
    x_angle: f32,
    y_angle: f32,
}

impl EndFlashState {
    /// Java's implicit no-arg constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Port of `EndFlashState#tick(long)`.
    pub fn tick(&mut self, clock_time: i64) {
        self.calculate_flash_parameters(clock_time);
        self.old_intensity = self.intensity;
        self.intensity = self.calculate_intensity(clock_time);
    }

    /// Port of the private `EndFlashState#calculateFlashParameters(long)`.
    fn calculate_flash_parameters(&mut self, clock_time: i64) {
        // Java `long / long`: truncates toward zero, like Rust's `/` (600 cannot overflow).
        let new_seed = clock_time / FLASH_INTERVAL_IN_TICKS as i64;
        if new_seed != self.flash_seed {
            let mut random_source = RandomSource::create_thread_local_instance(new_seed);
            let random_source = random_source.as_mut();
            random_source.next_float();
            self.offset = Mth::random_between_inclusive(random_source, 0, MAX_FLASH_OFFSET_IN_TICKS);
            self.duration = Mth::random_between_inclusive(
                random_source,
                MIN_FLASH_DURATION_IN_TICKS,
                std::cmp::min(MAX_FLASH_DURATION_IN_TICKS, FLASH_INTERVAL_IN_TICKS - self.offset),
            );
            self.x_angle = Mth::random_between(random_source, -60.0, 10.0);
            self.y_angle = Mth::random_between(random_source, -180.0, 180.0);
            self.flash_seed = new_seed;
        }
    }

    /// Port of the private `EndFlashState#calculateIntensity(long)`.
    fn calculate_intensity(&self, clock_time: i64) -> f32 {
        // Java `%` on long: sign of the dividend, same as Rust's `%`.
        let clock_time_within_interval = clock_time % FLASH_INTERVAL_IN_TICKS as i64;
        if clock_time_within_interval >= self.offset as i64
            && clock_time_within_interval <= (self.offset + self.duration) as i64
        {
            let phase = (clock_time_within_interval - self.offset as i64) as f32 * std::f64::consts::PI as f32
                / self.duration as f32;
            Mth::sin(phase as f64)
        } else {
            0.0
        }
    }

    /// Port of `EndFlashState#getXAngle()`.
    pub fn get_x_angle(&self) -> f32 {
        self.x_angle
    }

    /// Port of `EndFlashState#getYAngle()`.
    pub fn get_y_angle(&self) -> f32 {
        self.y_angle
    }

    /// Port of `EndFlashState#getIntensity(float)`: `Mth.lerp(partialTicks, oldIntensity, intensity)`.
    pub fn get_intensity(&self, partial_ticks: f32) -> f32 {
        Mth::lerp_f32(partial_ticks, self.old_intensity, self.intensity)
    }

    /// Port of `EndFlashState#flashStartedThisTick()`.
    pub fn flash_started_this_tick(&self) -> bool {
        self.intensity > 0.0 && self.old_intensity <= 0.0
    }

    /// Read access to the private `offset` field (tick within the 600-tick interval the flash
    /// starts at), for tests and debugging. Not a Java method.
    pub fn offset(&self) -> i32 {
        self.offset
    }

    /// Read access to the private `duration` field. Not a Java method.
    pub fn duration(&self) -> i32 {
        self.duration
    }

    /// Read access to the private `flashSeed` field. Not a Java method.
    pub fn flash_seed(&self) -> i64 {
        self.flash_seed
    }
}
