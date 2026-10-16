//! Port of: net/minecraft/world/level/block/state/properties/TestBlockMode.java
//! Java class(es): net.minecraft.world.level.block.state.properties.TestBlockMode
//! Status: PARTIAL
//!
//! Session 19. Four constants carrying an `int id`, a `String name` and two `Component`s.
//! Ordinals and the `id` and `name` fields of every constant are checked against the jar
//! (`session19_a.rs`). No `toString()` override: `Display` is `name()` (`START`).
//!
//! Ported: `getSerializedName`, the private `id` field ([`TestBlockMode::id`]) and the private
//! `BY_ID` lookup (`ByIdMap.continuous(..., OutOfBoundsStrategy.ZERO)`: an unknown id yields
//! `START`, the value whose id is 0). `BY_ID` is private in Java and only feeds `STREAM_CODEC`;
//! it is exposed here so the decode half is usable and testable before the codec is.
//!
//! PORT-BLOCKED:
//! * `getDisplayName()` (`"test_block.mode." + name`) and `getDetailedMessage()`
//!   (`"test_block.mode_info." + name`): `net.minecraft.network.chat.Component`.
//! * `CODEC`: DFU `com.mojang.serialization.Codec`.
//! * `STREAM_CODEC`: `ByteBufCodecs.idMapper` over netty `ByteBuf`.

/// Port of `net.minecraft.world.level.block.state.properties.TestBlockMode`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TestBlockMode {
    /// `START(0, "start")` -- ordinal 0.
    Start = 0,
    /// `LOG(1, "log")` -- ordinal 1.
    Log = 1,
    /// `FAIL(2, "fail")` -- ordinal 2.
    Fail = 2,
    /// `ACCEPT(3, "accept")` -- ordinal 3.
    Accept = 3,
}

/// Java: `TestBlockMode.values()`, declaration order.
pub const VALUES: [TestBlockMode; 4] = [
    TestBlockMode::Start,
    TestBlockMode::Log,
    TestBlockMode::Fail,
    TestBlockMode::Accept,
];

impl TestBlockMode {
    /// Java: `name()`.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            TestBlockMode::Start => "START",
            TestBlockMode::Log => "LOG",
            TestBlockMode::Fail => "FAIL",
            TestBlockMode::Accept => "ACCEPT",
        }
    }

    /// Java: `ordinal()`.
    #[inline]
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of the private `id` field (read by `BY_ID` and `STREAM_CODEC` as `mode -> mode.id`).
    ///
    /// Stored separately from the ordinal; they coincide today.
    #[inline]
    pub const fn id(&self) -> i32 {
        match self {
            TestBlockMode::Start => 0,
            TestBlockMode::Log => 1,
            TestBlockMode::Fail => 2,
            TestBlockMode::Accept => 3,
        }
    }

    /// Port of `TestBlockMode#getSerializedName()` -- the private `name` field.
    #[inline]
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            TestBlockMode::Start => "start",
            TestBlockMode::Log => "log",
            TestBlockMode::Fail => "fail",
            TestBlockMode::Accept => "accept",
        }
    }

    /// Java: `public Component getDisplayName()` --
    /// `Component.translatable("test_block.mode." + name)`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn get_display_name(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }

    /// Java: `public Component getDetailedMessage()` --
    /// `Component.translatable("test_block.mode_info." + name)`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn get_detailed_message(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }
}

impl std::fmt::Display for TestBlockMode {
    /// Java's `Enum#toString()` (not overridden), i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for TestBlockMode {
    fn get_serialized_name(&self) -> &str {
        TestBlockMode::get_serialized_name(self)
    }
}

static BY_ID_MAP: std::sync::LazyLock<crate::net::minecraft::util::ByIdMap::Continuous<TestBlockMode>> =
    std::sync::LazyLock::new(|| {
        crate::net::minecraft::util::ByIdMap::continuous(
            TestBlockMode::id,
            &VALUES,
            crate::net::minecraft::util::ByIdMap::OutOfBoundsStrategy::Zero,
        )
    });

/// Java: `private static final IntFunction<TestBlockMode> BY_ID`, applied.
///
/// `OutOfBoundsStrategy.ZERO`: any id outside `0..=3` (negative included) yields `START`.
pub fn BY_ID(id: i32) -> TestBlockMode {
    BY_ID_MAP.apply(id)
}

// ============================================================================
// PORT-BLOCKED
// ============================================================================

/// Java: `public static final Codec<TestBlockMode> CODEC = StringRepresentable.fromEnum(TestBlockMode::values)`.
///
/// `com.mojang.serialization.Codec` is not ported. See DESIGN_DECISIONS.md (#dfu-proposal).
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}

/// Java: `public static final StreamCodec<ByteBuf, TestBlockMode> STREAM_CODEC =
/// ByteBufCodecs.idMapper(BY_ID, mode -> mode.id)`.
///
/// `net.minecraft.network.codec.ByteBufCodecs` / `io.netty.buffer.ByteBuf` are not ported.
pub fn STREAM_CODEC() {
    todo!("PORT-BLOCKED: net.minecraft.network.codec.ByteBufCodecs is not ported")
}
