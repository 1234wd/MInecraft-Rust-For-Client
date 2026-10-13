//! Port of: net/minecraft/world/entity/ai/memory/MemoryStatus.java
//! Java class(es): net.minecraft.world.entity.ai.memory.MemoryStatus
//! Status: VERIFIED
//!
//! Three constants, no fields, no methods. Verified against the jar by `_porting/tests/consts.rs`.
//!
//! # THE ORDINAL IS NOT PERSISTED, BUT THE ORDER IS READ
//!
//! `MemoryModuleType` declares its memory types in this order and the brain scheduler relies on
//! the enum being the small closed set it is. Nothing is written to disk, so this is the least
//! risky enum in the project -- but it is still checked, because a dropped or reordered constant
//! would compile silently and change which memories a brain registers.

/// Port of `net.minecraft.world.entity.ai.memory.MemoryStatus`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemoryStatus {
    /// Port of `MemoryStatus#VALUE_PRESENT` -- ordinal 0.
    ValuePresent = 0,
    /// Port of `MemoryStatus#VALUE_ABSENT` -- ordinal 1.
    ValueAbsent = 1,
    /// Port of `MemoryStatus#REGISTERED` -- ordinal 2.
    Registered = 2,
}

/// Java: `MemoryStatus.values()`.
pub const VALUES: [MemoryStatus; 3] = [
    MemoryStatus::ValuePresent,
    MemoryStatus::ValueAbsent,
    MemoryStatus::Registered,
];