// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `io.netty.util.internal.ThreadLocalRandom`.
//
// Only RandomSource.createThreadLocalInstance() (the NO-SEED overload) touches
// this. That call is inherently non-deterministic -- it is seeded from the OS --
// so it cannot be parity-tested and is deliberately left out of the golden data.
// Everything reachable from a seeded RandomSource is unaffected by this stub.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package io.netty.util.internal;

public final class ThreadLocalRandom {
	private static final java.util.concurrent.ThreadLocalRandom INSTANCE = java.util.concurrent.ThreadLocalRandom.current();

	private ThreadLocalRandom() {
	}

	public static java.util.concurrent.ThreadLocalRandom current() {
		return INSTANCE;
	}
}