// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `com.mojang.serialization.DataResult`.
//
// This is on NO parity-tested path: the only reason the DFU Codec API has to
// compile is the `CODEC` static fields on Xoroshiro128PlusPlus /
// XoroshiroRandomSource, which the oracle never reads. `fixedSize` below is a
// faithful transcription of net/minecraft/util/Util.java#fixedSize so the class
// initialisers behave the same, but the encode/decode results are never used.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package com.mojang.serialization;

import java.util.Arrays;
import java.util.function.Function;
import java.util.function.Supplier;

public interface DataResult<T> {
	static <T> DataResult<T> success(final T value) {
		return new DataResult<>() {
			@Override
			public <R> DataResult<R> map(final Function<? super T, ? extends R> map) {
				return DataResult.success(map.apply(value));
			}

			@Override
			public T getOrThrow(boolean partial) {
				return value;
			}
		};
	}

	static <T> DataResult<T> error(final Supplier<String> message) {
		return new DataResult<>() {
			@Override
			public <R> DataResult<R> map(final Function<? super T, ? extends R> map) {
				return DataResult.error(message);
			}

			@Override
			public T getOrThrow(boolean partial) {
				throw new IllegalStateException(message.get());
			}
		};
	}

	static <T> DataResult<T> error(final Supplier<String> message, final T partialResult) {
		return error(message);
	}

	<R> DataResult<R> map(Function<? super T, ? extends R> map);

	T getOrThrow(boolean partial);
}