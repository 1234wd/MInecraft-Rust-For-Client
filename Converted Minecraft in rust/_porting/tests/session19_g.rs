//! Session 19 batch g: NBT core (net/minecraft/nbt). Checks ports against probes of the real jar
//! and the Java source's behaviour.

#[path = "support/consts_dump.rs"]
mod consts_dump;
#[allow(unused_imports)]
use consts_dump::*;

/// This batch is NOT started, and this marker exists so the suite says so.
///
/// A registered `[[test]]` target with no `#[test]` in it reports
/// `test result: ok. 0 passed` -- which reads exactly like a batch that passed. Two of
/// these sat in the suite output for a full session. Cargo.toml already carries a comment
/// about test FILES that are silently not compiled; this is the same bug one step later,
/// and the same fix: make the gap name itself.
///
/// `net/minecraft/nbt` is 36 of 37 files SKELETON (only `mod.rs` is not), so there is no
/// behaviour here to check yet. SNBT also waits on `FloatingDecimal` (OPEN_QUESTIONS; see
/// SESSION_LOG batch 2 Part B).
///
/// Delete this test when the first real NBT core test lands.
#[test]
#[ignore = "batch nbt (NBT core) is unstarted: net/minecraft/nbt is 36/37 SKELETON, nothing to assert yet"]
fn batch_is_unstarted() {
    panic!(
        "batch nbt is unstarted: net/minecraft/nbt is 36/37 SKELETON. \
         This test is #[ignore]d on purpose -- it is a marker, not a failure."
    );
}
