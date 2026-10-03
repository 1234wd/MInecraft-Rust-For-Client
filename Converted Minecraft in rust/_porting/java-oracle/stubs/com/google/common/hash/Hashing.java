// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `com.google.common.hash.Hashing`.
//
// Why this is bit-exact: Guava's `Hashing.md5()` is documented and implemented
// as plain RFC-1321 MD5 over the UTF-8 bytes of the input. The JDK's
// `MessageDigest.getInstance("MD5")` is *required* by the Java Cryptography
// Architecture specification to be exactly that algorithm. There is no
// provider-specific freedom here, so the digest is bit-identical to Guava's.
//
// Used by the ported code only via RandomSupport.seedFromHashOf(String), which
// feeds XoroshiroPositionalRandomFactory.fromHashOf(String) -- so this stub IS
// on a parity-tested path.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package com.google.common.hash;

import java.nio.charset.Charset;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

public final class Hashing {
	private Hashing() {
	}

	public static HashFunction md5() {
		return new HashFunction() {
			@Override
			public int hashLength() {
				return 16;
			}

			@Override
			public HashCode hashString(final CharSequence input, final Charset charset) {
				return hashBytes(input.toString().getBytes(charset));
			}

			@Override
			public HashCode hashBytes(final byte[] input) {
				try {
					return new HashCode(MessageDigest.getInstance("MD5").digest(input));
				} catch (NoSuchAlgorithmException e) {
					throw new IllegalStateException("MD5 is required by the JCA specification", e);
				}
			}
		};
	}
}