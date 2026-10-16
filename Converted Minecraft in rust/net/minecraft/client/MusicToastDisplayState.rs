//! Port of: net/minecraft/client/MusicToastDisplayState.java
//! Java class(es): net.minecraft.client.MusicToastDisplayState
//! Status: PARTIAL
//!
//! Session 19. Ported: constants, serialized names, both component keys, `renderInPauseScreen`,
//! `renderToast`.
//! `Component` is not ported, so the getters that return the translated `Component` are
//! PORT-BLOCKED. The translation KEY each one is built from is ported as a `*_key()` getter and is
//! checked against the dump, which records the component as `translation{key='...', args=[]}`.
//! `CODEC` is PORT-BLOCKED on DataFixerUpper.
//!
//! `PAUSE`'s key is `options.musicToast.pauseMenu`, not `...pause` -- the serialized name and the
//! translation key were named independently.

/// Port of `net.minecraft.client.MusicToastDisplayState`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MusicToastDisplayState {
    /// `NEVER` -- ordinal 0.
    Never = 0,
    /// `PAUSE` -- ordinal 1.
    Pause = 1,
    /// `PAUSE_AND_TOAST` -- ordinal 2.
    PauseAndToast = 2,
}

/// Java: `MusicToastDisplayState.values()`, declaration order.
pub const VALUES: [MusicToastDisplayState; 3] = [
    MusicToastDisplayState::Never,
    MusicToastDisplayState::Pause,
    MusicToastDisplayState::PauseAndToast,
];

impl MusicToastDisplayState {
    /// Java `Enum#name()`.
    pub const fn name(&self) -> &'static str {
        match self {
            MusicToastDisplayState::Never => "NEVER",
            MusicToastDisplayState::Pause => "PAUSE",
            MusicToastDisplayState::PauseAndToast => "PAUSE_AND_TOAST",
        }
    }

    /// Java `Enum#ordinal()`.
    pub const fn ordinal(&self) -> i32 {
        *self as i32
    }

    /// Port of `getSerializedName()`.
    pub const fn get_serialized_name(&self) -> &'static str {
        match self {
            MusicToastDisplayState::Never => "never",
            MusicToastDisplayState::Pause => "pause",
            MusicToastDisplayState::PauseAndToast => "pause_and_toast",
        }
    }

    /// The key the private `text` component is built from.
    pub const fn text_key(&self) -> &'static str {
        match self {
            MusicToastDisplayState::Never => "options.musicToast.never",
            MusicToastDisplayState::Pause => "options.musicToast.pauseMenu",
            MusicToastDisplayState::PauseAndToast => "options.musicToast.pauseMenuAndToast",
        }
    }

    /// The key the private `tooltip` component is built from (`translationKey + ".tooltip"`).
    pub const fn tooltip_key(&self) -> &'static str {
        match self {
            MusicToastDisplayState::Never => "options.musicToast.never.tooltip",
            MusicToastDisplayState::Pause => "options.musicToast.pauseMenu.tooltip",
            MusicToastDisplayState::PauseAndToast => "options.musicToast.pauseMenuAndToast.tooltip",
        }
    }

    /// Java: `public Component text()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn text(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }

    /// Java: `public Component tooltip()`.
    ///
    /// PORT-BLOCKED on `net.minecraft.network.chat.Component`.
    pub fn tooltip(&self) {
        todo!("PORT-BLOCKED: net.minecraft.network.chat.Component is not ported")
    }

    /// Port of `MusicToastDisplayState#renderInPauseScreen()`.
    pub const fn render_in_pause_screen(&self) -> bool {
        !matches!(self, MusicToastDisplayState::Never)
    }

    /// Port of `MusicToastDisplayState#renderToast()`.
    pub const fn render_toast(&self) -> bool {
        matches!(self, MusicToastDisplayState::PauseAndToast)
    }
}

impl std::fmt::Display for MusicToastDisplayState {
    /// Java's `Enum#toString()`, i.e. `name()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl crate::net::minecraft::util::StringRepresentable::StringRepresentable for MusicToastDisplayState {
    fn get_serialized_name(&self) -> &str {
        MusicToastDisplayState::get_serialized_name(self)
    }
}

static CODEC_BY_NAME: std::sync::LazyLock<
    crate::net::minecraft::util::StringRepresentable::EnumCodec<MusicToastDisplayState>,
> = std::sync::LazyLock::new(|| crate::net::minecraft::util::StringRepresentable::from_enum(&VALUES));

/// The name lookup behind `CODEC = StringRepresentable.fromEnum(values)` (the field's static type
/// is a bare `Codec`, so vanilla callers only reach it through decoding).
pub fn by_serialized_name(name: &str) -> Option<MusicToastDisplayState> {
    CODEC_BY_NAME.by_name(name)
}

/// Java: `public static final Codec<MusicToastDisplayState> CODEC`.
///
/// PORT-BLOCKED on `com.mojang.serialization.Codec`.
pub fn CODEC() {
    todo!("PORT-BLOCKED: com.mojang.serialization.Codec is not ported")
}
