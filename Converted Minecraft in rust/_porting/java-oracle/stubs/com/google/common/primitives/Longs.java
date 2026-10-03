// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `com.google.common.primitives.Longs`.
//
// Why this is bit-exact: Guava documents `Longs.fromBytes(byte...)` as "returns
// a long value that is byte-wise equal to the given bytes", i.e. little-endian.
// This stub is that literal reading. Used by RandomSupport.seedFromHashOf(String),
// which IS on a parity-tested path.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package com.google.common.primitives;

public final class Longs {
	private Longs() {
	}

	public static long fromBytes(final byte... bytes) {
		long out = 0L;
		for (int i = 0; i < 8; i++) {
			out |= (bytes[i] & 0xFFL) << (8 * i);
		}
		return out;
	}
}