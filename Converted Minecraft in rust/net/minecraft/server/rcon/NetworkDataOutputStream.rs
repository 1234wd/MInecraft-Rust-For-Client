//! Port of: net/minecraft/server/rcon/NetworkDataOutputStream.java
//! Java class(es): net.minecraft.server.rcon.NetworkDataOutputStream
//! Status: VERIFIED
//!
//! Session 22. `public class NetworkDataOutputStream`: the packet writer of the GameSpy4 query
//! protocol (`rcon/thread/QueryThreadGs4`) -- a `DataOutputStream` over a `ByteArrayOutputStream`,
//! writing multi-byte values LITTLE-endian.
//!
//! # Byte order
//!
//! `DataOutputStream` is big-endian, so every multi-byte write first reverses the value:
//! `writeShort(Short.reverseBytes(v))`, `writeInt(Integer.reverseBytes(v))`, and
//! `writeFloat` writes `Integer.reverseBytes(Float.floatToIntBits(v))` -- `floatToIntBits`, so every
//! NaN goes out as the canonical `0x7fc00000` (`00 00 c0 7f` on the wire). The net effect is
//! little-endian, which is `to_le_bytes` here.
//!
//! * `write(int)` writes the LOW byte only (`300` becomes `0x2c`).
//! * `writeString(s)` writes `s.getBytes(UTF_8)` and then a single `0` terminator. A Java string
//!   with an unpaired surrogate would encode it as `?`; a Rust `&str` cannot hold one, so that case
//!   has no counterpart.
//! * `reset()` empties the buffer; `toByteArray()` copies it.
//!
//! # No `IOException`
//!
//! The Java methods declare `throws IOException`, but a `ByteArrayOutputStream` never fails, so the
//! port's writes are infallible. The constructor's `size` is only the buffer's initial capacity.
//!
//! Verified by the `NETOUT` group of `small_probe.txt` (`tests/session22.rs`): a fourteen-step
//! script (negative shorts and ints, `1.5`, NaN, `-0.0`, out-of-range bytes, empty and non-ASCII
//! strings, a reset), the whole buffer after every step.

/// Port of `net.minecraft.server.rcon.NetworkDataOutputStream`.
#[derive(Debug, Clone, Default)]
pub struct NetworkDataOutputStream {
    output_stream: Vec<u8>,
}

impl NetworkDataOutputStream {
    /// Port of `new NetworkDataOutputStream(int size)`: `size` is the initial capacity.
    pub fn new(size: i32) -> Self {
        NetworkDataOutputStream {
            output_stream: Vec::with_capacity(size.max(0) as usize),
        }
    }

    /// Port of `writeBytes(byte[] data)`.
    pub fn write_bytes(&mut self, data: &[u8]) {
        self.output_stream.extend_from_slice(data);
    }

    /// Port of `writeString(String data)`: UTF-8 bytes, then a `0`.
    pub fn write_string(&mut self, data: &str) {
        self.output_stream.extend_from_slice(data.as_bytes());
        self.output_stream.push(0);
    }

    /// Port of `write(int data)`: the low byte.
    pub fn write(&mut self, data: i32) {
        self.output_stream.push(data as u8);
    }

    /// Port of `writeShort(short data)`: little-endian.
    pub fn write_short(&mut self, data: i16) {
        self.output_stream.extend_from_slice(&data.to_le_bytes());
    }

    /// Port of `writeInt(int data)`: little-endian.
    pub fn write_int(&mut self, data: i32) {
        self.output_stream.extend_from_slice(&data.to_le_bytes());
    }

    /// Port of `writeFloat(float data)`: `floatToIntBits` (canonical NaN), little-endian.
    pub fn write_float(&mut self, data: f32) {
        let bits = if data.is_nan() {
            0x7fc0_0000u32
        } else {
            data.to_bits()
        };
        self.output_stream.extend_from_slice(&bits.to_le_bytes());
    }

    /// Port of `toByteArray()`.
    pub fn to_byte_array(&self) -> Vec<u8> {
        self.output_stream.clone()
    }

    /// Port of `reset()`.
    pub fn reset(&mut self) {
        self.output_stream.clear();
    }
}
