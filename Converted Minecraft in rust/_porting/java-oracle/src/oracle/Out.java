// ============================================================================
// ORACLE HARNESS -- not game code, not part of the Rust mirror.
//
// Emits golden data produced by running the ORIGINAL, UNMODIFIED classes from
// minecraft-decompiled/. The Rust parity tests in _porting/tests/ read these
// files and assert bit-exact equality.
//
// FILE FORMAT (tab-separated; '\r' is tolerated on read):
//
//   #                        comment / blank
//   #fn <name> <argtypes>    start a method group; <argtypes> are space separated.
//                            Rows beneath it are consumed IN ORDER by the Rust test.
//   <argtype>:<val> ... -> <rettype>:<ret>
//
// Value encoding (bit-exact, no decimal rounding anywhere):
//   i32  -> signed decimal          i64  -> signed decimal
//   bool -> true|false              str  -> raw text (no tabs/newlines)
//   f32  -> 0x%08x of Float.floatToRawIntBits(v)
//   f64  -> 0x%016x of Double.doubleToRawLongBits(v)
//   s    -> comma separated decimal ints (IntStream results)
//
// Raw bits are used so that -0.0, subnormals and NaN payloads all survive the
// round trip. The Rust side compares with f32::to_bits() / f64::to_bits().
// ============================================================================
package oracle;

import java.io.IOException;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

public final class Out {
	/**
	 * Buffered mode. Everything is held in memory until {@link #write(Path)}.
	 *
	 * Kept for the three oracles that predate streaming. Fine for their sizes; do NOT
	 * use for anything that emits millions of rows -- see {@link #Out(Path)}.
	 */
	private final List<String> lines = new ArrayList<>();

	/** Streaming mode. Null when buffering. */
	private PrintWriter sink;

	private long count;

	/**
	 * Buffering writer. See {@link #Out(Path)} before reaching for this on a big oracle.
	 */
	public Out() {
	}

	/**
	 * STREAMING writer: rows go straight to disk.
	 *
	 * A golden file can be hundreds of megabytes. Holding that many {@code String}s
	 * at once is what turns a large oracle into an OutOfMemoryError -- and because the
	 * JVM then cannot allocate the exception object either, the failure prints as a bare
	 * `Exception in thread "main"` with no type and no stack trace. Streaming removes
	 * that failure mode entirely: size is bounded by disk, not by heap.
	 */
	public Out(final Path path) throws IOException {
		Files.createDirectories(path.getParent());
		this.sink = new PrintWriter(Files.newBufferedWriter(path, StandardCharsets.UTF_8));
	}

	private void emit(final String line) {
		this.count++;
		if (this.sink != null) {
			this.sink.print(line);
			this.sink.print('\n');
		} else {
			this.lines.add(line);
		}
	}

	public void fn(final String name, final String argTypes, final String retType) {
		this.emit("#fn " + name + " " + argTypes + " -> " + retType);
	}

	public void row(final String args, final String ret) {
		this.emit(args + " -> " + ret);
	}

	public void comment(final String text) {
		this.emit("# " + text);
	}

	public void blank() {
		this.emit("");
	}

	public void write(final Path path) throws IOException {
		if (this.sink != null) {
			// Already streaming to `path`; just close it.
			this.sink.flush();
			this.sink.close();
			this.sink = null;
			System.out.println("wrote " + this.count + " lines -> " + path);
			return;
		}
		Files.createDirectories(path.getParent());
		try (PrintWriter w = new PrintWriter(Files.newBufferedWriter(path, StandardCharsets.UTF_8))) {
			for (String line : this.lines) {
				w.print(line);
				w.print('\n');
			}
		}
		System.out.println("wrote " + this.lines.size() + " lines -> " + path);
	}

	// ---- value encoders -------------------------------------------------
	public static String i32(final int v) {
		return "i32:" + v;
	}

	public static String i64(final long v) {
		return "i64:" + v;
	}

	public static String f32(final float v) {
		return "f32:0x" + String.format("%08x", Integer.toUnsignedLong(Float.floatToRawIntBits(v)));
	}

	public static String f64(final double v) {
		return "f64:0x" + String.format("%016x", Double.doubleToRawLongBits(v));
	}

	public static String b(final boolean v) {
		return "bool:" + v;
	}

	/**
	 * Encodes a Java string.
	 *
	 * The format separates values with a single space, so a value that contains a
	 * space -- or is empty -- would be unparseable. Escape it: `\s` for a space,
	 * `\0` for empty, `\n` for a newline. The Rust reader unescapes.
	 */
	public static String str(final String v) {
		if (v == null) {
			return "str:\\0null";
		}
		if (v.isEmpty()) {
			return "str:\\0";
		}
		return "str:" + v.replace("\\", "\\\\").replace(" ", "\\s").replace("\n", "\\n").replace("\t", "\\t");
	}

	public static String ints(final int... values) {
		StringBuilder sb = new StringBuilder("s:");
		for (int i = 0; i < values.length; i++) {
			if (i > 0) {
				sb.append(',');
			}

			sb.append(values[i]);
		}

		return sb.toString();
	}

	public static String join(final String... parts) {
		return String.join(" ", parts);
	}
}