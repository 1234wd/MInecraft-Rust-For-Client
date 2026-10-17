//! Port of: net/minecraft/nbt/NumericTag.java
//! Java class(es): net.minecraft.nbt.NumericTag
//! Status: PORTED
//!
//! Session 21. The six abstract accessors, `box()`, and the eight `asX()` overrides.
//! Verified against the `PRIM` rows of `_porting/test-data/nbt_probe.txt` by
//! `_porting/tests/session19_g.rs`, which checks all six conversions, `toString`,
//! `hashCode`, `asBoolean`, the boxed class name and `sizeInBytes` for 92 values.
//!
//! # THE NARROWING CONVERSIONS ARE NOT CASTS, AND THEY DISAGREE WITH EACH OTHER
//!
//! Each of the six numeric kinds spells its own `byteValue`/`shortValue`/... and they are
//! *not* a consistent family. The table below is from the Java source, and every row is
//! checked against the jar:
//!
//! | | `intValue()` | `longValue()` | `shortValue()` | `byteValue()` |
//! |---|---|---|---|---|
//! | `ByteTag`   | widen | widen | widen | `value` |
//! | `ShortTag`  | widen | widen | `value` | `(byte)(v & 0xFF)` |
//! | `IntTag`    | `value` | widen | `(short)(v & 65535)` | `(byte)(v & 0xFF)` |
//! | `LongTag`   | `(int)(v & -1L)` | `value` | `(short)(v & 65535L)` | `(byte)(v & 255L)` |
//! | `FloatTag`  | `Mth.floor(v)` | **`(long) v`** | `(short)(Mth.floor(v) & 65535)` | `(byte)(Mth.floor(v) & 0xFF)` |
//! | `DoubleTag` | `Mth.floor(v)` | **`(long) Math.floor(v)`** | `(short)(Mth.floor(v) & 65535)` | `(byte)(Mth.floor(v) & 0xFF)` |
//!
//! Three of those deserve emphasis, because all three are easy to "tidy" into something
//! that is wrong:
//!
//! * **`FloatTag.longValue()` truncates toward zero; `DoubleTag.longValue()` floors.**
//!   `(long) -1.5F` is `-1`, `(long) Math.floor(-1.5)` is `-2`. Same method name, opposite
//!   answers on negatives, in sibling classes. It is not a decompiler artifact -- the two
//!   source lines are genuinely different -- and a port that routes both through one helper
//!   silently changes world data wherever a negative float is read as a long.
//! * **`LongTag.intValue()` is `(int)(this.value & -1L)`.** `& -1L` is the identity, so this
//!   is `(int) value`: a bit truncation, not a clamp. Reproduced with the mask written out,
//!   because deleting it would make the next reader wonder whether the original clamped.
//! * **The masks before the narrowing casts are also identities.** `(byte)(v & 0xFF)`
//!   equals `(byte) v` for every `v`, because the cast keeps the low 8 bits regardless.
//!   They are kept verbatim rather than simplified: this file is read against the Java
//!   source, and "the mask is redundant" is a claim the reader should be able to check
//!   without trusting me.
//!
//! `Mth.floor(f)` is not `f.floor()`: it returns an `i32` and saturates, so
//! `Mth.floor(-INFINITY)` is `Integer.MIN_VALUE` and `Mth.floor(NaN)` is `0`. The oracle
//! pins both (`PRIM D:fff0000000000000` -> `intValue` `-2147483648`).
//!
//! # `box()` IS OBSERVABLE BEYOND ITS VALUE
//!
//! `box()` returns a boxed `Number`, and the oracle records
//! `box().getClass().getSimpleName()` -- `Byte`, `Short`, `Integer`, `Long`, `Float`,
//! `Double`. Rust has no boxed numeric hierarchy, so [`NumericTag::box_class_simple_name`]
//! carries the name and [`NumericTag::box_to_string`] carries `String.valueOf(box())`.
//! Keeping the class name matters because `Integer` and `Long` print the same digits and
//! hash differently, and a future `NbtOps` will branch on the boxed type.

/// Port of the `sealed interface net.minecraft.nbt.NumericTag`.
///
/// The `asX()` defaults live on [`crate::net::minecraft::nbt::Tag::Tag`]; this trait is the
/// per-kind conversion surface they call into.
pub trait NumericTag {
    /// Port of `NumericTag#byteValue()`.
    fn byte_value(&self) -> i8;

    /// Port of `NumericTag#shortValue()`.
    fn short_value(&self) -> i16;

    /// Port of `NumericTag#intValue()`.
    fn int_value(&self) -> i32;

    /// Port of `NumericTag#longValue()`.
    fn long_value(&self) -> i64;

    /// Port of `NumericTag#floatValue()`.
    fn float_value(&self) -> f32;

    /// Port of `NumericTag#doubleValue()`.
    fn double_value(&self) -> f64;

    /// `box().getClass().getSimpleName()` -- `Byte`, `Short`, `Integer`, `Long`, `Float` or
    /// `Double`.
    ///
    /// Stands in for the part of `box()` that is not the numeric value. See the module docs.
    fn box_class_simple_name(&self) -> &'static str;

    /// `String.valueOf(box())`.
    ///
    /// For the integral kinds this is the decimal digits; for `FloatTag` and `DoubleTag` it
    /// is `Float.toString` / `Double.toString`, which is why it is not just `to_string()`
    /// on the Rust primitive.
    fn box_to_string(&self) -> String;

    /// Port of `NumericTag#asBoolean()`.
    ///
    /// Overrides `Tag#asBoolean()` to skip the `asNumber` hop. Same answer; written out
    /// because the override exists.
    fn as_boolean(&self) -> bool {
        self.byte_value() != 0
    }
}
