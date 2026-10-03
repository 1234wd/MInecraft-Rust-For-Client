// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `com.google.common.hash.HashCode` so that the ORIGINAL,
// UNMODIFIED sources in minecraft-decompiled/ can be compiled by
// _porting/java-oracle. Only RandomSupport.seedFromHashOf(String) uses it, and
// only via asBytes(), which is the raw digest -- byte-for-byte what Guava
// returns.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package com.google.common.hash;

public final class HashCode {
	private final byte[] bytes;

	public HashCode(final byte[] bytes) {
		this.bytes = bytes.clone();
	}

	public byte[] asBytes() {
		return this.bytes.clone();
	}

	public int length() {
		return this.bytes.length;
	}
}