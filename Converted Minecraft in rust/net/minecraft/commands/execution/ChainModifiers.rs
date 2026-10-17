//! Port of: net/minecraft/commands/execution/ChainModifiers.java
//! Java class(es): net.minecraft.commands.execution.ChainModifiers
//! Status: VERIFIED
//!
//! Session 22. `public record ChainModifiers(byte flags)`: a two-bit flag set for a command
//! execution chain (`FLAG_FORKED = 1`, `FLAG_IS_RETURN = 2`) with a `DEFAULT` of no flags.
//!
//! # `setFlag` widens to `int`, compares, then narrows
//!
//! ```java
//! int newFlags = this.flags | flag;
//! return newFlags != this.flags ? new ChainModifiers((byte) newFlags) : this;
//! ```
//!
//! Both operands are sign-extended to `int` before the `|`, and the comparison happens at `int`
//! width, so `flags = -128` (`0x80`) with `FLAG_FORKED` gives `-127`, a new instance. When the flag
//! is already set Java returns `this`, the same object; here it returns an equal copy, because a
//! `Copy` value has no identity to preserve.
//!
//! # Record members
//!
//! `equals` is the derived one (a single `byte`), `hashCode` is [`ChainModifiers::java_hash_code`]
//! (`Byte.hashCode`, i.e. the value), `toString` is `ChainModifiers[flags=<n>]`.
//!
//! Verified by the `REC`, `RECEQ`, `CHAIN` and `CHAINDEF` groups of `small_probe.txt`
//! (`tests/session22.rs`), including the negative and all-bits flag values.

/// Port of `net.minecraft.commands.execution.ChainModifiers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChainModifiers {
    flags: i8,
}

impl ChainModifiers {
    /// Port of `ChainModifiers.DEFAULT` = `new ChainModifiers((byte) 0)`.
    pub const DEFAULT: ChainModifiers = ChainModifiers { flags: 0 };

    /// Port of `private static final byte FLAG_FORKED = 1`.
    const FLAG_FORKED: i8 = 1;
    /// Port of `private static final byte FLAG_IS_RETURN = 2`.
    const FLAG_IS_RETURN: i8 = 2;

    /// Port of the canonical constructor.
    pub const fn new(flags: i8) -> Self {
        ChainModifiers { flags }
    }

    /// Port of the accessor `flags()`.
    pub const fn flags(&self) -> i8 {
        self.flags
    }

    /// Port of `private ChainModifiers setFlag(byte flag)`.
    fn set_flag(&self, flag: i8) -> ChainModifiers {
        let new_flags = (self.flags as i32) | (flag as i32);
        if new_flags != self.flags as i32 {
            ChainModifiers::new(new_flags as i8)
        } else {
            *self
        }
    }

    /// Port of `isForked()`: `(this.flags & 1) != 0`.
    pub const fn is_forked(&self) -> bool {
        (self.flags as i32 & Self::FLAG_FORKED as i32) != 0
    }

    /// Port of `setForked()`.
    pub fn set_forked(&self) -> ChainModifiers {
        self.set_flag(Self::FLAG_FORKED)
    }

    /// Port of `isReturn()`: `(this.flags & 2) != 0`.
    pub const fn is_return(&self) -> bool {
        (self.flags as i32 & Self::FLAG_IS_RETURN as i32) != 0
    }

    /// Port of `setReturn()`.
    pub fn set_return(&self) -> ChainModifiers {
        self.set_flag(Self::FLAG_IS_RETURN)
    }

    /// Port of the record's generated `hashCode()`: `Byte.hashCode(flags)`, the value widened.
    pub const fn java_hash_code(&self) -> i32 {
        self.flags as i32
    }
}

impl std::fmt::Display for ChainModifiers {
    /// Port of the record's generated `toString()`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChainModifiers[flags={}]", self.flags)
    }
}
