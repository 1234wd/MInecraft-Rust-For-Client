//! Port of: net/minecraft/world/entity/monster/warden/AngerLevel.java
//! Java class(es): net.minecraft.world.entity.monster.warden.AngerLevel
//! Status: PARTIAL
//!
//! Session 19. The three constants with `minimumAnger`, `getMinimumAnger()`, the private
//! `SORTED_LEVELS` table, `byAnger(int)` and `isAngry()` are ported. Ordinals and
//! `minimumAnger` are checked against the jar, and the `byAnger` thresholds are tested at every
//! boundary, in `_porting/tests/session19_c.rs`.
//!
//! PORT-BLOCKED on `net.minecraft.sounds.SoundEvent` / `SoundEvents` (both SKELETON):
//! `getAmbientSound()` and `getListeningSound()`. The jar records them as
//! `CALM: warden.ambient / warden.listening`, `AGITATED: warden.agitated /
//! warden.listening_angry`, `ANGRY: warden.angry / warden.listening_angry` (`entity.` prefix,
//! `minecraft:` namespace) -- `AGITATED` and `ANGRY` share the angry listening sound.
//!
//! # `byAnger` SCANS HIGHEST THRESHOLD FIRST, AND NEGATIVE ANGER IS `CALM`
//!
//! ```java
//! SORTED_LEVELS = Util.make(values(), v -> Arrays.sort(v, (a, b) -> Integer.compare(b.minimumAnger, a.minimumAnger)));
//! for (AngerLevel level : SORTED_LEVELS) if (anger >= level.minimumAnger) return level;
//! return CALM;
//! ```
//!
//! `Util.make` only applies the lambda and returns its argument, so the table is built here
//! directly (no dependency on the unported `Util`): `values()` sorted by `minimumAnger`
//! descending with a stable sort (Java's object `Arrays.sort` is TimSort, stable; so is Rust's
//! `sort_by`), giving `[ANGRY, AGITATED, CALM]`. Thresholds are inclusive (`>=`): 40 is
//! `AGITATED`, 80 is `ANGRY`. A negative anger fails every test (CALM's threshold is 0) and
//! reaches the trailing `return CALM`.

/// Port of `net.minecraft.world.entity.monster.warden.AngerLevel`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AngerLevel {
    /// Port of `AngerLevel#CALM` -- ordinal 0, minimum anger `0`.
    Calm = 0,
    /// Port of `AngerLevel#AGITATED` -- ordinal 1, minimum anger `40`.
    Agitated = 1,
    /// Port of `AngerLevel#ANGRY` -- ordinal 2, minimum anger `80`.
    Angry = 2,
}

/// Java: `AngerLevel.values()` -- declaration order.
pub const VALUES: [AngerLevel; 3] = [AngerLevel::Calm, AngerLevel::Agitated, AngerLevel::Angry];

/// Java: `private static final AngerLevel[] SORTED_LEVELS`. See the module docs.
static SORTED_LEVELS: std::sync::LazyLock<[AngerLevel; 3]> = std::sync::LazyLock::new(|| {
    let mut values = VALUES;
    // `(a, b) -> Integer.compare(b.minimumAnger, a.minimumAnger)`: descending, stable.
    values.sort_by(|a, b| b.get_minimum_anger().cmp(&a.get_minimum_anger()));
    values
});

impl AngerLevel {
    /// Java `Enum#name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            AngerLevel::Calm => "CALM",
            AngerLevel::Agitated => "AGITATED",
            AngerLevel::Angry => "ANGRY",
        }
    }

    /// Java `Enum#ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `AngerLevel#getMinimumAnger()`.
    #[inline]
    pub const fn get_minimum_anger(&self) -> i32 {
        match self {
            AngerLevel::Calm => 0,
            AngerLevel::Agitated => 40,
            AngerLevel::Angry => 80,
        }
    }

    /// Port of `AngerLevel#isAngry()`: `this == ANGRY` (not `AGITATED`).
    #[inline]
    pub const fn is_angry(&self) -> bool {
        matches!(self, AngerLevel::Angry)
    }

    /// Java: `public SoundEvent getAmbientSound()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.sounds.SoundEvent` (and the `SoundEvents.WARDEN_*`
    /// constants), which are SKELETON.
    pub fn get_ambient_sound(&self) {
        todo!("PORT-BLOCKED: net.minecraft.sounds.SoundEvent is not ported")
    }

    /// Java: `public SoundEvent getListeningSound()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.sounds.SoundEvent` (and the `SoundEvents.WARDEN_*`
    /// constants), which are SKELETON.
    pub fn get_listening_sound(&self) {
        todo!("PORT-BLOCKED: net.minecraft.sounds.SoundEvent is not ported")
    }
}

impl std::fmt::Display for AngerLevel {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Port of `AngerLevel#byAnger(int)`. See the module docs.
pub fn by_anger(anger: i32) -> AngerLevel {
    for level in SORTED_LEVELS.iter() {
        if anger >= level.get_minimum_anger() {
            return *level;
        }
    }
    AngerLevel::Calm
}

/// The private `SORTED_LEVELS` table `byAnger` scans (exposed read-only for tests).
pub fn sorted_levels() -> [AngerLevel; 3] {
    *SORTED_LEVELS
}
