//! Port of: net/minecraft/client/data/models/model/TextureSlot.java
//! Java class(es): net.minecraft.client.data.models.model.TextureSlot
//! Status: VERIFIED
//!
//! Session 22. `public final class TextureSlot`: the named texture variables of a block/item model
//! (`#all`, `#side`, `#top` ...), each with an optional parent slot it falls back to -- `NORTH` falls
//! back to `SIDE`, which falls back to `ALL`. 45 constants, an id, a parent and `toString` `"#" + id`.
//!
//! # `create` is PRIVATE in the jar
//!
//! The decompiled source marks both `create` overloads `public` with the comment "Access widened
//! by fabric-data-generation-api-v1 to accessible" -- that is Fabric's view, not Mojang's bytecode.
//! In the jar they are `private static` (the probe records the modifiers by reflection), so here
//! they are private `const fn`s used only to build the constants, as in Java.
//!
//! # Identity, not value
//!
//! No `equals` override: a fresh `create("all")` is NOT `ALL` (probed). The constants are
//! `static`s -- one address each, which is what a parent reference points at -- and the port
//! derives no `PartialEq`; compare slots with `std::ptr::eq`, as Java's `==` does.
//!
//! The 45 constants below were generated from the probe's reflection rows and then compared, name
//! by name, with the `create(...)` calls in the decompiled source: the jar and the source agree on
//! every id and parent. They are verified by the `TEXSLOT` and `TEXSLOTNEW` groups of
//! `small_probe.txt` (`tests/session22.rs`), which list the class's static `TextureSlot` fields BY
//! REFLECTION -- id, parent id and `toString` of every one -- so a constant added to or dropped from
//! the jar fails the test.

/// Port of `net.minecraft.client.data.models.model.TextureSlot`.
#[derive(Debug)]
pub struct TextureSlot {
    id: &'static str,
    parent: Option<&'static TextureSlot>,
}

impl TextureSlot {
    /// Port of `private static TextureSlot create(String id)`: no parent.
    const fn create(id: &'static str) -> TextureSlot {
        TextureSlot { id, parent: None }
    }

    /// Port of `private static TextureSlot create(String id, TextureSlot parent)`.
    const fn create_with_parent(id: &'static str, parent: &'static TextureSlot) -> TextureSlot {
        TextureSlot {
            id,
            parent: Some(parent),
        }
    }

    /// Port of `getId()`.
    pub const fn get_id(&self) -> &'static str {
        self.id
    }

    /// Port of `getParent()`. `None` is a Java `null`.
    pub const fn get_parent(&self) -> Option<&'static TextureSlot> {
        self.parent
    }
}

impl std::fmt::Display for TextureSlot {
    /// Port of `toString()`: `"#" + this.id`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.id)
    }
}

/// Port of `TextureSlot.ALL`.
pub static ALL: TextureSlot = TextureSlot::create("all");
/// Port of `TextureSlot.TEXTURE`.
pub static TEXTURE: TextureSlot = TextureSlot::create_with_parent("texture", &ALL);
/// Port of `TextureSlot.PARTICLE`.
pub static PARTICLE: TextureSlot = TextureSlot::create_with_parent("particle", &TEXTURE);
/// Port of `TextureSlot.END`.
pub static END: TextureSlot = TextureSlot::create_with_parent("end", &ALL);
/// Port of `TextureSlot.BOTTOM`.
pub static BOTTOM: TextureSlot = TextureSlot::create_with_parent("bottom", &END);
/// Port of `TextureSlot.TOP`.
pub static TOP: TextureSlot = TextureSlot::create_with_parent("top", &END);
/// Port of `TextureSlot.FRONT`.
pub static FRONT: TextureSlot = TextureSlot::create_with_parent("front", &ALL);
/// Port of `TextureSlot.BACK`.
pub static BACK: TextureSlot = TextureSlot::create_with_parent("back", &ALL);
/// Port of `TextureSlot.SIDE`.
pub static SIDE: TextureSlot = TextureSlot::create_with_parent("side", &ALL);
/// Port of `TextureSlot.NORTH`.
pub static NORTH: TextureSlot = TextureSlot::create_with_parent("north", &SIDE);
/// Port of `TextureSlot.SOUTH`.
pub static SOUTH: TextureSlot = TextureSlot::create_with_parent("south", &SIDE);
/// Port of `TextureSlot.EAST`.
pub static EAST: TextureSlot = TextureSlot::create_with_parent("east", &SIDE);
/// Port of `TextureSlot.WEST`.
pub static WEST: TextureSlot = TextureSlot::create_with_parent("west", &SIDE);
/// Port of `TextureSlot.UP`.
pub static UP: TextureSlot = TextureSlot::create("up");
/// Port of `TextureSlot.DOWN`.
pub static DOWN: TextureSlot = TextureSlot::create("down");
/// Port of `TextureSlot.CROSS`.
pub static CROSS: TextureSlot = TextureSlot::create("cross");
/// Port of `TextureSlot.CROSS_EMISSIVE`.
pub static CROSS_EMISSIVE: TextureSlot = TextureSlot::create("cross_emissive");
/// Port of `TextureSlot.PLANT`.
pub static PLANT: TextureSlot = TextureSlot::create("plant");
/// Port of `TextureSlot.WALL`.
pub static WALL: TextureSlot = TextureSlot::create_with_parent("wall", &ALL);
/// Port of `TextureSlot.RAIL`.
pub static RAIL: TextureSlot = TextureSlot::create("rail");
/// Port of `TextureSlot.WOOL`.
pub static WOOL: TextureSlot = TextureSlot::create("wool");
/// Port of `TextureSlot.PATTERN`.
pub static PATTERN: TextureSlot = TextureSlot::create("pattern");
/// Port of `TextureSlot.PANE`.
pub static PANE: TextureSlot = TextureSlot::create("pane");
/// Port of `TextureSlot.EDGE`.
pub static EDGE: TextureSlot = TextureSlot::create("edge");
/// Port of `TextureSlot.FAN`.
pub static FAN: TextureSlot = TextureSlot::create("fan");
/// Port of `TextureSlot.STEM`.
pub static STEM: TextureSlot = TextureSlot::create("stem");
/// Port of `TextureSlot.UPPER_STEM`.
pub static UPPER_STEM: TextureSlot = TextureSlot::create("upperstem");
/// Port of `TextureSlot.CROP`.
pub static CROP: TextureSlot = TextureSlot::create("crop");
/// Port of `TextureSlot.DIRT`.
pub static DIRT: TextureSlot = TextureSlot::create("dirt");
/// Port of `TextureSlot.FIRE`.
pub static FIRE: TextureSlot = TextureSlot::create("fire");
/// Port of `TextureSlot.LANTERN`.
pub static LANTERN: TextureSlot = TextureSlot::create("lantern");
/// Port of `TextureSlot.PLATFORM`.
pub static PLATFORM: TextureSlot = TextureSlot::create("platform");
/// Port of `TextureSlot.UNSTICKY`.
pub static UNSTICKY: TextureSlot = TextureSlot::create("unsticky");
/// Port of `TextureSlot.TORCH`.
pub static TORCH: TextureSlot = TextureSlot::create("torch");
/// Port of `TextureSlot.LAYER0`.
pub static LAYER0: TextureSlot = TextureSlot::create("layer0");
/// Port of `TextureSlot.LAYER1`.
pub static LAYER1: TextureSlot = TextureSlot::create("layer1");
/// Port of `TextureSlot.LAYER2`.
pub static LAYER2: TextureSlot = TextureSlot::create("layer2");
/// Port of `TextureSlot.LIT_LOG`.
pub static LIT_LOG: TextureSlot = TextureSlot::create("lit_log");
/// Port of `TextureSlot.CANDLE`.
pub static CANDLE: TextureSlot = TextureSlot::create("candle");
/// Port of `TextureSlot.INSIDE`.
pub static INSIDE: TextureSlot = TextureSlot::create("inside");
/// Port of `TextureSlot.CONTENT`.
pub static CONTENT: TextureSlot = TextureSlot::create("content");
/// Port of `TextureSlot.INNER_TOP`.
pub static INNER_TOP: TextureSlot = TextureSlot::create("inner_top");
/// Port of `TextureSlot.FLOWERBED`.
pub static FLOWERBED: TextureSlot = TextureSlot::create("flowerbed");
/// Port of `TextureSlot.TENTACLES`.
pub static TENTACLES: TextureSlot = TextureSlot::create("tentacles");
/// Port of `TextureSlot.BARS`.
pub static BARS: TextureSlot = TextureSlot::create("bars");
