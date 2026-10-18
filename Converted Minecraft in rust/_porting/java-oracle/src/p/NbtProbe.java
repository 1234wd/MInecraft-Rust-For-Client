package p;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.io.OutputStreamWriter;
import java.io.PrintWriter;
import java.io.Writer;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Random;
import net.minecraft.nbt.ByteArrayTag;
import net.minecraft.nbt.ByteTag;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.nbt.DoubleTag;
import net.minecraft.nbt.EndTag;
import net.minecraft.nbt.FloatTag;
import net.minecraft.nbt.IntArrayTag;
import net.minecraft.nbt.IntTag;
import net.minecraft.nbt.ListTag;
import net.minecraft.nbt.LongArrayTag;
import net.minecraft.nbt.LongTag;
import net.minecraft.nbt.NbtAccounter;
import net.minecraft.nbt.NbtIo;
import net.minecraft.nbt.NumericTag;
import net.minecraft.nbt.ShortTag;
import net.minecraft.nbt.StreamTagVisitor;
import net.minecraft.nbt.StringTag;
import net.minecraft.nbt.Tag;
import net.minecraft.nbt.TagType;
import net.minecraft.nbt.TagTypes;
import net.minecraft.nbt.visitors.CollectFields;
import net.minecraft.nbt.visitors.CollectToTag;
import net.minecraft.nbt.visitors.FieldSelector;
import net.minecraft.nbt.visitors.SkipAll;
import net.minecraft.nbt.visitors.SkipFields;

/**
 * NBT oracle (session 19, batch g). Writes {@code _porting/test-data/nbt_probe.txt}.
 *
 * <p>Everything here runs against Mojang's bytecode (minecraft-merged-deobf-26.2.jar) and the JDK
 * 25 {@code java.util.HashMap} / {@code java.io.Data*Stream}. It is NOT part of run.ps1's oracle;
 * build and run it by hand (it needs {@code --add-opens java.base/java.util=ALL-UNNAMED} to read
 * HashMap capacities):
 *
 * <pre>
 *   javac -d %TEMP%\nbtprobe -cp "&lt;mc jar&gt;;lib\*" src\p\NbtProbe.java
 *   java --add-opens java.base/java.util=ALL-UNNAMED -cp "%TEMP%\nbtprobe;&lt;mc jar&gt;;lib\*" p.NbtProbe ..\test-data\nbt_probe.txt
 * </pre>
 *
 * <p>Line format: TAB-separated fields, first field = record kind. Strings are escaped with
 * {@link #esc}: backslash doubled, chars outside 0x20..0x7E as {@code \\uXXXX} UTF-16 code units
 * (so lone surrogates survive), null as {@code \\N}. Byte strings are lowercase hex. The tree
 * generator ({@link #genCompound}) is mirrored line-for-line by {@code tests/session19_g.rs}.
 */
@SuppressWarnings("removal")
public class NbtProbe {
	static PrintWriter out;
	static int rows;

	static void row(String... fields) {
		out.print(String.join("\t", fields));
		out.print("\n");
		rows++;
	}

	// ------------------------------------------------------------------ escaping / hex

	static String esc(String s) {
		if (s == null) {
			return "\\N";
		}
		StringBuilder b = new StringBuilder();
		for (int i = 0; i < s.length(); i++) {
			char c = s.charAt(i);
			if (c == '\\') {
				b.append("\\\\");
			} else if (c >= 0x20 && c <= 0x7e) {
				b.append(c);
			} else {
				b.append(String.format("\\u%04x", (int) c));
			}
		}
		return b.toString();
	}

	static String hex(byte[] a) {
		StringBuilder b = new StringBuilder();
		for (byte x : a) {
			b.append(String.format("%02x", x & 0xff));
		}
		return b.toString();
	}

	static byte[] unhex(String s) {
		byte[] a = new byte[s.length() / 2];
		for (int i = 0; i < a.length; i++) {
			a[i] = (byte) Integer.parseInt(s.substring(2 * i, 2 * i + 2), 16);
		}
		return a;
	}

	static String exc(Throwable t) {
		Throwable c = t.getCause();
		return "EX\t" + t.getClass().getName() + "\t" + esc(t.getMessage()) + "\t"
			+ (c == null ? "\\N" : c.getClass().getName()) + "\t" + (c == null ? "\\N" : esc(c.getMessage()));
	}

	// ------------------------------------------------------------------ the shared tree generator

	static final String[] KEY_POOL = {
		"", "a", "b", "id", "Count", "x", "y", "z", "true", "FALSE", "tRuE", "false1",
		"key with space", "quote\"d", "apo's", "both\"'", "back\\slash", "1abc", "_under", ".dot", "a+b-c", "-lead", "+plus",
		"é", "中文", "😀", "Aa", "BB", "AaAa", "BBBB", "AaBB", "BBAa",
		"minecraft:stone", "A", "Z", "zz", "ÿ", "�", "", "a\u0080b", "߿ࠀ"
	};
	static final String[] UNI_POOL = {
		"a", "Z", "0", " ", "\"", "'", "\\", "é", "ÿ", "Ā", "߿", "ࠀ", "中", "�",
		"", "￿", "😀", "𐀀", "􏿿", "\u007f", "\u0080"
	};
	static final String IDENT = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_.+-";
	static final float[] SPECIAL_F = {
		0.0f, -0.0f, 1.0f, -1.0f, 0.5f, 0.1f, 1e7f, 9999999f, 1e-3f, 1e-4f, Float.NaN, Float.POSITIVE_INFINITY,
		Float.NEGATIVE_INFINITY, Float.MIN_VALUE, Float.MAX_VALUE, Float.MIN_NORMAL, 1.0E10f, -1.5f, 255.9f,
		2.14748365E9f, -2.14748365E9f, 65535.7f, 128.5f, -129.5f
	};
	static final double[] SPECIAL_D = {
		0.0, -0.0, 1.0, -1.0, 0.5, 0.1, 1e7, 9999999.0, 1e-3, 1e-4, Double.NaN, Double.POSITIVE_INFINITY,
		Double.NEGATIVE_INFINITY, Double.MIN_VALUE, Double.MAX_VALUE, Double.MIN_NORMAL, 1.0E10, -1.5, 255.9,
		2.147483648E9, -2.147483649E9, 9.3E18, -9.3E18, 65535.7, 128.5, -129.5, 1.0E-300
	};

	static String genKey(Random r) {
		switch (r.nextInt(4)) {
			case 0:
				return KEY_POOL[r.nextInt(KEY_POOL.length)];
			case 1: {
				int len = 1 + r.nextInt(12);
				StringBuilder b = new StringBuilder();
				for (int i = 0; i < len; i++) {
					b.append(IDENT.charAt(r.nextInt(IDENT.length())));
				}
				return b.toString();
			}
			case 2: {
				int len = r.nextInt(6);
				StringBuilder b = new StringBuilder();
				for (int i = 0; i < len; i++) {
					b.append(UNI_POOL[r.nextInt(UNI_POOL.length)]);
				}
				return b.toString();
			}
			default:
				return "k" + r.nextInt(1000);
		}
	}

	static String genString(Random r) {
		switch (r.nextInt(3)) {
			case 0:
				return genKey(r);
			case 1: {
				int len = r.nextInt(40);
				StringBuilder b = new StringBuilder();
				for (int i = 0; i < len; i++) {
					b.append(UNI_POOL[r.nextInt(UNI_POOL.length)]);
				}
				return b.toString();
			}
			default:
				return "";
		}
	}

	static float genFloat(Random r) {
		switch (r.nextInt(3)) {
			case 0:
				return SPECIAL_F[r.nextInt(SPECIAL_F.length)];
			case 1:
				return Float.intBitsToFloat(r.nextInt());
			default:
				return r.nextFloat() * 2000.0f - 1000.0f;
		}
	}

	static double genDouble(Random r) {
		switch (r.nextInt(3)) {
			case 0:
				return SPECIAL_D[r.nextInt(SPECIAL_D.length)];
			case 1:
				return Double.longBitsToDouble(r.nextLong());
			default:
				return r.nextDouble() * 2.0e6 - 1.0e6;
		}
	}

	static Tag genTagOfType(Random r, int t, int depth) {
		switch (t) {
			case 0:
				return ByteTag.valueOf((byte) r.nextInt());
			case 1:
				return ShortTag.valueOf((short) r.nextInt());
			case 2: {
				boolean big = r.nextBoolean();
				return IntTag.valueOf(big ? r.nextInt() : r.nextInt(2000) - 500);
			}
			case 3: {
				boolean big = r.nextBoolean();
				return LongTag.valueOf(big ? r.nextLong() : (long) (r.nextInt(2000) - 500));
			}
			case 4:
				return FloatTag.valueOf(genFloat(r));
			case 5:
				return DoubleTag.valueOf(genDouble(r));
			case 6:
				return StringTag.valueOf(genString(r));
			case 7: {
				byte[] a = new byte[r.nextInt(10)];
				for (int i = 0; i < a.length; i++) {
					a[i] = (byte) r.nextInt();
				}
				return new ByteArrayTag(a);
			}
			case 8: {
				int[] a = new int[r.nextInt(10)];
				for (int i = 0; i < a.length; i++) {
					a[i] = r.nextInt();
				}
				return new IntArrayTag(a);
			}
			case 9: {
				long[] a = new long[r.nextInt(10)];
				for (int i = 0; i < a.length; i++) {
					a[i] = r.nextLong();
				}
				return new LongArrayTag(a);
			}
			case 10:
				return genList(r, depth);
			case 11:
				return genCompound(r, depth);
			default:
				return genCompoundList(r, depth);
		}
	}

	static Tag genTag(Random r, int depth) {
		int t = r.nextInt(depth >= 4 ? 10 : 13);
		return genTagOfType(r, t, depth);
	}

	static ListTag genList(Random r, int depth) {
		ListTag l = new ListTag();
		int n = r.nextInt(6);
		boolean homogeneous = r.nextInt(3) != 0;
		int fixedType = r.nextInt(depth >= 4 ? 10 : 12);
		for (int i = 0; i < n; i++) {
			l.add(homogeneous ? genTagOfType(r, fixedType, depth + 1) : genTag(r, depth + 1));
		}
		return l;
	}

	static ListTag genCompoundList(Random r, int depth) {
		ListTag l = new ListTag();
		int n = r.nextInt(5);
		for (int i = 0; i < n; i++) {
			int k = r.nextInt(4);
			if (k == 0) {
				CompoundTag w = new CompoundTag();
				w.put("", genTag(r, depth + 2));
				l.add(w);
			} else if (k == 3) {
				l.add(genTag(r, depth + 1));
			} else {
				l.add(genCompound(r, depth + 1));
			}
		}
		return l;
	}

	static CompoundTag genCompound(Random r, int depth) {
		CompoundTag c = new CompoundTag();
		int n = r.nextInt(depth == 0 ? 40 : 8);
		List<String> inserted = new ArrayList<>();
		for (int i = 0; i < n; i++) {
			String key = genKey(r);
			Tag value = genTag(r, depth + 1);
			c.put(key, value);
			inserted.add(key);
		}
		int removes = r.nextInt(4) == 0 ? r.nextInt(inserted.size() + 1) : 0;
		for (int i = 0; i < removes; i++) {
			c.remove(inserted.get(r.nextInt(inserted.size())));
		}
		return c;
	}

	// ------------------------------------------------------------------ tracing stream visitor

	/** Logs every call; answers CONTINUE/ENTER, or pseudo-randomly when {@code r != null}. */
	static class TraceVisitor implements StreamTagVisitor {
		final StringBuilder log = new StringBuilder();
		final Random r;

		TraceVisitor(Random r) {
			this.r = r;
		}

		ValueResult vr(String s) {
			ValueResult res = ValueResult.CONTINUE;
			if (r != null) {
				int x = r.nextInt(20);
				res = x == 0 ? ValueResult.HALT : x <= 2 ? ValueResult.BREAK : ValueResult.CONTINUE;
			}
			log.append(s).append('=').append(res.name()).append(';');
			return res;
		}

		EntryResult er(String s) {
			EntryResult res = EntryResult.ENTER;
			if (r != null) {
				int x = r.nextInt(20);
				res = x == 0 ? EntryResult.HALT : x <= 2 ? EntryResult.BREAK : x <= 5 ? EntryResult.SKIP : EntryResult.ENTER;
			}
			log.append(s).append('=').append(res.name()).append(';');
			return res;
		}

		@Override public ValueResult visitEnd() { return vr("E"); }
		@Override public ValueResult visit(String v) { return vr("s:" + esc(v)); }
		@Override public ValueResult visit(byte v) { return vr("b:" + v); }
		@Override public ValueResult visit(short v) { return vr("h:" + v); }
		@Override public ValueResult visit(int v) { return vr("i:" + v); }
		@Override public ValueResult visit(long v) { return vr("l:" + v); }
		@Override public ValueResult visit(float v) { return vr("f:" + Integer.toHexString(Float.floatToIntBits(v))); }
		@Override public ValueResult visit(double v) { return vr("d:" + Long.toHexString(Double.doubleToLongBits(v))); }

		@Override
		public ValueResult visit(byte[] v) {
			StringBuilder b = new StringBuilder("B:");
			for (int i = 0; i < v.length; i++) {
				b.append(i == 0 ? "" : ",").append(v[i]);
			}
			return vr(b.toString());
		}

		@Override
		public ValueResult visit(int[] v) {
			StringBuilder b = new StringBuilder("I:");
			for (int i = 0; i < v.length; i++) {
				b.append(i == 0 ? "" : ",").append(v[i]);
			}
			return vr(b.toString());
		}

		@Override
		public ValueResult visit(long[] v) {
			StringBuilder b = new StringBuilder("L:");
			for (int i = 0; i < v.length; i++) {
				b.append(i == 0 ? "" : ",").append(v[i]);
			}
			return vr(b.toString());
		}

		@Override public ValueResult visitList(TagType<?> t, int size) { return vr("LIST:" + t.getName() + "," + size); }
		@Override public EntryResult visitEntry(TagType<?> t) { return er("ENT:" + t.getName()); }
		@Override public EntryResult visitEntry(TagType<?> t, String id) { return er("ENTN:" + t.getName() + "," + esc(id)); }
		@Override public EntryResult visitElement(TagType<?> t, int index) { return er("ELEM:" + t.getName() + "," + index); }
		@Override public ValueResult visitContainerEnd() { return vr("CEND"); }
		@Override public ValueResult visitRootEntry(TagType<?> t) { return vr("ROOT:" + t.getName()); }
	}

	// ------------------------------------------------------------------ helpers over the jar

	static Method SHALLOW_COPY;
	static Method IDENTIFY;
	static Field TABLE;

	static byte[] nbtWrite(CompoundTag t) throws IOException {
		ByteArrayOutputStream bos = new ByteArrayOutputStream();
		NbtIo.write(t, new DataOutputStream(bos));
		return bos.toByteArray();
	}

	static String directWrite(Tag t) {
		ByteArrayOutputStream bos = new ByteArrayOutputStream();
		try {
			t.write(new DataOutputStream(bos));
			return "OK\t" + hex(bos.toByteArray());
		} catch (Throwable e) {
			return exc(e);
		}
	}

	static String anyHex(Tag t) throws IOException {
		if (t == null) {
			return "\\N";
		}
		ByteArrayOutputStream bos = new ByteArrayOutputStream();
		NbtIo.writeAnyTag(t, new DataOutputStream(bos));
		return hex(bos.toByteArray());
	}

	static DataInputStream in(byte[] b) {
		return new DataInputStream(new ByteArrayInputStream(b));
	}

	static int capacity(Map<?, ?> m) throws Exception {
		Object[] t = (Object[]) TABLE.get(m);
		return t == null ? 0 : t.length;
	}

	static final FieldSelector[][] SELECTORS = {
		{ new FieldSelector(IntTag.TYPE, "id"), new FieldSelector(StringTag.TYPE, "x"), new FieldSelector("a", ByteTag.TYPE, "b") },
		{ new FieldSelector("a", "b", ListTag.TYPE, "x"), new FieldSelector(CompoundTag.TYPE, "a"), new FieldSelector(ListTag.TYPE, "y") },
		{ new FieldSelector("x", LongTag.TYPE, "y"), new FieldSelector("x", CompoundTag.TYPE, "z"), new FieldSelector(DoubleTag.TYPE, "Count"),
			new FieldSelector(ShortTag.TYPE, "") },
	};

	/** All per-tree records for one tree; {@code kind} is GEN or CASE. */
	static void treeRows(String kind, String id, CompoundTag t, boolean withTraces) throws Exception {
		byte[] w = nbtWrite(t);
		row(kind, id, "W", hex(w));
		row(kind, id, "S", esc(t.toString()));
		row(kind, id, "Z", Integer.toString(t.sizeInBytes()));
		row(kind, id, "H", Integer.toString(t.hashCode()));
		row(kind, id, "CW", hex(nbtWrite(t.copy())));
		row(kind, id, "SW", hex(nbtWrite((CompoundTag) SHALLOW_COPY.invoke(t))));
		NbtAccounter acc = NbtAccounter.unlimitedHeap();
		CompoundTag back = NbtIo.read(in(w), acc);
		row(kind, id, "RW", hex(nbtWrite(back)));
		row(kind, id, "RU", Long.toString(acc.getUsage()));
		row(kind, id, "RE", Boolean.toString(back.equals(t)));
		CollectToTag collect = new CollectToTag();
		NbtAccounter acc2 = NbtAccounter.unlimitedHeap();
		NbtIo.parse(in(w), collect, acc2);
		row(kind, id, "PW", anyHex(collect.getResult()));
		row(kind, id, "PU", Long.toString(acc2.getUsage()));
		NbtIo.parse(in(w), SkipAll.INSTANCE, NbtAccounter.unlimitedHeap());
		for (int s = 0; s < SELECTORS.length; s++) {
			CollectFields cf = new CollectFields(SELECTORS[s]);
			NbtIo.parse(in(w), cf, NbtAccounter.unlimitedHeap());
			row(kind, id, "CF" + s, anyHex(cf.getResult()) + "\t" + cf.getMissingFieldCount());
			SkipFields sf = new SkipFields(SELECTORS[s]);
			NbtIo.parse(in(w), sf, NbtAccounter.unlimitedHeap());
			row(kind, id, "SF" + s, anyHex(sf.getResult()));
		}
		if (withTraces) {
			for (int policy = 0; policy < 3; policy++) {
				Random pr = policy == 0 ? null : new Random(1000L * policy + id.hashCode());
				TraceVisitor tv = new TraceVisitor(pr);
				t.acceptAsRoot(tv);
				row(kind, id, "TA" + policy, tv.log.toString());
				pr = policy == 0 ? null : new Random(1000L * policy + id.hashCode());
				TraceVisitor tp = new TraceVisitor(pr);
				try {
					NbtIo.parse(in(w), tp, NbtAccounter.unlimitedHeap());
					row(kind, id, "TP" + policy, tp.log.toString());
				} catch (Throwable e) {
					row(kind, id, "TP" + policy, tp.log.toString() + "!" + e.getClass().getName());
				}
			}
		}
	}

	// ------------------------------------------------------------------ sections

	static void genSection() throws Exception {
		for (int seed = 0; seed < 120; seed++) {
			Random r = new Random(seed);
			CompoundTag t = genCompound(r, 0);
			treeRows("GEN", Integer.toString(seed), t, seed < 60);
		}
	}

	static String[] colliding(int n) {
		// "Aa" and "BB" share a String.hashCode; every same-length concatenation does too.
		String[] out = new String[n];
		for (int i = 0; i < n; i++) {
			StringBuilder b = new StringBuilder();
			for (int bit = 5; bit >= 0; bit--) {
				b.append(((i >> bit) & 1) == 0 ? "Aa" : "BB");
			}
			out[i] = b.toString();
		}
		return out;
	}

	static void caseSection() throws Exception {
		treeRows("CASE", "empty", new CompoundTag(), true);

		CompoundTag all = new CompoundTag();
		all.putByte("b", (byte) 1);
		all.putShort("s", (short) 2);
		all.putInt("i", 3);
		all.putLong("l", 4L);
		all.putFloat("f", 5.5f);
		all.putDouble("d", 6.25);
		all.putString("str", "hello");
		all.putByteArray("ba", new byte[] {1, -2, 3});
		all.putIntArray("ia", new int[] {1, -2, Integer.MAX_VALUE, Integer.MIN_VALUE});
		all.putLongArray("la", new long[] {1L, -2L, Long.MAX_VALUE, Long.MIN_VALUE});
		ListTag ints = new ListTag();
		ints.add(IntTag.valueOf(1));
		ints.add(IntTag.valueOf(2));
		all.put("list", ints);
		CompoundTag inner = new CompoundTag();
		inner.putString("name", "inner");
		all.put("cmp", inner);
		all.put("emptylist", new ListTag());
		all.putBoolean("bool", true);
		treeRows("CASE", "all_types", all, true);

		CompoundTag many = new CompoundTag();
		for (int i = 0; i < 200; i++) {
			many.putInt("k" + i, i);
		}
		treeRows("CASE", "many_keys", many, false);
		for (int i = 0; i < 150; i++) {
			many.remove("k" + i);
		}
		treeRows("CASE", "many_keys_removed", many, false);

		CompoundTag uni = new CompoundTag();
		uni.putString("é", "中文");
		uni.putString("😀", "x😀y");
		uni.putString("max", "￿ࠀ߿\u0080\u007f");
		uni.putString("�", "");
		treeRows("CASE", "unicode", uni, true);

		CompoundTag hetero = new CompoundTag();
		ListTag hl = new ListTag();
		hl.add(IntTag.valueOf(1));
		hl.add(StringTag.valueOf("a"));
		CompoundTag hx = new CompoundTag();
		hx.putInt("x", 1);
		hl.add(hx);
		CompoundTag hw = new CompoundTag();
		hw.putByte("", (byte) 5);
		hl.add(hw);
		hl.add(new ListTag());
		hetero.put("hetero", hl);
		ListTag wl = new ListTag();
		CompoundTag w1 = new CompoundTag();
		w1.putInt("", 1);
		wl.add(w1);
		CompoundTag w2 = new CompoundTag();
		w2.putInt("", 2);
		w2.putInt("other", 3);
		wl.add(w2);
		hetero.put("wrappers", wl);
		treeRows("CASE", "hetero_list", hetero, true);

		CompoundTag deep = new CompoundTag();
		CompoundTag cur = deep;
		for (int i = 0; i < 10; i++) {
			CompoundTag next = new CompoundTag();
			ListTag l = new ListTag();
			l.add(next);
			cur.put("d" + i, l);
			cur.putInt("v", i);
			cur = next;
		}
		treeRows("CASE", "nested_deep", deep, true);

		CompoundTag nz = new CompoundTag();
		nz.putFloat("f", -0.0f);
		nz.putDouble("d", -0.0);
		nz.put("f2", new FloatTag(-0.0f));
		nz.put("d2", new DoubleTag(-0.0));
		nz.put("nan", new FloatTag(Float.intBitsToFloat(0x7f800001)));
		nz.put("dnan", new DoubleTag(Double.longBitsToDouble(0xfff0000000000001L)));
		treeRows("CASE", "neg_zero_nan", nz, true);

		// Trees the SELECTORS actually hit (random keys rarely do).
		CompoundTag selA = new CompoundTag();
		selA.putInt("id", 5);
		CompoundTag sx = new CompoundTag();
		sx.putLong("y", 7L);
		CompoundTag sz = new CompoundTag();
		sz.putInt("q", 1);
		sx.put("z", sz);
		sx.putInt("w", 2);
		selA.put("x", sx);
		CompoundTag sa = new CompoundTag();
		sa.putByte("b", (byte) 1);
		sa.putInt("c", 2);
		selA.put("a", sa);
		selA.putDouble("Count", 3.5);
		selA.putShort("", (short) 4);
		selA.putString("other", "o");
		ListTag sl = new ListTag();
		CompoundTag sli = new CompoundTag();
		sli.putInt("id", 1);
		sl.add(sli);
		selA.put("lst", sl);
		treeRows("CASE", "sel_a", selA, true);
		CompoundTag selB = new CompoundTag();
		CompoundTag ba = new CompoundTag();
		CompoundTag bb = new CompoundTag();
		ListTag bx = new ListTag();
		bx.add(IntTag.valueOf(1));
		bx.add(IntTag.valueOf(2));
		bb.put("x", bx);
		bb.putInt("y", 3);
		ba.put("b", bb);
		ba.putInt("c", 4);
		selB.put("a", ba);
		ListTag by = new ListTag();
		by.add(IntTag.valueOf(5));
		selB.put("y", by);
		selB.putString("x", "str");
		selB.putString("id", "notint");
		treeRows("CASE", "sel_b", selB, true);

		for (int n : new int[] {8, 9, 10, 11, 12, 20, 40}) {
			CompoundTag col = new CompoundTag();
			String[] keys = colliding(n);
			for (int i = 0; i < n; i++) {
				col.putInt(keys[i], i);
			}
			treeRows("CASE", "colliding_" + n, col, false);
		}

		// Quirks with EndTag inside containers: written, but not readable back the same way.
		CompoundTag endList = new CompoundTag();
		ListTag el = new ListTag();
		el.add(EndTag.INSTANCE);
		el.add(EndTag.INSTANCE);
		endList.put("l", el);
		row("CASEW", "end_list", hex(nbtWrite(endList)), esc(endList.toString()), Integer.toString(endList.sizeInBytes()));
		CompoundTag endInt = new CompoundTag();
		ListTag ei = new ListTag();
		ei.add(EndTag.INSTANCE);
		ei.add(IntTag.valueOf(7));
		endInt.put("l", ei);
		row("CASEW", "end_then_int_list", hex(nbtWrite(endInt)), esc(endInt.toString()), Integer.toString(endInt.sizeInBytes()));
		CompoundTag endVal = new CompoundTag();
		endVal.put("a", EndTag.INSTANCE);
		endVal.putInt("b", 1);
		row("CASEW", "end_value", hex(nbtWrite(endVal)), esc(endVal.toString()), Integer.toString(endVal.sizeInBytes()));

		// Strings: the NbtIo fallback vs a direct write of an over-long string.
		CompoundTag longStr = new CompoundTag();
		longStr.putString("s", "a".repeat(65536));
		longStr.putString("t", "中".repeat(21845));
		row("CASEW", "long_string_fallback", hex(nbtWrite(longStr)), "-", Integer.toString(longStr.sizeInBytes()));
		row("CASED", "long_string_direct", directWrite(StringTag.valueOf("a".repeat(65536))));
		row("CASED", "long_string_direct_cjk", directWrite(StringTag.valueOf("中".repeat(21846))));
		row("CASED", "max_string_direct", directWrite(StringTag.valueOf("中".repeat(21845))));

		CompoundTag ctl = new CompoundTag();
		ctl.putString("tab\t", "nul\u0000 line\n cr\r bell\u0007 esc\u001b bs\b ff\f");
		row("CASEW", "control_chars", hex(nbtWrite(ctl)), esc(ctl.toString()), Integer.toString(ctl.sizeInBytes()));

		ListTag ident = new ListTag();
		row("IDENT", "empty", Byte.toString((Byte) IDENTIFY.invoke(ident)));
		ident.add(IntTag.valueOf(1));
		row("IDENT", "int", Byte.toString((Byte) IDENTIFY.invoke(ident)));
		ident.add(IntTag.valueOf(2));
		row("IDENT", "int_int", Byte.toString((Byte) IDENTIFY.invoke(ident)));
		ident.add(StringTag.valueOf("x"));
		row("IDENT", "int_int_string", Byte.toString((Byte) IDENTIFY.invoke(ident)));
		ListTag ident2 = new ListTag();
		ident2.add(EndTag.INSTANCE);
		ident2.add(LongTag.valueOf(5));
		row("IDENT", "end_long", Byte.toString((Byte) IDENTIFY.invoke(ident2)));
	}

	static byte[] build(IOConsumer f) throws IOException {
		ByteArrayOutputStream bos = new ByteArrayOutputStream();
		f.accept(new DataOutputStream(bos));
		return bos.toByteArray();
	}

	interface IOConsumer {
		void accept(DataOutputStream d) throws IOException;
	}

	interface Reader {
		Tag read(DataInputStream d, NbtAccounter a) throws IOException;
	}

	static void readRows(String name, byte[] input, long quota, int maxDepth) {
		String[] names = {"read", "any", "unnamed", "parse"};
		Reader[] readers = {
			NbtIo::read,
			NbtIo::readAnyTag,
			NbtIo::readUnnamedTag,
			(d, a) -> {
				CollectToTag c = new CollectToTag();
				NbtIo.parse(d, c, a);
				return c.getResult();
			}
		};
		for (int i = 0; i < readers.length; i++) {
			NbtAccounter acc = new NbtAccounter(quota, maxDepth);
			String res;
			try {
				Tag t = readers[i].read(in(input), acc);
				res = "OK\t" + anyHex(t) + "\t" + acc.getUsage() + "\t" + acc.getDepth();
			} catch (Throwable e) {
				res = exc(e);
			}
			row("READ", name, names[i], Long.toString(quota), Integer.toString(maxDepth), hex(input), res);
		}
	}

	static void readSection() throws Exception {
		long U = Long.MAX_VALUE;
		readRows("empty_input", new byte[0], U, 512);
		readRows("end_root", new byte[] {0}, U, 512);
		readRows("byte_root", unhex("0100000005"), U, 512);
		readRows("byte_root_named", unhex("010003616263ff"), U, 512);
		readRows("compound_no_end", unhex("0a0000"), U, 512);
		readRows("compound_empty", unhex("0a000000"), U, 512);
		readRows("compound_trailing", unhex("0a00000000ffff"), U, 512);
		readRows("compound_invalid_child", unhex("0a0000630001610000"), U, 512);
		readRows("root_invalid", unhex("20000000"), U, 512);
		readRows("root_negative_id", unhex("ff000000"), U, 512);
		readRows("list_negative", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(9); d.writeUTF("l"); d.writeByte(3); d.writeInt(-1); d.writeByte(0); }), U, 512);
		readRows("list_missing_type", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(9); d.writeUTF("l"); d.writeByte(0); d.writeInt(2); d.writeByte(0); }), U, 512);
		readRows("list_end_empty", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(9); d.writeUTF("l"); d.writeByte(0); d.writeInt(0); d.writeByte(0); }), U, 512);
		readRows("root_list_negative", build(d -> { d.writeByte(9); d.writeUTF(""); d.writeByte(1); d.writeInt(-5); }), U, 512);
		readRows("root_list_bytes", build(d -> { d.writeByte(9); d.writeUTF(""); d.writeByte(1); d.writeInt(3); d.writeByte(1); d.writeByte(2); d.writeByte(3); }), U, 512);
		readRows("root_list_truncated", build(d -> { d.writeByte(9); d.writeUTF(""); d.writeByte(3); d.writeInt(3); d.writeInt(1); }), U, 512);
		readRows("bytearray_negative", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(7); d.writeUTF("a"); d.writeInt(-1); d.writeByte(0); }), U, 512);
		readRows("intarray_negative", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(11); d.writeUTF("a"); d.writeInt(-3); d.writeByte(0); }), U, 512);
		readRows("bytearray_truncated", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(7); d.writeUTF("a"); d.writeInt(10); d.writeByte(1); }), U, 512);
		readRows("root_bytearray", build(d -> { d.writeByte(7); d.writeUTF("name"); d.writeInt(3); d.write(new byte[] {-1, 0, 1}); }), U, 512);
		readRows("root_string", build(d -> { d.writeByte(8); d.writeUTF(""); d.writeUTF("hi\u0000é"); }), U, 512);
		readRows("root_string_badutf", unhex("080000000280ff"), U, 512);
		readRows("root_string_partial", unhex("0800000002e0"), U, 512);
		readRows("root_string_partial2", unhex("0800000003e080"), U, 512);
		readRows("root_string_badcont", unhex("0800000003e08041"), U, 512);
		readRows("root_string_badcont2", unhex("0800000002c041"), U, 512);
		readRows("root_string_overlong", unhex("0800000004c080c1bf"), U, 512);
		readRows("root_string_raw_nul", unhex("080000000200f0"), U, 512);
		readRows("root_string_surrogates", unhex("0800000006eda0bdedb880"), U, 512);
		readRows("key_badutf", unhex("0a00000100018041" + "00"), U, 512);
		readRows("compound_dup_keys", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(3); d.writeUTF("a"); d.writeInt(1); d.writeByte(3); d.writeUTF("a"); d.writeInt(2); d.writeByte(0); }), U, 512);
		readRows("wrapped_list", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(9); d.writeUTF("l"); d.writeByte(10); d.writeInt(3);
			d.writeByte(3); d.writeUTF(""); d.writeInt(7); d.writeByte(0);
			d.writeByte(0);
			d.writeByte(3); d.writeUTF(""); d.writeInt(8); d.writeByte(3); d.writeUTF("x"); d.writeInt(9); d.writeByte(0);
			d.writeByte(0); }), U, 512);
		readRows("float_nan_payload", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(5); d.writeUTF("f"); d.writeInt(0x7f800001); d.writeByte(6); d.writeUTF("d"); d.writeLong(0xfff0000000000001L); d.writeByte(0); }), U, 512);

		// Quota and depth.
		byte[] small = nbtWrite(new CompoundTag());
		for (long q : new long[] {0, 47, 48, 49}) {
			readRows("quota_empty_" + q, small, q, 512);
		}
		CompoundTag one = new CompoundTag();
		one.putInt("abc", 1);
		byte[] oneB = nbtWrite(one);
		for (long q : new long[] {48, 100, 117, 118, 129, 130, 1000}) {
			readRows("quota_one_" + q, oneB, q, 512);
		}
		Random gr = new Random(77);
		byte[] big = nbtWrite(genCompound(gr, 0));
		for (long q : new long[] {100, 1000, 3000, 10000}) {
			readRows("quota_gen_" + q, big, q, 512);
		}
		for (int depth : new int[] {511, 512, 513, 600}) {
			final int dd = depth;
			byte[] nested = build(d -> {
				d.writeByte(9);
				d.writeUTF("");
				for (int i = 0; i < dd - 1; i++) {
					d.writeByte(9);
					d.writeInt(1);
				}
				d.writeByte(0);
				d.writeInt(0);
			});
			readRows("deep_list_" + depth, nested, U, 512);
			byte[] nestedC = build(d -> {
				d.writeByte(10);
				d.writeUTF("");
				for (int i = 0; i < dd - 1; i++) {
					d.writeByte(10);
					d.writeUTF("c");
				}
				for (int i = 0; i < dd; i++) {
					d.writeByte(0);
				}
			});
			readRows("deep_compound_" + depth, nestedC, U, 512);
		}
		readRows("maxdepth2_ok", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(10); d.writeUTF("a"); d.writeByte(0); d.writeByte(0); }), U, 2);
		readRows("maxdepth2_fail", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(10); d.writeUTF("a"); d.writeByte(9); d.writeUTF("b"); d.writeByte(0); d.writeInt(0); d.writeByte(0); d.writeByte(0); }), U, 2);
		readRows("huge_list_count_quota", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(9); d.writeUTF("l"); d.writeByte(3); d.writeInt(0x7fffffff); }), 2097152L, 512);
		readRows("huge_bytearray_quota", build(d -> { d.writeByte(10); d.writeUTF(""); d.writeByte(7); d.writeUTF("l"); d.writeInt(0x7fffffff); }), 2097152L, 512);

		// Direct accounter behaviour.
		NbtAccounter a = NbtAccounter.create(100);
		StringBuilder log = new StringBuilder();
		for (long sz : new long[] {10, 50, 40, 1, 0, -1}) {
			try {
				a.accountBytes(sz);
				log.append("ok:").append(a.getUsage()).append(';');
			} catch (Throwable e) {
				log.append(e.getClass().getName()).append(':').append(e.getMessage()).append(';');
			}
		}
		try {
			a.accountBytes(3L, -2L);
		} catch (Throwable e) {
			log.append(e.getClass().getName()).append(':').append(e.getMessage()).append(';');
		}
		row("ACC", "account", esc(log.toString()));
		NbtAccounter d = new NbtAccounter(Long.MAX_VALUE, 3);
		log = new StringBuilder();
		for (int i = 0; i < 5; i++) {
			try {
				d.pushDepth();
				log.append("push:").append(d.getDepth()).append(';');
			} catch (Throwable e) {
				log.append(e.getClass().getName()).append(':').append(e.getMessage()).append(';');
			}
		}
		for (int i = 0; i < 5; i++) {
			try {
				d.popDepth();
				log.append("pop:").append(d.getDepth()).append(';');
			} catch (Throwable e) {
				log.append(e.getClass().getName()).append(':').append(e.getMessage()).append(';');
			}
		}
		row("ACC", "depth", esc(log.toString()));
		NbtAccounter ov = NbtAccounter.unlimitedHeap();
		log = new StringBuilder();
		try {
			ov.accountBytes(Long.MAX_VALUE);
			log.append("ok:").append(ov.getUsage()).append(';');
			ov.accountBytes(1);
			log.append("ok:").append(ov.getUsage()).append(';');
		} catch (Throwable e) {
			log.append(e.getClass().getName()).append(':').append(e.getMessage()).append(';');
		}
		try {
			NbtAccounter.unlimitedHeap().accountBytes(4L, 0x4000000000000000L);
			log.append("ok-wrap;");
		} catch (Throwable e) {
			log.append(e.getClass().getName()).append(':').append(e.getMessage()).append(';');
		}
		row("ACC", "overflow", esc(log.toString()));
	}

	static void typeSection() {
		for (int id = -2; id <= 14; id++) {
			TagType<?> t = TagTypes.getType(id);
			String size = t instanceof TagType.StaticSize<?> s ? Integer.toString(s.size()) : "-";
			row("TT", Integer.toString(id), t.getName(), t.getPrettyName(), size);
		}
		try {
			TagTypes.getType(77).load(in(new byte[0]), NbtAccounter.unlimitedHeap());
		} catch (Throwable e) {
			row("TTEX", "load77", exc(e));
		}
	}

	static String numRow(NumericTag t) {
		Tag tag = (Tag) t;
		return t.byteValue() + "\t" + t.shortValue() + "\t" + t.intValue() + "\t" + t.longValue() + "\t"
			+ Integer.toHexString(Float.floatToRawIntBits(t.floatValue())) + "\t"
			+ Long.toHexString(Double.doubleToRawLongBits(t.doubleValue())) + "\t" + esc(tag.toString()) + "\t" + tag.hashCode()
			+ "\t" + tag.asBoolean().get() + "\t" + t.box().getClass().getSimpleName() + "\t" + t.box() + "\t" + tag.sizeInBytes();
	}

	static void primSection() {
		for (int b : new int[] {-128, -1, 0, 1, 127}) {
			row("PRIM", "B:" + b, numRow(ByteTag.valueOf((byte) b)));
		}
		for (int s : new int[] {-32768, -129, -128, -1, 0, 255, 256, 1024, 1025, 32767}) {
			row("PRIM", "S:" + s, numRow(ShortTag.valueOf((short) s)));
		}
		for (int i : new int[] {Integer.MIN_VALUE, -65536, -32769, -129, -1, 0, 1, 255, 256, 65535, 65536, 1025, Integer.MAX_VALUE}) {
			row("PRIM", "I:" + i, numRow(IntTag.valueOf(i)));
		}
		for (long l : new long[] {Long.MIN_VALUE, -4294967296L, -2147483649L, -1L, 0L, 1L, 2147483648L, 4294967297L, 1099511627776L, Long.MAX_VALUE}) {
			row("PRIM", "L:" + l, numRow(LongTag.valueOf(l)));
		}
		for (float f : SPECIAL_F) {
			row("PRIM", "F:" + Integer.toHexString(Float.floatToRawIntBits(f)), numRow(new FloatTag(f)));
		}
		row("PRIM", "F:7f800001", numRow(new FloatTag(Float.intBitsToFloat(0x7f800001))));
		row("PRIM", "F:ffc00000", numRow(new FloatTag(Float.intBitsToFloat(0xffc00000))));
		for (double v : SPECIAL_D) {
			row("PRIM", "D:" + Long.toHexString(Double.doubleToRawLongBits(v)), numRow(new DoubleTag(v)));
		}
		row("PRIM", "D:fff0000000000001", numRow(new DoubleTag(Double.longBitsToDouble(0xfff0000000000001L))));

		// Non-numeric tags' views and hash codes.
		Tag[] others = {
			StringTag.valueOf(""), StringTag.valueOf("abc"), StringTag.valueOf("😀"),
			new ByteArrayTag(new byte[] {1, 2, -3}), new IntArrayTag(new int[] {1, 2, -3}), new LongArrayTag(new long[] {1, 2, -3, Long.MIN_VALUE}),
			new ByteArrayTag(new byte[0]), new ListTag(), new CompoundTag()
		};
		for (int i = 0; i < others.length; i++) {
			Tag t = others[i];
			row("OTHER", Integer.toString(i), esc(t.toString()), Integer.toString(t.hashCode()), Integer.toString(t.sizeInBytes()),
				t.asString().map(NbtProbe::esc).orElse("\\N"), t.asNumber().map(Object::toString).orElse("\\N"),
				t.asBoolean().map(Object::toString).orElse("\\N"));
		}

		// ListTag / CompoundTag getters over mixed content.
		ListTag l = new ListTag();
		l.add(ByteTag.valueOf((byte) -5));
		l.add(FloatTag.valueOf(3.75f));
		l.add(StringTag.valueOf("s"));
		l.add(new IntArrayTag(new int[] {9}));
		l.add(DoubleTag.valueOf(-2.5));
		for (int i = -1; i <= 5; i++) {
			row("LGET", Integer.toString(i), Short.toString(l.getShortOr(i, (short) 77)), Integer.toString(l.getIntOr(i, 77)),
				Long.toHexString(Double.doubleToRawLongBits(l.getDoubleOr(i, 77.0))), Integer.toHexString(Float.floatToRawIntBits(l.getFloatOr(i, 77.0f))),
				esc(l.getStringOr(i, "dflt")), l.getInt(i).map(Object::toString).orElse("\\N"), l.getString(i).map(NbtProbe::esc).orElse("\\N"),
				l.getDouble(i).map(Object::toString).orElse("\\N"), l.getIntArray(i).map(a -> Integer.toString(a.length)).orElse("\\N"));
		}
		CompoundTag c = new CompoundTag();
		c.putFloat("f", -1.5f);
		c.putDouble("d", 1e10);
		c.putString("s", "str");
		c.putLong("l", 0x100000080L);
		c.putBoolean("t", true);
		c.putByte("two", (byte) 2);
		for (String k : new String[] {"f", "d", "s", "l", "t", "two", "missing"}) {
			row("CGET", k, Byte.toString(c.getByteOr(k, (byte) 9)), Short.toString(c.getShortOr(k, (short) 9)), Integer.toString(c.getIntOr(k, 9)),
				Long.toString(c.getLongOr(k, 9L)), Integer.toHexString(Float.floatToRawIntBits(c.getFloatOr(k, 9.0f))),
				Long.toHexString(Double.doubleToRawLongBits(c.getDoubleOr(k, 9.0))), esc(c.getStringOr(k, "dflt")),
				Boolean.toString(c.getBooleanOr(k, false)), Boolean.toString(c.getBooleanOr(k, true)),
				c.getBoolean(k).map(Object::toString).orElse("\\N"), c.getByte(k).map(Object::toString).orElse("\\N"),
				c.getLong(k).map(Object::toString).orElse("\\N"), c.getFloat(k).map(Object::toString).orElse("\\N"));
		}
	}

	static void indexSection() {
		String[] names = {
			"list_get_3", "list_get_neg", "list_set_3", "list_add_4", "list_add_neg", "list_remove_3", "list_settag_3", "list_addtag_4",
			"ba_get_3", "ba_settag_3", "ba_addtag_4", "ba_addtag_neg", "ba_remove_3", "ia_get_3", "ia_addtag_4", "ia_remove_neg",
			"la_get_3", "la_addtag_5", "la_remove_3"
		};
		for (String n : names) {
			ListTag l = new ListTag();
			l.add(IntTag.valueOf(1));
			l.add(IntTag.valueOf(2));
			l.add(IntTag.valueOf(3));
			ByteArrayTag ba = new ByteArrayTag(new byte[] {1, 2, 3});
			IntArrayTag ia = new IntArrayTag(new int[] {1, 2, 3});
			LongArrayTag la = new LongArrayTag(new long[] {1, 2, 3});
			String res;
			try {
				Object o = switch (n) {
					case "list_get_3" -> l.get(3);
					case "list_get_neg" -> l.get(-1);
					case "list_set_3" -> l.set(3, IntTag.valueOf(0));
					case "list_add_4" -> { l.add(4, IntTag.valueOf(0)); yield null; }
					case "list_add_neg" -> { l.add(-1, IntTag.valueOf(0)); yield null; }
					case "list_remove_3" -> l.remove(3);
					case "list_settag_3" -> l.setTag(3, IntTag.valueOf(0));
					case "list_addtag_4" -> l.addTag(4, IntTag.valueOf(0));
					case "ba_get_3" -> ba.get(3);
					case "ba_settag_3" -> ba.setTag(3, IntTag.valueOf(0));
					case "ba_addtag_4" -> ba.addTag(4, IntTag.valueOf(0));
					case "ba_addtag_neg" -> ba.addTag(-1, IntTag.valueOf(0));
					case "ba_remove_3" -> ba.remove(3);
					case "ia_get_3" -> ia.get(3);
					case "ia_addtag_4" -> ia.addTag(4, IntTag.valueOf(0));
					case "ia_remove_neg" -> ia.remove(-1);
					case "la_get_3" -> la.get(3);
					case "la_addtag_5" -> la.addTag(5, IntTag.valueOf(0));
					case "la_remove_3" -> la.remove(3);
					default -> throw new IllegalStateException(n);
				};
				res = "OK\t" + o;
			} catch (Throwable e) {
				res = exc(e);
			}
			row("IDX", n, res);
		}
		// Successful collection edits, shown via toString.
		ByteArrayTag ba = new ByteArrayTag(new byte[] {1, 2, 3});
		ba.addTag(0, FloatTag.valueOf(-1.5f));
		ba.addTag(4, LongTag.valueOf(0x1ff));
		ba.setTag(1, DoubleTag.valueOf(300.7));
		boolean rejected = ba.addTag(0, StringTag.valueOf("x"));
		ba.remove(2);
		row("COLL", "bytearray", esc(ba.toString()), Boolean.toString(rejected));
		IntArrayTag ia = new IntArrayTag(new int[] {1, 2, 3});
		ia.addTag(3, DoubleTag.valueOf(-1e20));
		ia.setTag(0, LongTag.valueOf(0x123456789L));
		ia.remove(1);
		row("COLL", "intarray", esc(ia.toString()), Boolean.toString(ia.setTag(0, new ListTag())));
		LongArrayTag la = new LongArrayTag(new long[] {1, 2, 3});
		la.addTag(1, FloatTag.valueOf(1e30f));
		la.setTag(0, DoubleTag.valueOf(-2.5));
		la.clear();
		la.addTag(0, ByteTag.valueOf((byte) -1));
		row("COLL", "longarray", esc(la.toString()), Boolean.toString(la.isEmpty()));
	}

	static void quoteSection() {
		String[] inputs = {
			"", "a", "a b", "\"", "'", "\"'", "'\"", "it's", "say \"hi\"", "back\\slash", "both ' and \"", "é中😀",
			"\u007f\u0080", "tab\tnl\n", "\u0000", "\u0001\u001f", "\b\f\r", " "
		};
		for (String s : inputs) {
			row("QUOTE", esc(s), esc(StringTag.quoteAndEscape(s)), esc(StringTag.escapeWithoutQuotes(s)));
		}
	}

	static void ioSection() throws Exception {
		String[] utf = {
			"", "abc", "\u0000", "a\u0000b", "\u007f", "\u0080", "߿", "ࠀ", "￿", "😀", "é中",
			"\ud800", "\udc00x", "a".repeat(65535), "a".repeat(65536), "中".repeat(21845), "中".repeat(21846),
			"\u0000".repeat(32768), "\u0000".repeat(32767) + "a", "😀".repeat(10923), "😀".repeat(10922) + "é",
			"😀abcdefgh" + "x".repeat(65530) + "123456😀"
		};
		for (String s : utf) {
			ByteArrayOutputStream bos = new ByteArrayOutputStream();
			DataOutputStream d = new DataOutputStream(bos);
			String res;
			try {
				d.writeUTF(s);
				byte[] b = bos.toByteArray();
				res = b.length > 200 ? "OKLEN\t" + b.length + "\t" + hex(java.util.Arrays.copyOf(b, 16)) : "OK\t" + hex(b);
			} catch (Throwable e) {
				res = exc(e);
			}
			String label = s.length() > 64 ? "LONG:" + s.length() + ":" + s.hashCode() : esc(s);
			row("UTFW", label, res);
		}
		String[] reads = {
			"0000", "0003616263", "0002c080", "0001c0", "0002c041", "0003e08080", "0003e080", "0003e0c080", "000180", "0001ff", "0001f0",
			"0003eda080", "0006eda0bdedb880", "0002c1bf", "000100", "0004616263", "00", "", "0002e0", "0005e4b8ade4b8"
		};
		for (String h : reads) {
			String res;
			try {
				String s = in(unhex(h)).readUTF();
				res = "OK\t" + esc(s);
			} catch (Throwable e) {
				res = exc(e);
			}
			row("UTFR", h, res);
		}
		for (int bits : new int[] {0x7f800001, 0xffc00000, 0x7fc00000, 0xff800001, 0x7fffffff, 0x80000000, 0x3f800000}) {
			ByteArrayOutputStream bos = new ByteArrayOutputStream();
			new DataOutputStream(bos).writeFloat(Float.intBitsToFloat(bits));
			float back = in(String.format("%08x", bits).length() == 8 ? unhex(String.format("%08x", bits)) : null).readFloat();
			row("F32", String.format("%08x", bits), hex(bos.toByteArray()), Integer.toHexString(Float.floatToRawIntBits(back)));
		}
		for (long bits : new long[] {0x7ff0000000000001L, 0xfff8000000000000L, 0x7ff8000000000000L, 0xfff0000000000001L, 0x3ff0000000000000L, 0x8000000000000000L}) {
			ByteArrayOutputStream bos = new ByteArrayOutputStream();
			new DataOutputStream(bos).writeDouble(Double.longBitsToDouble(bits));
			double back = in(unhex(String.format("%016x", bits))).readDouble();
			row("F64", String.format("%016x", bits), hex(bos.toByteArray()), Long.toHexString(Double.doubleToRawLongBits(back)));
		}
		for (int[] sk : new int[][] {{5, 3}, {5, 5}, {5, 9}, {5, 0}, {5, -1}, {0, 4}}) {
			byte[] b = new byte[sk[0]];
			for (int i = 0; i < b.length; i++) {
				b[i] = (byte) (i + 1);
			}
			DataInputStream d = in(b);
			int n = d.skipBytes(sk[1]);
			String next;
			try {
				next = Integer.toString(d.readUnsignedByte());
			} catch (Throwable e) {
				next = e.getClass().getName();
			}
			row("SKIP", sk[0] + ":" + sk[1], Integer.toString(n), next);
		}
		String[] prim = {"readInt:010203", "readShort:01", "readLong:01020304050607", "readByte:", "readUnsignedShort:ff", "readFully3:0102"};
		for (String p : prim) {
			String[] parts = p.split(":", -1);
			DataInputStream d = in(unhex(parts[1]));
			String res;
			try {
				switch (parts[0]) {
					case "readInt" -> d.readInt();
					case "readShort" -> d.readShort();
					case "readLong" -> d.readLong();
					case "readByte" -> d.readByte();
					case "readUnsignedShort" -> d.readUnsignedShort();
					default -> d.readFully(new byte[3]);
				}
				res = "OK";
			} catch (Throwable e) {
				res = exc(e);
			}
			row("PRIMIO", p, res);
		}
		ByteArrayOutputStream bos = new ByteArrayOutputStream();
		DataOutputStream d = new DataOutputStream(bos);
		d.writeShort(0x12345);
		d.writeChar(-2);
		d.writeByte(0x1ff);
		d.writeBoolean(true);
		d.writeBytes("AŁ￿");
		d.writeChars("AŁ");
		d.writeInt(-2);
		d.writeLong(0x0102030405060708L);
		row("OUT", "mixed", hex(bos.toByteArray()), Integer.toString(d.size()));
	}

	static void hashMapSection() throws Exception {
		List<Object[]> cases = new ArrayList<>();
		// case name, ops (each "P:key", "R:key", "C:", "K:")
		List<String> ops = new ArrayList<>();
		for (int i = 0; i < 100; i++) {
			ops.add("P:k" + i);
		}
		cases.add(new Object[] {"seq_k100", new ArrayList<>(ops)});
		ops.clear();
		for (int i = 0; i < 13; i++) {
			ops.add("P:k" + i);
		}
		cases.add(new Object[] {"seq_k13", new ArrayList<>(ops)});
		ops.clear();
		Random r = new Random(5);
		for (int i = 0; i < 300; i++) {
			int len = 1 + r.nextInt(6);
			StringBuilder b = new StringBuilder();
			for (int j = 0; j < len; j++) {
				b.append((char) (0x20 + r.nextInt(0x5f)));
			}
			ops.add("P:" + b);
			if (r.nextInt(5) == 0) {
				ops.add("R:" + ops.get(r.nextInt(ops.size())).substring(2));
			}
		}
		cases.add(new Object[] {"random300", new ArrayList<>(ops)});
		for (int n : new int[] {8, 9, 10, 11, 12, 20, 40, 64}) {
			ops.clear();
			for (String k : colliding(n)) {
				ops.add("P:" + k);
			}
			cases.add(new Object[] {"colliding" + n, new ArrayList<>(ops)});
		}
		ops.clear();
		for (String k : colliding(20)) {
			ops.add("P:" + k);
		}
		for (int i = 0; i < 40; i++) {
			ops.add("P:z" + i);
		}
		for (String k : colliding(14)) {
			ops.add("R:" + k);
		}
		cases.add(new Object[] {"colliding20_mixed_removed", new ArrayList<>(ops)});
		// Keys whose hash codes agree in the low 16 bits: only h ^ (h >>> 16) separates them.
		ops.clear();
		Random hr = new Random(9);
		int found = 0;
		while (found < 40) {
			StringBuilder b = new StringBuilder();
			for (int j = 0; j < 4; j++) {
				b.append((char) ('a' + hr.nextInt(26)));
			}
			if ((b.toString().hashCode() & 0xffff) == 0x1234) {
				ops.add("P:" + b);
				found++;
			}
		}
		cases.add(new Object[] {"low16_equal", new ArrayList<>(ops)});
		ops.clear();
		for (int i = 0; i < 60; i++) {
			ops.add("P:k" + i);
		}
		for (int i = 0; i < 55; i++) {
			ops.add("R:k" + i);
		}
		ops.add("K:");
		for (int i = 0; i < 5; i++) {
			ops.add("P:n" + i);
		}
		cases.add(new Object[] {"grow_remove_copy", new ArrayList<>(ops)});
		ops.clear();
		for (int i = 0; i < 30; i++) {
			ops.add("P:k" + i);
		}
		ops.add("C:");
		for (int i = 0; i < 5; i++) {
			ops.add("P:k" + (i * 7));
		}
		cases.add(new Object[] {"clear_keeps_table", new ArrayList<>(ops)});
		ops.clear();
		for (int i = 0; i < 20; i++) {
			ops.add("P:k" + i);
		}
		ops.add("R:k3");
		ops.add("P:k3");
		ops.add("P:k5");
		ops.add("R:nope");
		cases.add(new Object[] {"reinsert", new ArrayList<>(ops)});
		for (int s : new int[] {1, 3, 6, 7, 12, 13, 24, 25, 48, 49, 96, 97, 192, 193}) {
			ops.clear();
			for (int i = 0; i < s; i++) {
				ops.add("P:c" + i);
			}
			ops.add("K:");
			cases.add(new Object[] {"copy" + s, new ArrayList<>(ops)});
		}
		for (Object[] cs : cases) {
			String name = (String) cs[0];
			@SuppressWarnings("unchecked")
			List<String> list = (List<String>) cs[1];
			HashMap<String, Integer> m = new HashMap<>();
			int idx = 0;
			for (String op : list) {
				String key = op.substring(2);
				row("HMOP", name, op.substring(0, 1), esc(key));
				switch (op.charAt(0)) {
					case 'P' -> m.put(key, idx);
					case 'R' -> m.remove(key);
					case 'C' -> m.clear();
					default -> m = new HashMap<>(m);
				}
				idx++;
			}
			List<String> f = new ArrayList<>();
			f.add("HMRES");
			f.add(name);
			f.add(Integer.toString(capacity(m)));
			f.add(Integer.toString(m.size()));
			f.add(Integer.toString(m.hashCode()));
			for (Map.Entry<String, Integer> e : m.entrySet()) {
				f.add(esc(e.getKey()) + "=" + e.getValue());
			}
			row(f.toArray(new String[0]));
			HashMap<String, Integer> copy = new HashMap<>(m);
			f.clear();
			f.add("HMCOPY");
			f.add(name);
			f.add(Integer.toString(capacity(copy)));
			for (String k : copy.keySet()) {
				f.add(esc(k));
			}
			row(f.toArray(new String[0]));
		}
		// Integer keys, to cover a non-String hashCode.
		HashMap<Integer, Integer> im = new HashMap<>();
		Random ir = new Random(11);
		List<String> f = new ArrayList<>();
		f.add("HMINT");
		for (int i = 0; i < 50; i++) {
			int k = ir.nextInt();
			im.put(k, i);
			f.add(Integer.toString(k));
		}
		row(f.toArray(new String[0]));
		f.clear();
		f.add("HMINTRES");
		f.add(Integer.toString(capacity(im)));
		for (Integer k : im.keySet()) {
			f.add(Integer.toString(k));
		}
		row(f.toArray(new String[0]));
	}

	// ------------------------------------------------------------------ session 23: HashMap step logs

	/** One HashMap op: P put, R remove, G get, C clear, K copy constructor. */
	record HmOp(char op, Object key, int value) {}

	static String hmKey(Object k) {
		return k instanceof String s ? esc(s) : k.toString();
	}

	/** Replays {@code ops} on a fresh map; one HMSTEP row per op, with the whole state after it. */
	@SuppressWarnings({"unchecked", "rawtypes"})
	static void hmSteps(String name, String keyType, List<HmOp> ops) throws Exception {
		HashMap m = new HashMap<>();
		for (int step = 0; step < ops.size(); step++) {
			HmOp o = ops.get(step);
			Object ret;
			switch (o.op()) {
				case 'P' -> ret = m.put(o.key(), o.value());
				case 'R' -> ret = m.remove(o.key());
				case 'G' -> ret = m.get(o.key());
				case 'C' -> { m.clear(); ret = null; }
				default -> { m = new HashMap<>(m); ret = null; }
			}
			List<String> f = new ArrayList<>();
			f.add("HMSTEP");
			f.add(name);
			f.add(keyType);
			f.add(Integer.toString(step));
			f.add(String.valueOf(o.op()));
			f.add(o.key() == null ? "-" : hmKey(o.key()));
			f.add(Integer.toString(o.value()));
			f.add(ret == null ? "\\N" : ret.toString());
			f.add(Integer.toString(capacity(m)));
			f.add(Integer.toString(m.size()));
			f.add(Integer.toString(m.hashCode()));
			for (Object e : m.entrySet()) {
				Map.Entry me = (Map.Entry) e;
				f.add(hmKey(me.getKey()) + "=" + me.getValue());
			}
			row(f.toArray(new String[0]));
		}
	}

	/** Keys prefix+0, prefix+1, ... whose spread hash lands in bucket {@code b} of a 64-slot table. */
	static List<String> bucketKeys(String prefix, int b, int n) {
		List<String> out = new ArrayList<>();
		for (int i = 0; out.size() < n; i++) {
			String s = prefix + i;
			int h = s.hashCode();
			if (((h ^ (h >>> 16)) & 63) == b) {
				out.add(s);
			}
		}
		return out;
	}

	/** Keys prefix+0, prefix+1, ... with {@code (spread hash & mask) == b}, skipping ones used before. */
	static final java.util.Set<String> SPREAD_USED = new java.util.HashSet<>();

	static List<String> spreadKeys(String prefix, int b, int mask, int n) {
		List<String> out = new ArrayList<>();
		for (int i = 0; out.size() < n; i++) {
			String s = prefix + i;
			int h = s.hashCode();
			if (((h ^ (h >>> 16)) & mask) == b && SPREAD_USED.add(s)) {
				out.add(s);
			}
		}
		return out;
	}

	static void hashMapStepSection() throws Exception {
		// 1. tree_grow: 48 keys share bucket 5 of every table up to 64 -> resize, resize, treeify;
		// fillers then grow the table to 128 and 256, splitting the tree into lo/hi trees;
		// removals take the trees apart (successor swaps, rebalancing, untreeify when too small);
		// re-puts rebuild lists into trees; more fillers grow to 512 and untreeify the halves.
		List<HmOp> ops = new ArrayList<>();
		List<String> b5 = bucketKeys("t", 5, 48);
		int v = 0;
		for (String k : b5) {
			ops.add(new HmOp('P', k, v++));
		}
		for (int i = 0; i < 102; i++) {
			ops.add(new HmOp('P', "f" + i, v++));
		}
		Random r = new Random(23);
		List<String> order = new ArrayList<>(b5);
		java.util.Collections.shuffle(order, r);
		for (int i = 0; i < 40; i++) {
			ops.add(new HmOp('R', order.get(i), 0));
			if (i % 4 == 0) {
				ops.add(new HmOp('G', order.get(r.nextInt(order.size())), 0));
				ops.add(new HmOp('P', order.get(40 + r.nextInt(8)), v++));
			}
		}
		ops.add(new HmOp('R', "nope", 0));
		for (int i = 0; i < 30; i++) {
			ops.add(new HmOp('P', order.get(i), v++));
		}
		for (int i = 102; i < 160; i++) {
			ops.add(new HmOp('P', "f" + i, v++));
		}
		ops.add(new HmOp('K', null, 0));
		for (int i = 0; i < 12; i++) {
			ops.add(new HmOp('G', order.get(i * 4), 0));
		}
		hmSteps("tree_grow", "S", ops);

		// 2. same_hash: 32 keys with one String.hashCode -- one bucket at every capacity, so the
		// tree is ordered by compareTo alone. Remove in a shuffled order down to empty (gets and
		// value replacements mixed in), refill in another order, copy, clear, refill, copy.
		ops = new ArrayList<>();
		String[] col = colliding(32);
		v = 0;
		for (String k : col) {
			ops.add(new HmOp('P', k, v++));
		}
		List<String> co = new ArrayList<>(java.util.Arrays.asList(col));
		java.util.Collections.shuffle(co, new Random(5));
		for (int i = 0; i < co.size(); i++) {
			ops.add(new HmOp('R', co.get(i), 0));
			if (i % 3 == 0 && i + 1 < co.size()) {
				ops.add(new HmOp('P', co.get(co.size() - 1), v++));
				ops.add(new HmOp('G', co.get(i), 0));
				ops.add(new HmOp('G', co.get(i + 1), 0));
			}
		}
		java.util.Collections.shuffle(co, new Random(6));
		for (int i = 0; i < 20; i++) {
			ops.add(new HmOp('P', co.get(i), v++));
		}
		ops.add(new HmOp('K', null, 0));
		ops.add(new HmOp('C', null, 0));
		for (int i = 0; i < 12; i++) {
			ops.add(new HmOp('P', co.get(31 - i), v++));
		}
		ops.add(new HmOp('K', null, 0));
		hmSteps("same_hash", "S", ops);

		// 3. int_keys: Integer keys, so hashCode is the value itself -- multiples of 64 collide in
		// bucket 0 until the table outgrows them; negatives and extremes spread differently.
		ops = new ArrayList<>();
		v = 0;
		for (int k = 0; k < 24; k++) {
			ops.add(new HmOp('P', k * 64, v++));
		}
		for (int k : new int[] {Integer.MIN_VALUE, Integer.MAX_VALUE, -1, -64, -65536, 65536, 0x10000 * 3, 0x7fff0000, 1 << 30}) {
			ops.add(new HmOp('P', k, v++));
		}
		for (int k = 0; k < 60; k++) {
			ops.add(new HmOp('P', (k << 16) | 5, v++));
		}
		Random ir = new Random(77);
		for (int i = 0; i < 40; i++) {
			int k = ir.nextInt(3) == 0 ? (ir.nextInt(60) << 16) | 5 : ir.nextInt(24) * 64;
			ops.add(new HmOp(ir.nextBoolean() ? 'R' : 'P', k, v++));
		}
		ops.add(new HmOp('K', null, 0));
		hmSteps("int_keys", "I", ops);

		// 4. split_boundary: four tree buckets of a 64 table that the resize to 128 splits by bit 6
		// into 6|7, 7|6, 10|0 and 0|10 -- UNTREEIFY_THRESHOLD is 6, so each side lands on or just
		// past the untreeify boundary, and 10|0 / 0|10 keep their tree without rebuilding it.
		// Whether a rebuilt tree's root is already the bucket's head depends on insertion order,
		// so the same buckets run in natural, reversed and four shuffled orders.
		int[][] buckets = {{5, 6}, {69, 7}, {9, 7}, {73, 6}, {13, 10}, {81, 10}};
		List<List<String>> groups = new ArrayList<>();
		for (int[] bk : buckets) {
			groups.add(spreadKeys("s", bk[0], 127, bk[1]));
		}
		// After the split, three more keys into each boundary bucket: a tree bin links a new node
		// after its tree parent, a list appends it, so this is where tree-vs-list becomes visible.
		List<String> after = new ArrayList<>();
		for (int b : new int[] {5, 69, 9, 73, 13, 81}) {
			after.addAll(spreadKeys("w", b, 127, 3));
		}
		for (int mode = 0; mode < 6; mode++) {
			ops = new ArrayList<>();
			v = 0;
			for (List<String> g : groups) {
				List<String> keys = new ArrayList<>(g);
				if (mode == 1) {
					java.util.Collections.reverse(keys);
				} else if (mode >= 2) {
					java.util.Collections.shuffle(keys, new Random(mode));
				}
				for (String k : keys) {
					ops.add(new HmOp('P', k, v++));
				}
			}
			for (int i = 0; i < 3; i++) {
				ops.add(new HmOp('P', "u" + i, v++));
			}
			for (String k : after) {
				ops.add(new HmOp('P', k, v++));
			}
			hmSteps("split_boundary_" + mode, "S", ops);
		}

		// 4. HMEQ: AbstractMap.equals / hashCode across histories that end with equal or unequal
		// contents. Each side is an op script "P:k=v" / "R:k" joined by ','.
		String[][] pairs = {
			{"same_order", "P:a=1,P:b=2,P:c=3", "P:a=1,P:b=2,P:c=3"},
			{"other_order", "P:a=1,P:b=2,P:c=3", "P:c=3,P:a=1,P:b=2"},
			{"value_differs", "P:a=1,P:b=2", "P:a=1,P:b=3"},
			{"size_differs", "P:a=1,P:b=2", "P:a=1"},
			{"key_differs", "P:a=1,P:b=2", "P:a=1,P:c=2"},
			{"both_empty", "", "P:a=1,R:a"},
			{"replaced", "P:a=1,P:a=2", "P:a=2"},
		};
		for (String[] pr : pairs) {
			HashMap<String, Integer> a = hmScript(pr[1]);
			HashMap<String, Integer> b = hmScript(pr[2]);
			row("HMEQ", pr[0], pr[1].isEmpty() ? "-" : pr[1], pr[2].isEmpty() ? "-" : pr[2], Boolean.toString(a.equals(b)),
				Boolean.toString(b.equals(a)), Integer.toString(a.hashCode()), Integer.toString(b.hashCode()));
		}
		// The same 30 colliding keys; one map grown to a 512 table and emptied back down: equal.
		StringBuilder big = new StringBuilder();
		StringBuilder small = new StringBuilder();
		for (String k : colliding(30)) {
			big.append(big.length() == 0 ? "" : ",").append("P:").append(k).append("=1");
			small.append(small.length() == 0 ? "" : ",").append("P:").append(k).append("=1");
		}
		for (int i = 0; i < 200; i++) {
			big.append(",P:x").append(i).append("=0");
		}
		for (int i = 0; i < 200; i++) {
			big.append(",R:x").append(i);
		}
		HashMap<String, Integer> a = hmScript(big.toString());
		HashMap<String, Integer> b = hmScript(small.toString());
		row("HMEQ", "tree_vs_regrown", big.toString(), small.toString(), Boolean.toString(a.equals(b)), Boolean.toString(b.equals(a)),
			Integer.toString(a.hashCode()), Integer.toString(b.hashCode()));
	}

	static HashMap<String, Integer> hmScript(String script) {
		HashMap<String, Integer> m = new HashMap<>();
		if (script.isEmpty()) {
			return m;
		}
		for (String op : script.split(",")) {
			if (op.startsWith("P:")) {
				String[] kv = op.substring(2).split("=");
				m.put(kv[0], Integer.parseInt(kv[1]));
			} else {
				m.remove(op.substring(2));
			}
		}
		return m;
	}

	// ------------------------------------------------------------------ session 23: containers

	/**
	 * A tag as a literal the Rust tests parse back: {@code E}, {@code b<n>} {@code s<n>} {@code i<n>}
	 * {@code l<n>}, {@code f<raw bits hex>} {@code d<raw bits hex>}, {@code S"<esc>"}, {@code B(..)}
	 * {@code I(..)} {@code L(..)}, {@code [a,b]}, and {@code {"k":v,...}} in ITERATION order. Inside
	 * quotes the text is {@link #esc}aped and a double quote is written as {@code "}.
	 */
	static String lit(Tag t) {
		if (t instanceof EndTag) {
			return "E";
		} else if (t instanceof ByteTag b) {
			return "b" + b.value();
		} else if (t instanceof ShortTag s) {
			return "s" + s.value();
		} else if (t instanceof IntTag i) {
			return "i" + i.value();
		} else if (t instanceof LongTag l) {
			return "l" + l.value();
		} else if (t instanceof FloatTag f) {
			return "f" + Integer.toHexString(Float.floatToRawIntBits(f.value()));
		} else if (t instanceof DoubleTag d) {
			return "d" + Long.toHexString(Double.doubleToRawLongBits(d.value()));
		} else if (t instanceof StringTag(String v)) {
			return "S" + qesc(v);
		} else if (t instanceof ByteArrayTag a) {
			StringBuilder b = new StringBuilder("B(");
			byte[] d = a.getAsByteArray();
			for (int i = 0; i < d.length; i++) {
				b.append(i == 0 ? "" : ",").append(d[i]);
			}
			return b.append(')').toString();
		} else if (t instanceof IntArrayTag a) {
			StringBuilder b = new StringBuilder("I(");
			int[] d = a.getAsIntArray();
			for (int i = 0; i < d.length; i++) {
				b.append(i == 0 ? "" : ",").append(d[i]);
			}
			return b.append(')').toString();
		} else if (t instanceof LongArrayTag a) {
			StringBuilder b = new StringBuilder("L(");
			long[] d = a.getAsLongArray();
			for (int i = 0; i < d.length; i++) {
				b.append(i == 0 ? "" : ",").append(d[i]);
			}
			return b.append(')').toString();
		} else if (t instanceof ListTag l) {
			StringBuilder b = new StringBuilder("[");
			for (int i = 0; i < l.size(); i++) {
				b.append(i == 0 ? "" : ",").append(lit(l.get(i)));
			}
			return b.append(']').toString();
		} else {
			CompoundTag c = (CompoundTag) t;
			StringBuilder b = new StringBuilder("{");
			boolean first = true;
			for (Map.Entry<String, Tag> e : c.entrySet()) {
				b.append(first ? "" : ",").append(qesc(e.getKey())).append(':').append(lit(e.getValue()));
				first = false;
			}
			return b.append('}').toString();
		}
	}

	static String qesc(String s) {
		return "\"" + esc(s).replace("\"", "\\u0022") + "\"";
	}

	static boolean hasEnd(Tag t) {
		if (t instanceof EndTag) {
			return true;
		} else if (t instanceof ListTag l) {
			for (Tag e : l) {
				if (hasEnd(e)) {
					return true;
				}
			}
		} else if (t instanceof CompoundTag c) {
			for (Tag e : c.values()) {
				if (hasEnd(e)) {
					return true;
				}
			}
		}
		return false;
	}

	/** hashCode, or "-" where it folds in EndTag's identity hash. */
	static String hashOf(Tag t) {
		return hasEnd(t) ? "-" : Integer.toString(t.hashCode());
	}

	static String hexWrite(Tag t) {
		ByteArrayOutputStream bos = new ByteArrayOutputStream();
		try {
			t.write(new DataOutputStream(bos));
			return hex(bos.toByteArray());
		} catch (Throwable e) {
			return "EX:" + e.getClass().getName();
		}
	}

	static String exText(Throwable e) {
		return "EX:" + e.getClass().getName() + ":" + esc(e.getMessage());
	}

	static String optStr(java.util.Optional<?> o) {
		return o.map(Object::toString).orElse("-");
	}

	/** Every CompoundTag getter for one key, in a fixed order. */
	static String compoundGetters(CompoundTag c, String k) {
		return String.join(";",
			"B=" + optStr(c.getByte(k)), "BO=" + c.getByteOr(k, (byte) 7),
			"S=" + optStr(c.getShort(k)), "SO=" + c.getShortOr(k, (short) 7),
			"I=" + optStr(c.getInt(k)), "IO=" + c.getIntOr(k, 7),
			"L=" + optStr(c.getLong(k)), "LO=" + c.getLongOr(k, 7L),
			"F=" + c.getFloat(k).map(v -> Integer.toHexString(Float.floatToRawIntBits(v))).orElse("-"),
			"FO=" + Integer.toHexString(Float.floatToRawIntBits(c.getFloatOr(k, 7.0f))),
			"D=" + c.getDouble(k).map(v -> Long.toHexString(Double.doubleToRawLongBits(v))).orElse("-"),
			"DO=" + Long.toHexString(Double.doubleToRawLongBits(c.getDoubleOr(k, 7.0))),
			"STR=" + c.getString(k).map(NbtProbe::esc).orElse("-"), "STRO=" + esc(c.getStringOr(k, "d")),
			"BA=" + c.getByteArray(k).map(a -> Integer.toString(a.length)).orElse("-"),
			"IA=" + c.getIntArray(k).map(a -> Integer.toString(a.length)).orElse("-"),
			"LA=" + c.getLongArray(k).map(a -> Integer.toString(a.length)).orElse("-"),
			"C=" + c.getCompound(k).map(NbtProbe::lit).orElse("-"), "CO=" + lit(c.getCompoundOrEmpty(k)),
			"LI=" + c.getList(k).map(NbtProbe::lit).orElse("-"), "LIO=" + lit(c.getListOrEmpty(k)),
			"BOOL=" + optStr(c.getBoolean(k)), "BOOLO0=" + c.getBooleanOr(k, false), "BOOLO1=" + c.getBooleanOr(k, true),
			"HAS=" + c.contains(k), "GET=" + (c.get(k) == null ? "-" : lit(c.get(k))));
	}

	/** Every ListTag getter for one index, in a fixed order. */
	static String listGetters(ListTag l, int i) {
		return String.join(";",
			"C=" + l.getCompound(i).map(NbtProbe::lit).orElse("-"), "CO=" + lit(l.getCompoundOrEmpty(i)),
			"LI=" + l.getList(i).map(NbtProbe::lit).orElse("-"), "LIO=" + lit(l.getListOrEmpty(i)),
			"S=" + optStr(l.getShort(i)), "SO=" + l.getShortOr(i, (short) 7),
			"I=" + optStr(l.getInt(i)), "IO=" + l.getIntOr(i, 7),
			"IA=" + l.getIntArray(i).map(a -> Integer.toString(a.length)).orElse("-"),
			"LA=" + l.getLongArray(i).map(a -> Integer.toString(a.length)).orElse("-"),
			"D=" + l.getDouble(i).map(v -> Long.toHexString(Double.doubleToRawLongBits(v))).orElse("-"),
			"DO=" + Long.toHexString(Double.doubleToRawLongBits(l.getDoubleOr(i, 7.0))),
			"F=" + l.getFloat(i).map(v -> Integer.toHexString(Float.floatToRawIntBits(v))).orElse("-"),
			"FO=" + Integer.toHexString(Float.floatToRawIntBits(l.getFloatOr(i, 7.0f))),
			"STR=" + l.getString(i).map(NbtProbe::esc).orElse("-"), "STRO=" + esc(l.getStringOr(i, "d")));
	}

	/** One container state: literal, write bytes, toString, hashCode, sizeInBytes, size. */
	static List<String> state(Tag t) throws Exception {
		List<String> f = new ArrayList<>();
		f.add(lit(t));
		f.add(hexWrite(t));
		f.add(esc(t.toString()));
		f.add(hashOf(t));
		f.add(Integer.toString(t.sizeInBytes()));
		f.add(Integer.toString(t instanceof ListTag l ? l.size() : ((CompoundTag) t).size()));
		if (t instanceof ListTag l) {
			f.add(Byte.toString((Byte) IDENTIFY.invoke(l)));
		} else {
			f.add("-");
		}
		return f;
	}

	static ListTag listOf(Tag... tags) {
		ListTag l = new ListTag();
		for (Tag t : tags) {
			l.add(t);
		}
		return l;
	}

	static CompoundTag compoundOf(Object... kv) {
		CompoundTag c = new CompoundTag();
		for (int i = 0; i < kv.length; i += 2) {
			c.put((String) kv[i], (Tag) kv[i + 1]);
		}
		return c;
	}

	static void ctRow(String name, int step, String op, String arg, String ret, CompoundTag c) throws Exception {
		List<String> f = new ArrayList<>();
		f.add("CTOPS");
		f.add(name);
		f.add(Integer.toString(step));
		f.add(op);
		f.add(arg);
		f.add(ret);
		f.addAll(state(c));
		row(f.toArray(new String[0]));
	}

	static void ltRow(String name, int step, String op, String arg, String ret, ListTag l) throws Exception {
		List<String> f = new ArrayList<>();
		f.add("LTOPS");
		f.add(name);
		f.add(Integer.toString(step));
		f.add(op);
		f.add(arg);
		f.add(ret);
		f.addAll(state(l));
		row(f.toArray(new String[0]));
	}

	static void containerSection() throws Exception {
		// TEQ: Tag.equals matrix and hashCode. Records compare floats with Float.compare.
		Tag[] eq = {
			new FloatTag(0.0f), new FloatTag(-0.0f), new FloatTag(Float.NaN), new FloatTag(Float.intBitsToFloat(0x7f800001)),
			new FloatTag(1.0f), new DoubleTag(0.0), new DoubleTag(-0.0), new DoubleTag(Double.NaN),
			new DoubleTag(Double.longBitsToDouble(0xfff0000000000001L)), new DoubleTag(1.0), ByteTag.valueOf((byte) 1), IntTag.valueOf(1),
			StringTag.valueOf("1"), compoundOf("a", new FloatTag(0.0f)), compoundOf("a", new FloatTag(-0.0f)),
			compoundOf("a", new FloatTag(Float.NaN)), compoundOf("a", new FloatTag(Float.intBitsToFloat(0x7fc00001))),
			listOf(new DoubleTag(Double.NaN)), listOf(new DoubleTag(Double.longBitsToDouble(0x7ff0000000000002L))),
			listOf(IntTag.valueOf(1), IntTag.valueOf(2)), listOf(IntTag.valueOf(2), IntTag.valueOf(1)),
			compoundOf("a", IntTag.valueOf(1), "b", IntTag.valueOf(2)), compoundOf("b", IntTag.valueOf(2), "a", IntTag.valueOf(1)),
			new ListTag(), new CompoundTag(), new ByteArrayTag(new byte[] {1}), new IntArrayTag(new int[] {1}),
			listOf(compoundOf("", IntTag.valueOf(1))), listOf(IntTag.valueOf(1)), EndTag.INSTANCE, listOf(EndTag.INSTANCE)
		};
		for (int i = 0; i < eq.length; i++) {
			StringBuilder bits = new StringBuilder();
			for (Tag other : eq) {
				bits.append(eq[i].equals(other) ? '1' : '0');
			}
			row("TEQ", Integer.toString(i), lit(eq[i]), hashOf(eq[i]), bits.toString());
		}

		// CTOPS: a CompoundTag through every mutator and getter, with the full state after each op.
		CompoundTag c = new CompoundTag();
		int step = 0;
		String name = "compound";
		Tag[] values = {
			ByteTag.valueOf((byte) -1), ShortTag.valueOf((short) 300), IntTag.valueOf(-129), LongTag.valueOf(0x100L),
			LongTag.valueOf(0x100000080L), new FloatTag(-1.5f), new FloatTag(3.0e9f), new FloatTag(Float.NaN), new DoubleTag(1e10),
			new DoubleTag(-1e30), new DoubleTag(Double.NaN), StringTag.valueOf("txt"), new ByteArrayTag(new byte[] {1, 2}),
			new IntArrayTag(new int[] {3}), new LongArrayTag(new long[] {4, 5, 6}), listOf(IntTag.valueOf(1)),
			compoundOf("in", IntTag.valueOf(2)), new ListTag(), new CompoundTag(), ByteTag.valueOf((byte) 0)
		};
		String[] keys = {"b", "s", "i", "l", "l2", "f", "f2", "fnan", "d", "d2", "dnan", "str", "ba", "ia", "la", "lst", "cmp", "el", "ec", "zero"};
		for (int i = 0; i < values.length; i++) {
			Tag prev = c.put(keys[i], values[i]);
			ctRow(name, step++, "put", qesc(keys[i]) + "=" + lit(values[i]), prev == null ? "\\N" : lit(prev), c);
		}
		for (String k : new String[] {"b", "s", "i", "l", "l2", "f", "f2", "fnan", "d", "d2", "dnan", "str", "ba", "ia", "la", "lst", "cmp", "el", "ec", "zero", "missing"}) {
			ctRow(name, step++, "getters", qesc(k), esc(compoundGetters(c, k)), c);
		}
		c.putByte("pb", (byte) 0x7f);
		ctRow(name, step++, "putByte", qesc("pb") + "=127", "-", c);
		c.putShort("ps", (short) -32768);
		ctRow(name, step++, "putShort", qesc("ps") + "=-32768", "-", c);
		c.putInt("pi", Integer.MIN_VALUE);
		ctRow(name, step++, "putInt", qesc("pi") + "=-2147483648", "-", c);
		c.putLong("pl", Long.MAX_VALUE);
		ctRow(name, step++, "putLong", qesc("pl") + "=9223372036854775807", "-", c);
		c.putFloat("pf", -0.0f);
		ctRow(name, step++, "putFloat", qesc("pf") + "=80000000", "-", c);
		c.putDouble("pd", -0.0);
		ctRow(name, step++, "putDouble", qesc("pd") + "=8000000000000000", "-", c);
		c.putString("pstr", "q\"u");
		ctRow(name, step++, "putString", qesc("pstr") + "=" + qesc("q\"u"), "-", c);
		c.putByteArray("pba", new byte[] {-128, 127});
		ctRow(name, step++, "putByteArray", qesc("pba") + "=-128,127", "-", c);
		c.putIntArray("pia", new int[] {});
		ctRow(name, step++, "putIntArray", qesc("pia") + "=", "-", c);
		c.putLongArray("pla", new long[] {Long.MIN_VALUE});
		ctRow(name, step++, "putLongArray", qesc("pla") + "=-9223372036854775808", "-", c);
		c.putBoolean("pt", true);
		ctRow(name, step++, "putBoolean", qesc("pt") + "=true", "-", c);
		c.putBoolean("pfalse", false);
		ctRow(name, step++, "putBoolean", qesc("pfalse") + "=false", "-", c);
		for (String k : new String[] {"i", "missing", "cmp", "pl"}) {
			Tag removed = c.remove(k);
			ctRow(name, step++, "remove", qesc(k), removed == null ? "\\N" : lit(removed), c);
		}
		Tag prev = c.put("b", StringTag.valueOf("replaced"));
		ctRow(name, step++, "put", qesc("b") + "=" + lit(StringTag.valueOf("replaced")), prev == null ? "\\N" : lit(prev), c);
		// merge: a compound into a compound merges; anything else replaces with a copy.
		CompoundTag src = compoundOf("lst", compoundOf("x", IntTag.valueOf(9)), "ec", compoundOf("m", IntTag.valueOf(1)),
			"new", listOf(IntTag.valueOf(3)), "str", IntTag.valueOf(5));
		c.merge(src);
		ctRow(name, step++, "merge", lit(src), "-", c);
		// copy isolation: mutate the original's nested children; the copy must not change.
		CompoundTag copy = c.copy();
		c.getCompound("ec").ifPresent(e -> e.putInt("after", 1));
		c.getList("new").ifPresent(l -> l.add(IntTag.valueOf(4)));
		ctRow(name, step++, "mutated-original", "-", "-", c);
		ctRow(name, step++, "copy", "-", Boolean.toString(copy.equals(c)), copy);
		ctRow(name, step++, "shallowCopy", "-", "-", (CompoundTag) SHALLOW_COPY.invoke(c));
		// A compound that grew and shrank: copy() re-inserts into a smaller table.
		CompoundTag grown = new CompoundTag();
		for (int i = 0; i < 100; i++) {
			grown.putInt("g" + i, i);
		}
		for (int i = 0; i < 90; i++) {
			grown.remove("g" + i);
		}
		ctRow("grown", 0, "grow-shrink", "100-90", "-", grown);
		ctRow("grown", 1, "copy", "-", Boolean.toString(grown.copy().equals(grown)), grown.copy());
		ctRow("grown", 2, "shallowCopy", "-", "-", (CompoundTag) SHALLOW_COPY.invoke(grown));
		// copy() of a compound HOLDING that one: the nested copy re-inserts too.
		CompoundTag outer = new CompoundTag();
		outer.put("g", grown);
		outer.putInt("z", 1);
		ctRow("grown", 3, "nested-copy", "-", "-", outer.copy());
		// merge into a compound that already has content, two levels deep.
		CompoundTag base = compoundOf("keep", compoundOf("a", IntTag.valueOf(1), "deep", compoundOf("x", IntTag.valueOf(1), "y", IntTag.valueOf(2))),
			"other", IntTag.valueOf(5), "list", listOf(IntTag.valueOf(1)));
		ctRow("merge2", 0, "base", lit(base), "-", base);
		CompoundTag src2 = compoundOf("keep", compoundOf("b", IntTag.valueOf(2), "deep", compoundOf("y", IntTag.valueOf(3), "z", IntTag.valueOf(4))),
			"other", compoundOf("now", StringTag.valueOf("compound")), "list", listOf(IntTag.valueOf(2)));
		base.merge(src2);
		ctRow("merge2", 1, "merge", lit(src2), "-", base);

		// LTOPS: a ListTag through every mutator and getter.
		ListTag l = new ListTag();
		name = "list";
		step = 0;
		ltRow(name, step++, "new", "-", "-", l);
		Tag[] adds = {IntTag.valueOf(1), IntTag.valueOf(2), StringTag.valueOf("mixed"), compoundOf("x", IntTag.valueOf(1)),
			compoundOf("", ByteTag.valueOf((byte) 5)), new ListTag(), new DoubleTag(-2.5), new FloatTag(3.75f), new IntArrayTag(new int[] {9, 8}),
			new LongArrayTag(new long[] {7}), ShortTag.valueOf((short) -300), EndTag.INSTANCE};
		for (Tag t : adds) {
			ltRow(name, step++, "add", lit(t), Boolean.toString(l.add(t)), l);
		}
		for (int i = -1; i <= l.size(); i++) {
			ltRow(name, step++, "getters", Integer.toString(i), esc(listGetters(l, i)), l);
		}
		String[][] edits = {
			{"set", "0", "S\"first\""}, {"set", "12", "i0"}, {"set", "-1", "i0"}, {"remove", "11", ""}, {"remove", "11", ""},
			{"addAt", "0", "l64"}, {"addAt", "12", "i99"}, {"addAt", "14", "i0"}, {"addAt", "-1", "i0"},
			{"setTag", "1", "b1"}, {"setTag", "13", "b1"}, {"addTag", "13", "s2"}, {"addTag", "15", "s2"},
			{"get", "4", ""}, {"get", "14", ""}, {"get", "-1", ""}
		};
		for (String[] e : edits) {
			int idx = Integer.parseInt(e[1]);
			String ret;
			try {
				switch (e[0]) {
					case "set" -> ret = lit(l.set(idx, parseLit(e[2])));
					case "remove" -> ret = lit(l.remove(idx));
					case "addAt" -> { l.add(idx, parseLit(e[2])); ret = "-"; }
					case "setTag" -> ret = Boolean.toString(l.setTag(idx, parseLit(e[2])));
					case "addTag" -> ret = Boolean.toString(l.addTag(idx, parseLit(e[2])));
					default -> ret = lit(l.get(idx));
				}
			} catch (Throwable ex) {
				ret = exText(ex);
			}
			ltRow(name, step++, e[0], e[1] + (e[2].isEmpty() ? "" : "=" + e[2]), ret, l);
		}
		ListTag lcopy = l.copy();
		l.getCompound(4).ifPresent(cc -> cc.putInt("y", 2));
		ltRow(name, step++, "mutated-original", "-", "-", l);
		ltRow(name, step++, "copy", "-", Boolean.toString(lcopy.equals(l)), lcopy);
		l.clear();
		ltRow(name, step++, "clear", "-", "-", l);
		// addAndUnwrap: a one-entry "" compound unwraps; anything else is added as is.
		Tag[] unwraps = {compoundOf("", IntTag.valueOf(7)), compoundOf("", compoundOf("", IntTag.valueOf(8))), compoundOf("x", IntTag.valueOf(1)),
			compoundOf("", IntTag.valueOf(1), "y", IntTag.valueOf(2)), new CompoundTag(), StringTag.valueOf("s"), compoundOf("", EndTag.INSTANCE)};
		for (Tag t : unwraps) {
			l.addAndUnwrap(t);
			ltRow(name, step++, "addAndUnwrap", lit(t), "-", l);
		}
		// Element-type transitions on write: empty, homogeneous, mixed, back to homogeneous.
		ListTag tr = new ListTag();
		step = 0;
		ltRow("transitions", step++, "new", "-", "-", tr);
		for (Tag t : new Tag[] {compoundOf("a", IntTag.valueOf(1)), compoundOf("", IntTag.valueOf(2)), IntTag.valueOf(3)}) {
			tr.add(t);
			ltRow("transitions", step++, "add", lit(t), "-", tr);
		}
		tr.remove(0);
		ltRow("transitions", step++, "remove", "0", "-", tr);
		tr.remove(0);
		ltRow("transitions", step++, "remove", "0", "-", tr);
		tr.add(0, EndTag.INSTANCE);
		ltRow("transitions", step++, "addAt", "0=E", "-", tr);

		// TACC: acceptAsRoot traces over containers the generated trees never hold -- lists
		// with END elements (whose element type is the first NON-END id), wrappers, nesting.
		String[][] accs = {
			{"end_long", "{\"l\":[E,l5]}"}, {"end_only", "{\"l\":[E,E]}"},
			{"mixed", "{\"m\":[i1,S\"a\",{\"x\":i1},{\"\":b5},[],[i1]]}"},
			{"nested_lists", "{\"n\":[[i1,i2],[S\"s\"],[]]}"}, {"empty", "{}"}, {"list_root", "[i1,{\"a\":i2},E]"},
			{"end_root", "E"}
		};
		for (String[] a : accs) {
			Tag t = parseFullLit(a[1]);
			for (int policy = 0; policy < 3; policy++) {
				Random pr = policy == 0 ? null : new Random(1000L * policy + a[0].hashCode());
				TraceVisitor tv = new TraceVisitor(pr);
				t.acceptAsRoot(tv);
				row("TACC", a[0], Integer.toString(policy), lit(t), esc(tv.log.toString()));
			}
		}

		// TLOAD: TYPE.load / skip / parse of crafted payloads (no root framing).
		String[][] payloads = {
			{"c_empty_input", "10", ""}, {"c_empty", "10", "00"}, {"c_one", "10", "030001610000000100"},
			{"c_dup", "10", "03000161000000010300016100000002" + "00"}, {"c_invalid_child", "10", "6300016100"},
			{"c_child_list_negative", "10", "0900016c03ffffffff00"}, {"c_child_list_missing_type", "10", "0900016c000000000200"},
			{"c_child_list_end_empty", "10", "0900016c000000000000"}, {"c_truncated_value", "10", "0300016100"},
			{"c_truncated_key", "10", "0300"}, {"c_end_child", "10", "0000"},
			{"c_nested", "10", "0a0001610a0001620300016300000007" + "0000" + "00"},
			{"c_wrapped_list", "10", "0900016c0a00000002" + "030000000000070000" + "00" + "00"},
			{"l_ints", "9", "030000000200000001fffffffe"}, {"l_empty", "9", "0000000000"}, {"l_missing_type", "9", "0000000001"},
			{"l_negative", "9", "03ffffffff"}, {"l_negative_static_skip", "9", "03fffffffe"}, {"l_bytes", "9", "0100000003010203"},
			{"l_compounds", "9", "0a00000003" + "030000000000070000" + "030001780000000100"},
			{"l_truncated", "9", "03000000030000000100000002"}, {"l_strings", "9", "080000000200016100026263"},
			{"l_lists", "9", "09000000020100000001ff0000000000"}
		};
		long[][] limits = {{Long.MAX_VALUE, 512}, {100, 512}, {Long.MAX_VALUE, 1}};
		for (String[] p : payloads) {
			for (long[] lim : limits) {
				byte[] in = unhex(p[2]);
				TagType<?> type = TagTypes.getType(Integer.parseInt(p[1]));
				String load;
				NbtAccounter acc = new NbtAccounter(lim[0], (int) lim[1]);
				try {
					Tag t = type.load(in(in), acc);
					load = "OK\t" + lit(t) + "\t" + acc.getUsage() + "\t" + acc.getDepth();
				} catch (Throwable e) {
					load = exc(e);
				}
				String skip;
				acc = new NbtAccounter(lim[0], (int) lim[1]);
				DataInputStream d = in(in);
				try {
					type.skip(d, acc);
					skip = "OK\t" + d.available() + "\t" + acc.getUsage() + "\t" + acc.getDepth();
				} catch (Throwable e) {
					skip = exc(e);
				}
				String parse;
				acc = new NbtAccounter(lim[0], (int) lim[1]);
				TraceVisitor tv = new TraceVisitor(null);
				d = in(in);
				try {
					StreamTagVisitor.ValueResult r = type.parse(d, tv, acc);
					parse = "OK\t" + esc(tv.log.toString()) + "\t" + r.name() + "\t" + d.available() + "\t" + acc.getUsage() + "\t" + acc.getDepth();
				} catch (Throwable e) {
					parse = exc(e) + "\t" + esc(tv.log.toString());
				}
				row("TLOAD", p[0], p[1], Long.toString(lim[0]), Long.toString(lim[1]), p[2], load, skip, parse);
			}
		}
	}

	/** The inverse of {@link #lit}, complete: what the Rust tests parse, parsed the same way. */
	static Tag parseFullLit(String s) {
		int[] at = {0};
		Tag t = parseLitAt(s, at);
		if (at[0] != s.length()) {
			throw new IllegalArgumentException("trailing text in " + s);
		}
		return t;
	}

	static String litWord(String s, int[] at) {
		int start = at[0];
		while (at[0] < s.length() && ",)]}".indexOf(s.charAt(at[0])) < 0) {
			at[0]++;
		}
		return s.substring(start, at[0]);
	}

	static String litQuoted(String s, int[] at) {
		at[0]++;
		int start = at[0];
		while (s.charAt(at[0]) != '"') {
			at[0]++;
		}
		String raw = s.substring(start, at[0]);
		at[0]++;
		StringBuilder b = new StringBuilder();
		for (int i = 0; i < raw.length(); i++) {
			char c = raw.charAt(i);
			if (c == '\\' && raw.charAt(i + 1) == 'u') {
				b.append((char) Integer.parseInt(raw.substring(i + 2, i + 6), 16));
				i += 5;
			} else if (c == '\\') {
				b.append('\\');
				i++;
			} else {
				b.append(c);
			}
		}
		return b.toString();
	}

	static Tag parseLitAt(String s, int[] at) {
		char c = s.charAt(at[0]++);
		switch (c) {
			case 'E':
				return EndTag.INSTANCE;
			case 'b':
				return ByteTag.valueOf(Byte.parseByte(litWord(s, at)));
			case 's':
				return ShortTag.valueOf(Short.parseShort(litWord(s, at)));
			case 'i':
				return IntTag.valueOf(Integer.parseInt(litWord(s, at)));
			case 'l':
				return LongTag.valueOf(Long.parseLong(litWord(s, at)));
			case 'S':
				return StringTag.valueOf(litQuoted(s, at));
			case '[': {
				ListTag l = new ListTag();
				while (s.charAt(at[0]) != ']') {
					l.add(parseLitAt(s, at));
					if (s.charAt(at[0]) == ',') {
						at[0]++;
					}
				}
				at[0]++;
				return l;
			}
			case '{': {
				CompoundTag t = new CompoundTag();
				while (s.charAt(at[0]) != '}') {
					String k = litQuoted(s, at);
					at[0]++;
					t.put(k, parseLitAt(s, at));
					if (s.charAt(at[0]) == ',') {
						at[0]++;
					}
				}
				at[0]++;
				return t;
			}
			default:
				throw new IllegalArgumentException("literal: " + c);
		}
	}

	/** The inverse of {@link #lit}, for the handful of literals the edit script uses. */
	static Tag parseLit(String s) {
		return switch (s.charAt(0)) {
			case 'i' -> IntTag.valueOf(Integer.parseInt(s.substring(1)));
			case 'b' -> ByteTag.valueOf(Byte.parseByte(s.substring(1)));
			case 's' -> ShortTag.valueOf(Short.parseShort(s.substring(1)));
			case 'l' -> LongTag.valueOf(Long.parseLong(s.substring(1)));
			case 'S' -> StringTag.valueOf(s.substring(2, s.length() - 1));
			default -> throw new IllegalArgumentException(s);
		};
	}

	public static void main(String[] args) throws Exception {
		SHALLOW_COPY = CompoundTag.class.getDeclaredMethod("shallowCopy");
		SHALLOW_COPY.setAccessible(true);
		IDENTIFY = ListTag.class.getDeclaredMethod("identifyRawElementType");
		IDENTIFY.setAccessible(true);
		TABLE = HashMap.class.getDeclaredField("table");
		TABLE.setAccessible(true);
		Path target = Path.of(args[0]);
		try (Writer w = new OutputStreamWriter(Files.newOutputStream(target), StandardCharsets.UTF_8)) {
			out = new PrintWriter(w);
			out.print("# nbt_probe.txt -- generated by _porting/java-oracle/src/p/NbtProbe.java (JDK " + System.getProperty("java.version")
				+ ", minecraft-merged-deobf-26.2.jar). TAB-separated; see NbtProbe's javadoc for the format.\n");
			out.print("# sanity\tRandom(0).nextInt()=" + new Random(0).nextInt() + "\n");
			typeSection();
			primSection();
			quoteSection();
			indexSection();
			ioSection();
			hashMapSection();
			caseSection();
			readSection();
			genSection();
			hashMapStepSection();
			containerSection();
			out.flush();
		}
		System.err.println("rows: " + rows);
		System.exit(0);
	}
}
