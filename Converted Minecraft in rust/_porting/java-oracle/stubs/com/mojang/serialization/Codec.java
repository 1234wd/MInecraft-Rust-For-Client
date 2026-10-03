// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `com.mojang.serialization.Codec`.
//
// ON NO PARITY-TESTED PATH. The stub exists purely so that the `CODEC` static
// initialisers on Xoroshiro128PlusPlus and XoroshiroRandomSource compile. Every
// method returns a fresh empty Codec; none of them is ever invoked by the oracle.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package com.mojang.serialization;

import java.util.function.Function;
import java.util.stream.IntStream;
import java.util.stream.LongStream;

public class Codec<A> {
	public static final Codec<Long> LONG = new Codec<>();
	public static final Codec<Integer> INT = new Codec<>();
	public static final Codec<String> STRING = new Codec<>();
	public static final Codec<LongStream> LONG_STREAM = new Codec<>();
	public static final Codec<IntStream> INT_STREAM = new Codec<>();

	public <B> Codec<B> comapFlatMap(final Function<A, DataResult<B>> to, final Function<B, A> from) {
		return new Codec<>();
	}

	public <B> Codec<B> xmap(final Function<A, B> to, final Function<B, A> from) {
		return new Codec<>();
	}
}