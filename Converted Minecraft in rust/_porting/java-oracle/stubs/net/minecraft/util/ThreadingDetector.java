// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `net.minecraft.util.ThreadingDetector.makeThreadingException`.
// Reachable only from the AtomicLong compareAndSet failure branches of
// LegacyRandomSource / ThreadSafeLegacyRandomSource, which cannot fire on a
// single-threaded oracle run. Never invoked by the golden-data generation.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package net.minecraft.util;

public class ThreadingDetector {
	private ThreadingDetector() {
	}

	public static RuntimeException makeThreadingException(final String name, final Thread threadThatFailedToAcquire) {
		return new IllegalStateException("Accessing " + name + " from multiple threads");
	}
}