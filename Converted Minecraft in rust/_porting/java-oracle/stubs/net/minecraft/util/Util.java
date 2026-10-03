// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `net.minecraft.util.Util`, reduced to the two members the ported
// batch touches. `make` is a verbatim transcription of
// minecraft-decompiled/net/minecraft/util/Util.java#make(Consumer) and
// `fixedSize` is a verbatim transcription of Util#fixedSize(IntStream,int).
//
// ON A PARITY-TESTED PATH -- with a caveat that is recorded in
// OPEN_QUESTIONS.md: Util.make(T, Consumer) initialises Mth's 65536-entry SIN
// table via a lambda, and our stub uses the identical one-line body, so the table
// contents are unaffected. Util.fixedSize is only reachable through the DFU
// Codec machinery, which this oracle never invokes.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package net.minecraft.util;

import com.mojang.serialization.DataResult;
import java.util.Arrays;
import java.util.List;
import java.util.function.Consumer;
import java.util.function.Supplier;
import java.util.stream.IntStream;
import java.util.stream.LongStream;

public class Util {
	private Util() {
	}

	public static <T> T make(final T t, final Consumer<? super T> consumer) {
		consumer.accept(t);
		return t;
	}

	public static DataResult<int[]> fixedSize(final IntStream stream, final int size) {
		int[] ints = stream.limit(size + 1).toArray();
		if (ints.length != size) {
			Supplier<String> message = () -> "Input is not a list of " + size + " ints";
			return ints.length >= size ? DataResult.error(message, Arrays.copyOf(ints, size)) : DataResult.error(message);
		} else {
			return DataResult.success(ints);
		}
	}

	public static DataResult<long[]> fixedSize(final LongStream stream, final int size) {
		long[] longs = stream.limit(size + 1).toArray();
		if (longs.length != size) {
			Supplier<String> message = () -> "Input is not a list of " + size + " longs";
			return longs.length >= size ? DataResult.error(message, Arrays.copyOf(longs, size)) : DataResult.error(message);
		} else {
			return DataResult.success(longs);
		}
	}

	public static <T> DataResult<List<T>> fixedSize(final List<T> list, final int size) {
		if (list.size() != size) {
			Supplier<String> message = () -> "Input is not a list of " + size + " elements";
			return list.size() >= size ? DataResult.error(message, list.subList(0, size)) : DataResult.error(message);
		} else {
			return DataResult.success(list);
		}
	}
}