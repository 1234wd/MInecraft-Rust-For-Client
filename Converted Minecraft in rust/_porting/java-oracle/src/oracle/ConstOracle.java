// ============================================================================
// GENERIC CONSTANT DUMP -- not game code.
//
// Builds golden rows for ANY class in the jar by reflecting over it. Built in session 14 so
// that enums, records and constant holders can be verified without hand-writing a golden group
// per class: before this, every structure file needed bespoke emission code; after it, each one
// costs one line in `const-classes.txt` and one comparison in a test.
//
// WHY THIS EXISTS
// ---------------
// Hand-written golden groups are the right tool for LOGIC -- arithmetic, iteration order, hash
// functions. They are the wrong tool for a file that is a list of constants, because the
// emission code costs more than the thing being checked and it gets the FORMAT wrong often
// enough to matter. The format below is deliberately uniform: one row per constant, arguments
// are its name, and every expected value is a `type:value` string.
//
// THE JAR IS STILL GROUND TRUTH
// -----------------------------
// This reads the jar's own class objects via reflection, so a decompiler artifact in
// `minecraft-decompiled/` cannot make a constant "right". That is the whole point: a golden
// constant that came from the SOURCE would only prove the source agrees with itself.
//
// WHAT IS AND IS NOT DUMPED
// -------------------------
// * enum constants: name, ordinal, and every PUBLIC declared field (the `id`/`index`/`serialized
//   name` family that carries real meaning).
// * `static final` fields of primitive or String type. Deliberately NOT object-typed statics:
//   a `static final Codec` needs DataFixerUpper bootstrapped and its value is not a constant we
//   can compare, only the fact that it exists. Those are PORT-BLOCKED in the Rust port instead,
//   which is the honest description.
//
// ORDER
// -----
// `Class.getFields()` has no order guarantee, so rows are SORTED by name before writing. A
// golden whose row order depends on the JVM's reflection order would churn on every JDK, and
// churn in a golden is indistinguishable from a real behaviour change.
package oracle;

import java.io.IOException;
import java.lang.reflect.Field;
import java.lang.reflect.Modifier;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

public final class ConstOracle {

	/** One row: the constant's name, then every value as a `type:value` string. */
	private record Row(String name, List<String> values) {}

	/**
	 * Read the class list and emit one group per class.
	 *
	 * The list comes from the `const.list` system property (a path), falling back to
	 * `const-classes.txt` in the working directory. A FILE rather than a hardcoded array so that
	 * growing the corpus is a one-line edit with no recompile -- which matters because this
	 * stage's whole purpose is to be cheap per class, and a recompile would eat that.
	 */
	public static void emit(Path out, Path listFile) throws Exception {
		List<String> names = readList(listFile);
		emitFrom(out, names);
	}

	/**
	 * Dump every ENUM and every class with comparable `static final` scalars in the whole game,
	 * discovered from `MANIFEST.csv` rather than from a hand-kept list.
	 *
	 * This is what makes the stage pay for itself. A hand-kept list means a class is only ever
	 * verified if someone remembered to add it; walking the manifest means the corpus is the
	 * entire game, and adding a port adds its verification for free.
	 *
	 * Classes are loaded with `initialize = false` so nothing runs a static initialiser merely to
	 * be inspected -- otherwise walking 7,055 classes would bootstrap the registry, load
	 * textures, and take minutes. Actual initialisation happens only for the handful of classes
	 * that turn out to be enums or constant holders, and each is wrapped so one failure cannot
	 * stop the walk.
	 *
	 * Only `net.minecraft.` is walked, and `client` is included deliberately: a client-side enum
	 * still has to be right, and skipping it would leave a hole that grows.
	 */
	public static void emitFromManifest(Path out, Path manifest) throws Exception {
		List<String> names = new ArrayList<>();
		for (String line : Files.readAllLines(manifest, StandardCharsets.UTF_8)) {
			if (line.isEmpty() || line.startsWith("java_path")) {
				continue;
			}
			String jp = line.split(",", 2)[0].trim();
			if (!jp.startsWith("net/minecraft/") || !jp.endsWith(".java")) {
				continue;
			}
			names.add(jp.substring(0, jp.length() - 5).replace('/', '.'));
		}
		List<String> keep = new ArrayList<>();
		int enums = 0, scalars = 0, unloadable = 0, uninspectable = 0;
		for (String fq : names) {
			// `Class.forName(name, false, ...)` succeeding does NOT mean the class is usable:
			// `getFields()` resolves each field's TYPE, and Minecraft has plenty of classes with a
			// field whose type comes from an optional dependency that is not on this classpath
			// (joptsimple was the first). That surfaces as `NoClassDefFoundError`, which is an
			// Error and so is NOT caught by a `catch (Exception)`. It has to be a `Throwable`
			// here, or one unresolvable class aborts the sweep of all 7,055.
			Class<?> c;
			try {
				c = Class.forName(fq, false, ConstOracle.class.getClassLoader());
			} catch (Throwable t) {
				unloadable++;
				continue;
			}
			try {
				if (c.isEnum()) {
					enums++;
					keep.add(fq);
				} else if (hasScalarStatic(c)) {
					scalars++;
					keep.add(fq);
				}
			} catch (Throwable t) {
				uninspectable++;
			}
		}
		System.out.println("    manifest scan: " + names.size() + " classes, " + enums + " enums, "
				+ scalars + " constant holders, " + unloadable + " unloadable, "
				+ uninspectable + " uninspectable");
		emitFrom(out, keep);
	}

	private static boolean hasScalarStatic(Class<?> c) {
		for (Field f : c.getFields()) {
			int m = f.getModifiers();
			if (Modifier.isStatic(m) && Modifier.isFinal(m) && isScalar(f.getType())) {
				return true;
			}
		}
		return false;
	}

	private static void emitFrom(Path out, List<String> names) throws Exception {
		List<String> lines = new ArrayList<>();
		int classes = 0, rows = 0, errors = 0;

		for (String fq : names) {
			Class<?> c;
			try {
				c = Class.forName(fq);
			} catch (Throwable t) {
				// Recorded, not fatal. A class that cannot load (missing optional dependency,
				// or a name typo) must not take the other 200 classes down with it -- and it
				// must be VISIBLE, or a typo looks exactly like an empty group.
				lines.add("#error " + fq + " -> " + t.getClass().getName() + ": " + t.getMessage());
				continue;
			}
			List<Row> rs = describe(c);
			if (rs.isEmpty()) {
				// Not an error in the sweep: a class can pass `hasScalarStatic` and then produce
				// nothing because reading it failed. Skipped silently here, counted in the header,
				// because the alternative -- thousands of `#error` lines for the client sweep --
				// would bury the real ones.
				errors++;
				continue;
			}
			rs.sort(Comparator.comparing(Row::name));
			lines.add("#fn const." + fq + " -> str str");
			for (Row r : rs) {
				// Both columns go through `Out.str`, NOT a raw `"str:" + x`.
			// The golden format is SPACE-separated (`Out.join` is `String.join(" ", ...)` and the
			// Rust loader is `split(' ')`), and `Out.str` is what escapes a space as `\s`, a
			// newline as `\n` and a backslash as `\\`. Hand-rolling the prefix instead produced
			// `str:Please make sure you have up-to-date drivers ...`, which the loader split into
			// five tokens and rejected with "value `make` has no `type:` prefix" -- one String
			// constant containing a space made the whole 1,740-class dump unparseable.
			// The `key=value` composites are escaped as a WHOLE, so an `=` or a space inside a
			// value survives the round trip.
			StringBuilder sb = new StringBuilder();
				for (String v : r.values()) {
					if (sb.length() > 0) {
						sb.append(' ');
					}
					sb.append(Out.str(v));
				}
				lines.add(Out.str(r.name()) + " -> " + sb);
				rows++;
			}
			classes++;
		}

		String header = "# generic constant dump -- GENERATED, do not hand-edit.\n"
				+ "# one row per constant: str:<name> -> str:<type:value> ...\n"
				+ "# classes: " + classes + "  rows: " + rows + "  skipped: " + errors + "\n";
		// .tmp then move, so an interrupted run never leaves a half-written golden behind.
		Path tmp = out.resolveSibling(out.getFileName() + ".tmp");
		Files.writeString(tmp, header + String.join("\n", lines) + "\n", StandardCharsets.UTF_8);
		Files.move(tmp, out, java.nio.file.StandardCopyOption.REPLACE_EXISTING);
		System.out.println("    " + classes + " classes, " + rows + " constant rows");
	}

	private static List<String> readList(Path listFile) throws IOException {
		List<String> out = new ArrayList<>();
		if (!Files.exists(listFile)) {
			return out;
		}
		for (String line : Files.readAllLines(listFile, StandardCharsets.UTF_8)) {
			String t = line.trim();
			if (t.isEmpty() || t.startsWith("#")) {
				continue;
			}
			out.add(t);
		}
		return out;
	}

	/** Enum constants if it is an enum, otherwise its static final primitive/String fields. */
	private static List<Row> describe(Class<?> c) {
		List<Row> out = new ArrayList<>();
		if (c.isEnum()) {
			for (Object k : c.getEnumConstants()) {
				List<String> v = new ArrayList<>();
				v.add("ordinal=" + ((Enum<?>) k).ordinal());
				for (Field f : c.getDeclaredFields()) {
					// `getDeclaredFields()`, not `getFields()`: an enum's payload fields are
					// `private final` (PacketFlow#id, DialogAction#id and #name), and those are
					// precisely the values that matter -- they are the persisted strings and the
					// BY_ID keys. `getFields()` sees only the public ones, so the first version of
					// this dump returned just the ordinal and missed every id.
					if (Modifier.isStatic(f.getModifiers()) || f.isSynthetic()) {
						continue;
					}
					try {
						f.setAccessible(true);
						v.add(f.getName() + "=" + encode(f.get(k)));
					} catch (Throwable ignored) {
						v.add(f.getName() + "=<inaccessible>");
					}
				}
				out.add(new Row(((Enum<?>) k).name(), v));
			}
			return out;
		}
		for (Field f : c.getFields()) {
			int m = f.getModifiers();
			if (!Modifier.isStatic(m) || !Modifier.isFinal(m)) {
				continue;
			}
			Class<?> t = f.getType();
			if (!isScalar(t)) {
				continue; // Codecs, registries, BitSets: existence-only, not comparable
			}
			try {
				out.add(new Row(f.getName(), List.of("value=" + encode(f.get(null)))));
			} catch (Throwable cause) {
				out.add(new Row(f.getName(), List.of("value=<inaccessible:" + cause + ">")));
			}
		}
		return out;
	}

	private static boolean isScalar(Class<?> t) {
		return t == int.class || t == long.class || t == short.class || t == byte.class
				|| t == char.class || t == boolean.class || t == float.class || t == double.class
				|| t == String.class;
	}

	/**
	 * Encode a value with its type, so the Rust side never has to guess.
	 *
	 * Floats and doubles are RAW BITS, hex, for the same reason the rest of the goldens do it:
	 * `-0.0` and `0.0` must compare unequal, and no decimal rendering is guaranteed to round-trip.
	 */
	private static String encode(Object v) throws IllegalAccessException {
		if (v instanceof Integer i) {
			return "i32:" + i;
		}
		if (v instanceof Long l) {
			return "i64:" + l;
		}
		if (v instanceof Short s) {
			return "i32:" + s.intValue();
		}
		if (v instanceof Byte b) {
			return "i32:" + b.intValue();
		}
		if (v instanceof Character ch) {
			return "i32:" + (int) ch.charValue();
		}
		if (v instanceof Boolean bo) {
			return "bool:" + bo;
		}
		if (v instanceof Float f) {
			return "f32:" + String.format("0x%08x", Float.floatToRawIntBits(f));
		}
		if (v instanceof Double d) {
			return "f64:" + String.format("0x%016x", Double.doubleToRawLongBits(d));
		}
		// An enum-typed field records `name()`, NOT `toString()`. This is load-bearing rather than
		// cosmetic: `AdvancementType.chatColor` is a `ChatFormatting`, whose `toString()` is the
		// LEGACY colour code -- `ChatFormatting.GREEN` renders as `"§a"`. Recording that instead
		// of `"GREEN"` makes the dump useless for checking which colour an enum uses, which is
		// the only reason to record the field at all. An arbitrary object's `toString()` is
		// still recorded as-is below, because there is nothing better to record.
		if (v instanceof Enum<?> e) {
			return "str:" + e.name();
		}
		return "str:" + v;
	}
}