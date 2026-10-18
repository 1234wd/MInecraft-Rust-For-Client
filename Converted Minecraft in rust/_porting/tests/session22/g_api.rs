//! Sub-batch F: declaration-only interfaces. Groups `API`, `APISUPER`.
//!
//! These have no behaviour to run, so what is checked is their SHAPE, from two sides:
//!
//! * [`EXPECTED`] is the Java signature of every method of every ported trait, written from the
//!   Rust trait (`fn get_count(&self) -> i32` is `getCount()int`). The test compares it with the
//!   methods the probe found on the jar's class by reflection -- name, generic parameter and return
//!   types, and that each is abstract. A method the jar has and the trait lacks, or the reverse,
//!   fails here.
//! * [`implementations`] implements every trait with exactly those methods. It is never called; it
//!   exists so that a method added to or removed from a trait file fails to COMPILE, which keeps
//!   [`EXPECTED`] honest about the trait it describes.

use crate::small_probe::*;

/// (simple class name, Java signatures, supertypes, interface|class)
const EXPECTED: &[(&str, &[&str], &str, &str)] = &[
    ("CompiledRenderPipeline", &["isValid()boolean"], "-", "interface"),
    ("ResettableOptionWidget", &["resetValue()void"], "-", "interface"),
    ("HasDifficultyReaction", &["onDifficultyChanged()void"], "-", "interface"),
    ("HasGamemasterPermissionReaction", &["onGamemasterPermissionChanged(boolean)void"], "-", "interface"),
    ("TickableTexture", &["tick()void"], "-", "interface"),
    ("AmbientSoundHandler", &["tick()void"], "-", "interface"),
    ("Destroyable", &["destroy()void"], "-", "interface"),
    ("SlotSelectTime", &["currentIndex()int"], "-", "interface"),
    ("ModelDebugName", &["debugName()java.lang.String"], "-", "interface"),
    ("StreamMemberEncoder", &["encode(T,O)void"], "-", "interface"),
    ("ProfiledDuration", &["finish(boolean)void"], "-", "interface"),
    ("SampledFloat", &["sample(net.minecraft.util.RandomSource)float"], "-", "interface"),
    ("GpuFence", &["awaitCompletion(long)boolean", "close()void"], "java.lang.AutoCloseable", "interface"),
    ("WindowEventHandler", &["cursorEntered()void", "framebufferSizeChanged()void", "resizeGui()void"], "-", "interface"),
    ("Dumpable", &["dumpContents(net.minecraft.resources.Identifier,java.nio.file.Path)void"], "-", "interface"),
    ("Position", &["x()double", "y()double", "z()double"], "-", "interface"),
    ("TestPosFinder", &["findTestPos()java.util.stream.Stream<net.minecraft.core.BlockPos>"], "-", "interface"),
    (
        "GizmoProperties",
        &[
            "fadeOut()net.minecraft.gizmos.GizmoProperties",
            "persistForMillis(int)net.minecraft.gizmos.GizmoProperties",
            "setAlwaysOnTop()net.minecraft.gizmos.GizmoProperties",
        ],
        "-",
        "interface",
    ),
    ("SampleLogger", &["logFullSample(long[])void", "logPartialSample(long,int)void", "logSample(long)void"], "-", "interface"),
    ("ContainerData", &["get(int)int", "getCount()int", "set(int,int)void"], "-", "interface"),
    (
        "ChunkStatusUpdateListener",
        &["onChunkStatusChange(net.minecraft.world.level.ChunkPos,net.minecraft.server.level.FullChunkStatus)void"],
        "-",
        "interface",
    ),
    ("GpuQuery", &["close()void", "getValue()java.util.OptionalLong"], "java.lang.AutoCloseable", "interface"),
    (
        "ResolvableModel",
        &["resolveDependencies(net.minecraft.client.resources.model.ResolvableModel$Resolver)void"],
        "-",
        "interface",
    ),
    ("ResolvableModel$Resolver", &["markDependency(net.minecraft.resources.Identifier)void"], "-", "interface"),
    (
        "ServerInfo",
        &["getMaxPlayers()int", "getMotd()java.lang.String", "getPlayerCount()int", "getServerVersion()java.lang.String"],
        "-",
        "interface",
    ),
    ("SampleStorage", &["capacity()int", "get(int)long", "get(int,int)long", "reset()void", "size()int"], "-", "interface"),
    (
        "TickingBlockEntity",
        &["getPos()net.minecraft.core.BlockPos", "getType()java.lang.String", "isRemoved()boolean", "tick()void"],
        "-",
        "interface",
    ),
    (
        "GpuQueryPool",
        &["close()void", "getValue(int)java.util.OptionalLong", "getValues(int,int)java.util.OptionalLong[]", "size()int"],
        "java.lang.AutoCloseable",
        "interface",
    ),
    (
        "DebugScreenDisplayer",
        &[
            "addLine(java.lang.String)void",
            "addPriorityLine(java.lang.String)void",
            "addToGroup(net.minecraft.resources.Identifier,java.lang.String)void",
            "addToGroup(net.minecraft.resources.Identifier,java.util.Collection<java.lang.String>)void",
        ],
        "-",
        "interface",
    ),
    (
        "TraceCallbacks",
        &[
            "close()void",
            "onCall(int,net.minecraft.resources.Identifier,int)void",
            "onCommand(int,java.lang.String)void",
            "onError(java.lang.String)void",
            "onReturn(int,java.lang.String,int)void",
        ],
        "java.lang.AutoCloseable",
        "interface",
    ),
    (
        "LevelCallback",
        &[
            "onCreated(T)void",
            "onDestroyed(T)void",
            "onSectionChange(T)void",
            "onTickingEnd(T)void",
            "onTickingStart(T)void",
            "onTrackingEnd(T)void",
            "onTrackingStart(T)void",
        ],
        "-",
        "interface",
    ),
    (
        "GpuSampler",
        &[
            "close()void",
            "getAddressModeU()com.mojang.blaze3d.textures.AddressMode",
            "getAddressModeV()com.mojang.blaze3d.textures.AddressMode",
            "getMagFilter()com.mojang.blaze3d.textures.FilterMode",
            "getMaxAnisotropy()int",
            "getMaxLod()java.util.OptionalDouble",
            "getMinFilter()com.mojang.blaze3d.textures.FilterMode",
        ],
        "java.lang.AutoCloseable",
        "class",
    ),
];

#[test]
fn declaration_only_interfaces_have_the_jars_methods() {
    let api = rows("API");
    let sup = rows("APISUPER");
    assert_eq!(sup.len(), EXPECTED.len(), "the probe and the test cover different classes");
    for (class, sigs, supers, kind) in EXPECTED {
        let mut got: Vec<String> =
            api.iter().filter(|r| r[1] == *class).map(|r| format!("{}\t{}", r[2], r[3])).collect();
        got.sort();
        let mut want: Vec<String> = sigs.iter().map(|s| format!("{s}\tabstract")).collect();
        want.sort();
        assert_eq!(got, want, "{class}: declared methods");
        let s = sup.iter().find(|r| r[1] == *class).unwrap_or_else(|| panic!("no APISUPER row for {class}"));
        assert_eq!((s[2], s[3]), (*supers, *kind), "{class}: supertypes");
    }
}

/// Compile-time half of the check: every trait implemented with exactly its declared methods.
#[allow(dead_code)]
mod implementations {
    use minecraft_rust::com::mojang::blaze3d::buffers::GpuFence::GpuFence;
    use minecraft_rust::com::mojang::blaze3d::pipeline::CompiledRenderPipeline::CompiledRenderPipeline;
    use minecraft_rust::com::mojang::blaze3d::platform::WindowEventHandler::WindowEventHandler;
    use minecraft_rust::com::mojang::blaze3d::systems::GpuQuery::GpuQuery;
    use minecraft_rust::com::mojang::blaze3d::systems::GpuQueryPool::GpuQueryPool;
    use minecraft_rust::com::mojang::blaze3d::textures::AddressMode::AddressMode;
    use minecraft_rust::com::mojang::blaze3d::textures::FilterMode::FilterMode;
    use minecraft_rust::com::mojang::blaze3d::textures::GpuSampler::GpuSampler;
    use minecraft_rust::com::mojang::blaze3d::vulkan::Destroyable::Destroyable;
    use minecraft_rust::net::minecraft::client::gui::components::debug::DebugScreenDisplayer::DebugScreenDisplayer;
    use minecraft_rust::net::minecraft::client::gui::components::ResettableOptionWidget::ResettableOptionWidget;
    use minecraft_rust::net::minecraft::client::gui::screens::options::HasDifficultyReaction::HasDifficultyReaction;
    use minecraft_rust::net::minecraft::client::gui::screens::options::HasGamemasterPermissionReaction::HasGamemasterPermissionReaction;
    use minecraft_rust::net::minecraft::client::gui::screens::recipebook::SlotSelectTime::SlotSelectTime;
    use minecraft_rust::net::minecraft::client::renderer::texture::Dumpable::Dumpable;
    use minecraft_rust::net::minecraft::client::renderer::texture::TickableTexture::TickableTexture;
    use minecraft_rust::net::minecraft::client::resources::model::ModelDebugName::ModelDebugName;
    use minecraft_rust::net::minecraft::client::resources::model::ResolvableModel::{ResolvableModel, Resolver};
    use minecraft_rust::net::minecraft::client::resources::sounds::AmbientSoundHandler::AmbientSoundHandler;
    use minecraft_rust::net::minecraft::commands::execution::TraceCallbacks::TraceCallbacks;
    use minecraft_rust::net::minecraft::core::BlockPos::BlockPos;
    use minecraft_rust::net::minecraft::core::Position::Position;
    use minecraft_rust::net::minecraft::gametest::framework::TestPosFinder::TestPosFinder;
    use minecraft_rust::net::minecraft::gizmos::GizmoProperties::GizmoProperties;
    use minecraft_rust::net::minecraft::network::codec::StreamMemberEncoder::StreamMemberEncoder;
    use minecraft_rust::net::minecraft::resources::Identifier::Identifier;
    use minecraft_rust::net::minecraft::server::level::FullChunkStatus::FullChunkStatus;
    use minecraft_rust::net::minecraft::server::ServerInfo::ServerInfo;
    use minecraft_rust::net::minecraft::util::debugchart::SampleLogger::SampleLogger;
    use minecraft_rust::net::minecraft::util::debugchart::SampleStorage::SampleStorage;
    use minecraft_rust::net::minecraft::util::profiling::jfr::callback::ProfiledDuration::ProfiledDuration;
    use minecraft_rust::net::minecraft::util::valueproviders::SampledFloat::SampledFloat;
    use minecraft_rust::net::minecraft::util::RandomSource::RandomSource;
    use minecraft_rust::net::minecraft::world::inventory::ContainerData::ContainerData;
    use minecraft_rust::net::minecraft::world::level::block::entity::TickingBlockEntity::TickingBlockEntity;
    use minecraft_rust::net::minecraft::world::level::entity::ChunkStatusUpdateListener::ChunkStatusUpdateListener;
    use minecraft_rust::net::minecraft::world::level::entity::LevelCallback::LevelCallback;
    use minecraft_rust::net::minecraft::world::level::ChunkPos::ChunkPos;

    struct X;

    impl CompiledRenderPipeline for X {
        fn is_valid(&self) -> bool {
            true
        }
    }
    impl ResettableOptionWidget for X {
        fn reset_value(&mut self) {}
    }
    impl HasDifficultyReaction for X {
        fn on_difficulty_changed(&mut self) {}
    }
    impl HasGamemasterPermissionReaction for X {
        fn on_gamemaster_permission_changed(&mut self, _has_gamemaster_permission: bool) {}
    }
    impl TickableTexture for X {
        fn tick(&mut self) {}
    }
    impl AmbientSoundHandler for X {
        fn tick(&mut self) {}
    }
    impl Destroyable for X {
        fn destroy(&mut self) {}
    }
    impl SlotSelectTime for X {
        fn current_index(&self) -> i32 {
            0
        }
    }
    impl ModelDebugName for X {
        fn debug_name(&self) -> String {
            String::new()
        }
    }
    impl StreamMemberEncoder<&mut Vec<u8>, u8> for X {
        fn encode(&self, value: u8, output: &mut Vec<u8>) {
            output.push(value);
        }
    }
    impl ProfiledDuration for X {
        fn finish(&mut self, _success: bool) {}
    }
    impl SampledFloat for X {
        fn sample(&self, random: &mut dyn RandomSource) -> f32 {
            random.next_float()
        }
    }
    impl GpuFence for X {
        fn close(&mut self) {}
        fn await_completion(&self, _timeout_ns: i64) -> bool {
            true
        }
    }
    impl WindowEventHandler for X {
        fn framebuffer_size_changed(&mut self) {}
        fn resize_gui(&mut self) {}
        fn cursor_entered(&mut self) {}
    }
    impl Dumpable for X {
        fn dump_contents(&self, _self_id: &Identifier, _dir: &std::path::Path) -> std::io::Result<()> {
            Ok(())
        }
    }
    impl Position for X {
        fn x(&self) -> f64 {
            0.0
        }
        fn y(&self) -> f64 {
            0.0
        }
        fn z(&self) -> f64 {
            0.0
        }
    }
    impl TestPosFinder for X {
        fn find_test_pos(&self) -> Box<dyn Iterator<Item = BlockPos> + '_> {
            Box::new(std::iter::empty())
        }
    }
    impl GizmoProperties for X {
        fn set_always_on_top(&mut self) -> &mut dyn GizmoProperties {
            self
        }
        fn persist_for_millis(&mut self, _milliseconds: i32) -> &mut dyn GizmoProperties {
            self
        }
        fn fade_out(&mut self) -> &mut dyn GizmoProperties {
            self
        }
    }
    impl SampleLogger for X {
        fn log_full_sample(&mut self, _sample: &[i64]) {}
        fn log_sample(&mut self, _sample: i64) {}
        fn log_partial_sample(&mut self, _sample: i64, _dimension: i32) {}
    }
    impl ContainerData for X {
        fn get(&self, _data_id: i32) -> i32 {
            0
        }
        fn set(&mut self, _data_id: i32, _value: i32) {}
        fn get_count(&self) -> i32 {
            0
        }
    }
    impl ChunkStatusUpdateListener for X {
        fn on_chunk_status_change(&mut self, _pos: ChunkPos, _chunk_status: FullChunkStatus) {}
    }
    impl GpuQuery for X {
        fn get_value(&self) -> Option<i64> {
            None
        }
        fn close(&mut self) {}
    }
    impl ResolvableModel for X {
        fn resolve_dependencies(&self, _resolver: &mut dyn Resolver) {}
    }
    impl Resolver for X {
        fn mark_dependency(&mut self, _id: &Identifier) {}
    }
    impl ServerInfo for X {
        fn get_motd(&self) -> String {
            String::new()
        }
        fn get_server_version(&self) -> String {
            String::new()
        }
        fn get_player_count(&self) -> i32 {
            0
        }
        fn get_max_players(&self) -> i32 {
            0
        }
    }
    impl SampleStorage for X {
        fn capacity(&self) -> i32 {
            0
        }
        fn size(&self) -> i32 {
            0
        }
        fn get_i32(&self, _index: i32) -> i64 {
            0
        }
        fn get_i32_i32(&self, _index: i32, _dimension: i32) -> i64 {
            0
        }
        fn reset(&mut self) {}
    }
    impl TickingBlockEntity for X {
        fn tick(&mut self) {}
        fn is_removed(&self) -> bool {
            false
        }
        fn get_pos(&self) -> BlockPos {
            BlockPos::new(0, 0, 0)
        }
        fn get_type(&self) -> String {
            String::new()
        }
    }
    impl GpuQueryPool for X {
        fn size(&self) -> i32 {
            0
        }
        fn get_value(&self, _index: i32) -> Option<i64> {
            None
        }
        fn get_values(&self, _index: i32, _count: i32) -> Vec<Option<i64>> {
            Vec::new()
        }
        fn close(&mut self) {}
    }
    impl DebugScreenDisplayer for X {
        fn add_priority_line(&mut self, _line: &str) {}
        fn add_line(&mut self, _line: &str) {}
        fn add_to_group_collection(&mut self, _group: &Identifier, _lines: &[String]) {}
        fn add_to_group_string(&mut self, _group: &Identifier, _lines: &str) {}
    }
    impl TraceCallbacks for X {
        fn on_command(&mut self, _depth: i32, _command: &str) {}
        fn on_return(&mut self, _depth: i32, _command: &str, _result: i32) {}
        fn on_error(&mut self, _message: &str) {}
        fn on_call(&mut self, _depth: i32, _function: &Identifier, _size: i32) {}
        fn close(&mut self) {}
    }
    impl LevelCallback<u32> for X {
        fn on_created(&mut self, _entity: u32) {}
        fn on_destroyed(&mut self, _entity: u32) {}
        fn on_ticking_start(&mut self, _entity: u32) {}
        fn on_ticking_end(&mut self, _entity: u32) {}
        fn on_tracking_start(&mut self, _entity: u32) {}
        fn on_tracking_end(&mut self, _entity: u32) {}
        fn on_section_change(&mut self, _entity: u32) {}
    }
    impl GpuSampler for X {
        fn get_address_mode_u(&self) -> AddressMode {
            AddressMode::Repeat
        }
        fn get_address_mode_v(&self) -> AddressMode {
            AddressMode::Repeat
        }
        fn get_min_filter(&self) -> FilterMode {
            FilterMode::Nearest
        }
        fn get_mag_filter(&self) -> FilterMode {
            FilterMode::Nearest
        }
        fn get_max_anisotropy(&self) -> i32 {
            1
        }
        fn get_max_lod(&self) -> Option<f64> {
            None
        }
        fn close(&mut self) {}
    }
}
