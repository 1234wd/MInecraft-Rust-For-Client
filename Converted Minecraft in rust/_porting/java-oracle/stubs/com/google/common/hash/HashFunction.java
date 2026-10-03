// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `com.google.common.hash.HashFunction`.
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package com.google.common.hash;

import java.nio.charset.Charset;

public interface HashFunction {
	int hashLength();

	HashCode hashString(CharSequence input, Charset charset);

	HashCode hashBytes(byte[] input);
}