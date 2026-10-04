// ============================================================================
// CORE VALUE TYPE ORACLE -- not game code.
//
// Emits golden rows for the batch-2 value types: Vec3i, Direction, BlockPos,
// ChunkPos, Vec3, AABB, ARGB, Identifier.
//
// Run via _porting/java-oracle/run.ps1, which compiles these against the ORIGINAL
// sources in minecraft-decompiled/ and the pinned third-party jars.
// ============================================================================
package oracle;

import java.util.ArrayList;
import java.util.List;

import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.Vec3i;
import net.minecraft.resources.Identifier;
import net.minecraft.util.ARGB;
import net.minecraft.world.level.ChunkPos;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;

/**
 * Coordinates worth covering.
 *
 * Deliberately includes 0, +/-1, +/-2^24, +/-2^31 and the world-limit edges, because
 * the interesting failures in this batch are all at the extremes: float narrowing in
 * {@code distManhattan}, wrapping in {@code asLong}, and the signed-zero and
 * integer-overflow cases in the AABB predicates.
 */
final class Vals {
	private Vals() {
	}

	static final int[] INTS = {
		0, 1, -1, 2, -2, 3, -3, 7, -7, 8, -8, 16, -16, 31, 32, 33, 63, 64, 127, 128, 255, 256, -255, -256,
		1_000, -1_000, 30_000_000, -30_000_000, 1 << 23, -(1 << 23), (1 << 23) + 1, 1 << 24, -(1 << 24),
		(1 << 24) + 1, Integer.MAX_VALUE, Integer.MIN_VALUE, Integer.MAX_VALUE - 1, Integer.MIN_VALUE + 1
	};

	static final long[] LONGS = {
		0L, 1L, -1L, 2L, -2L, 255L, 65535L, 1L << 32, -(1L << 32), Long.MAX_VALUE, Long.MIN_VALUE
	};

	static final float[] FLOATS = {
		0.0F, -0.0F, 1.0F, -1.0F, 0.5F, -0.5F, 1.5F, -1.5F, 2.0F, -2.0F, 0.1F, -0.1F,
		Float.MIN_VALUE, Float.MAX_VALUE, Float.MIN_NORMAL, 1.0e-45F, 3.4028235E38F,
		Float.POSITIVE_INFINITY, Float.NEGATIVE_INFINITY, Float.NaN
	};

	static final double[] DOUBLES = {
		0.0, -0.0, 1.0, -1.0, 0.5, -0.5, 1.5, -1.5, 2.0, -2.0, 0.1, -0.1,
		Double.MIN_VALUE, Double.MAX_VALUE, Double.MIN_NORMAL,
		Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY, Double.NaN,
		1.0e30, -1.0e30, 1.0e-30
	};
}

final class CoreOracle {

	private CoreOracle() {
	}

	/**
	 * Coordinates for the pairwise distance groups.
	 *
	 * The 2^24 / 2^25 entries are the point of the whole list: `distManhattan` sums in
	 * `float` and `distChessboard` in `int`, and above 2^24 they disagree. Values around
	 * the 30M world limit check the realistic edge too.
	 */
	private static final int[][] DISTANCE_INPUTS = {
		{ 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 }, { 30_000_000, -30_000_000, 7 },
		{ 1 << 24, 0, 0 }, { (1 << 24) + 3, 0, 0 }, { 1 << 25, 1 << 25, 1 << 25 },
		{ Integer.MAX_VALUE, Integer.MIN_VALUE, 0 }
	};

	/** Chunk coordinates, including the region-file index range and the int extremes. */
	private static final int[][] CHUNK_INPUTS = {
		{ 0, 0 }, { 1, 2 }, { -1, -2 }, { 1_000_000, -1_000_000 },
		{ Integer.MAX_VALUE, Integer.MIN_VALUE }, { 1_875_000, -1_875_000 }, { 1_875_066, 1_875_066 }
	};

	/** Vectors for `getApproximateNearest`; includes 1e30f, which is finite but huge. */
	private static final float[] APPROX_INPUTS = { 0.0F, -0.0F, 1.0F, -1.0F, 0.5F, -0.5F, 2.0F, -2.0F, 1.0e-30F, 1.0e30F };
	static void emit(final java.nio.file.Path out) throws Exception {
		final Out o = new Out();

		// -----------------------------------------------------------------------
		// Vec3i
		// -----------------------------------------------------------------------
		o.fn("vec3i.hashCode", "i32 i32 i32", "i32");
		for (int x : Vals.INTS) {
			for (int y : Vals.INTS) {
				for (int z : Vals.INTS) {
					o.row(Out.join(Out.i32(x), Out.i32(y), Out.i32(z)),
							Out.i32(new Vec3i(x, y, z).hashCode()));
				}
			}
		}
		o.blank();

		o.fn("vec3i.compareTo", "i32 i32 i32 i32 i32 i32", "i32");
		// Deliberately small. A full cross product here is ~1.8M rows and 60 MB of
		// committed golden data; what compareTo can get wrong is tie-breaking and sign
		// of the subtraction, and every value below exercises one of those.
		final int[] CMP = { 0, 1, -1, 2, 100, -100, Integer.MAX_VALUE, Integer.MIN_VALUE };
		for (int x : CMP) {
			for (int y : CMP) {
				for (int z : CMP) {
					Vec3i a = new Vec3i(x, y, z);
					for (int x2 : CMP) {
						for (int y2 : CMP) {
							for (int z2 : CMP) {
								o.row(Out.join(Out.i32(x), Out.i32(y), Out.i32(z), Out.i32(x2), Out.i32(y2), Out.i32(z2)),
										Out.i32(a.compareTo(new Vec3i(x2, y2, z2))));
							}
						}
					}
				}
			}
		}
		o.blank();

		o.fn("vec3i.offset", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] a : new int[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 }, { Integer.MAX_VALUE, 0, Integer.MIN_VALUE } }) {
			Vec3i p = new Vec3i(a[0], a[1], a[2]);
			for (int[] d : new int[][] { { 0, 0, 0 }, { 1, 0, 0 }, { -1, 0, 0 }, { 0, 1, 0 }, { 0, 0, -1 }, { 7, -7, 7 },
					{ Integer.MAX_VALUE, Integer.MIN_VALUE, 1 } }) {
				Vec3i r = p.offset(d[0], d[1], d[2]);
				o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]), Out.i32(d[0]), Out.i32(d[1]), Out.i32(d[2])),
						Out.join(Out.i32(r.getX()), Out.i32(r.getY()), Out.i32(r.getZ())));
			}
		}
		o.blank();

		o.fn("vec3i.multiply", "i32 i32 i32 i32", "i32 i32 i32");
		for (int[] a : new int[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 }, { 1000, -1000, 7 }, { Integer.MAX_VALUE, Integer.MIN_VALUE, 2 } }) {
			Vec3i p = new Vec3i(a[0], a[1], a[2]);
			for (int s : new int[] { 0, 1, -1, 2, -2, 3, 1000, Integer.MAX_VALUE, Integer.MIN_VALUE }) {
				Vec3i r = p.multiply(s);
				o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]), Out.i32(s)),
						Out.join(Out.i32(r.getX()), Out.i32(r.getY()), Out.i32(r.getZ())));
			}
		}
		o.blank();

		o.fn("vec3i.cross", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] a : new int[][] { { 1, 0, 0 }, { 0, 1, 0 }, { 0, 0, 1 }, { 1, 2, 3 }, { -1, -2, -3 }, { 1000, 2000, 3000 } }) {
			Vec3i p = new Vec3i(a[0], a[1], a[2]);
			for (int[] b : new int[][] { { 1, 0, 0 }, { 0, 1, 0 }, { 0, 0, 1 }, { 3, -2, 1 }, { -4, 5, -6 } }) {
				Vec3i r = p.cross(new Vec3i(b[0], b[1], b[2]));
				o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]), Out.i32(b[0]), Out.i32(b[1]), Out.i32(b[2])),
						Out.join(Out.i32(r.getX()), Out.i32(r.getY()), Out.i32(r.getZ())));
			}
		}
		o.blank();

		o.fn("vec3i.distManhattan", "i32 i32 i32 i32 i32 i32", "i32");
		for (int[] a : DISTANCE_INPUTS) {
			Vec3i p = new Vec3i(a[0], a[1], a[2]);
			for (int[] b : DISTANCE_INPUTS) {
				o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]), Out.i32(b[0]), Out.i32(b[1]), Out.i32(b[2])),
						Out.i32(p.distManhattan(new Vec3i(b[0], b[1], b[2]))));
			}
		}
		o.blank();

		// A SEPARATE loop from the one above. `Out` attributes every row to the most
		// recent `fn(...)` header, so emitting two different methods inside one loop
		// silently files all of them under the LAST header. That is not a formatting
		// nit -- it produces empty groups and a parity test that "passes" by testing
		// nothing.
		o.fn("vec3i.distChessboard", "i32 i32 i32 i32 i32 i32", "i32");
		for (int[] a : DISTANCE_INPUTS) {
			Vec3i p = new Vec3i(a[0], a[1], a[2]);
			for (int[] b : DISTANCE_INPUTS) {
				o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]), Out.i32(b[0]), Out.i32(b[1]), Out.i32(b[2])),
						Out.i32(p.distChessboard(new Vec3i(b[0], b[1], b[2]))));
			}
		}
		o.blank();

		o.fn("vec3i.distSqr", "i32 i32 i32 i32 i32 i32", "f64");
		for (int[] a : DISTANCE_INPUTS) {
			Vec3i p = new Vec3i(a[0], a[1], a[2]);
			for (int[] b : DISTANCE_INPUTS) {
				o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]), Out.i32(b[0]), Out.i32(b[1]), Out.i32(b[2])),
						Out.f64(p.distSqr(new Vec3i(b[0], b[1], b[2]))));
			}
		}
		o.blank();

		// Separate loop again -- see the note on vec3i.distChessboard.
		o.fn("vec3i.distToCenterSqr", "i32 i32 i32 f64 f64 f64", "f64");
		for (int[] a : DISTANCE_INPUTS) {
			Vec3i p = new Vec3i(a[0], a[1], a[2]);
			for (double d : new double[] { 0.0, -0.0, 0.5, 1.0, -1.0, 0.5e-9, 1.0e9 }) {
				o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]), Out.f64(d), Out.f64(d), Out.f64(d)),
						Out.f64(p.distToCenterSqr(d, d, d)));
			}
		}
		o.blank();

		// TWO groups, not one. `relative(Direction,int)` and `relative(Axis,int)` both
		// produce a five-argument row, so sharing a group makes them indistinguishable
		// to the parity test -- an axis ordinal would be read as a step count.
		o.fn("vec3i.relative", "i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] a : new int[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 } }) {
			Vec3i p = new Vec3i(a[0], a[1], a[2]);
			for (Direction d : Direction.values()) {
				for (int s : new int[] { 0, 1, -1, 2, 7, -7, 64 }) {
					Vec3i r = p.relative(d, s);
					o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]), Out.i32(d.ordinal()), Out.i32(s)),
							Out.join(Out.i32(r.getX()), Out.i32(r.getY()), Out.i32(r.getZ())));
				}
			}
		}
		o.blank();

		o.fn("vec3i.relativeAxis", "i32 i32 i32 i32", "i32 i32 i32");
		for (int[] a : new int[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 } }) {
			Vec3i p = new Vec3i(a[0], a[1], a[2]);
			for (Direction.Axis axis : Direction.Axis.values()) {
				for (int s : new int[] { 0, 1, -1, 5, -5 }) {
					Vec3i r = p.relative(axis, s);
					o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]), Out.i32(axis.ordinal()), Out.i32(s)),
							Out.join(Out.i32(r.getX()), Out.i32(r.getY()), Out.i32(r.getZ())));
				}
			}
		}
		o.blank();

		o.fn("vec3i.equals", "i32 i32 i32 i32 i32 i32", "b");
		for (int[] a : new int[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 } }) {
			for (int[] b : new int[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 }, { 1, 2, 4 } }) {
				o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]), Out.i32(b[0]), Out.i32(b[1]), Out.i32(b[2])),
						Out.b(new Vec3i(a[0], a[1], a[2]).equals(new Vec3i(b[0], b[1], b[2]))));
			}
		}
		o.blank();

		o.fn("vec3i.hashEdge", "i32 i32 i32", "i32");
		for (int x : Vals.INTS) {
			o.row(Out.join(Out.i32(x), Out.i32(0), Out.i32(0)), Out.i32(new Vec3i(x, 0, 0).hashCode()));
			o.row(Out.join(Out.i32(0), Out.i32(x), Out.i32(0)), Out.i32(new Vec3i(0, x, 0).hashCode()));
			o.row(Out.join(Out.i32(0), Out.i32(0), Out.i32(x)), Out.i32(new Vec3i(0, 0, x).hashCode()));
		}
		o.blank();

		// -----------------------------------------------------------------------
		// Direction -- ordinals and the lookup tables
		// -----------------------------------------------------------------------
		o.fn("direction.ordinal", "", "i32");
		for (Direction d : Direction.values()) {
			o.row("", Out.join(Out.i32(d.ordinal()), Out.i32(d.get3DDataValue()), Out.i32(d.get2DDataValue()),
					Out.i32(d.getAxis().ordinal()), Out.i32(d.getAxisDirection().ordinal()),
					Out.str(d.getName())));
		}
		o.blank();

		// The ordinal is recorded as an argument. Without it a parity test cannot tell
		// which direction a row describes, and every step triple is a valid-looking
		// answer for some other direction.
		o.fn("direction.step", "i32", "i32 i32 i32");
		for (Direction d : Direction.values()) {
			o.row(Out.i32(d.ordinal()), Out.join(Out.i32(d.getStepX()), Out.i32(d.getStepY()), Out.i32(d.getStepZ())));
		}
		o.blank();

		// BY_ID is exercised through the same out-of-range inputs as from3DDataValue:
		// ByIdMap.continuous(..., WRAP) is the same abs(x % n) rule, and the parity test
		// asserts the two agree rather than assuming it.
		o.fn("direction.byId", "i32", "i32");
		for (int d : new int[] { 0, 1, 2, 3, 4, 5, 6, 7, -1, -2, -6, -7, 100, -100, Integer.MAX_VALUE, Integer.MIN_VALUE }) {
			o.row(Out.i32(d), Out.i32(Direction.BY_ID.apply(d).ordinal()));
		}
		o.blank();

		o.fn("direction.fromData", "i32", "i32 i32");
		for (int d : new int[] { 0, 1, 2, 3, 4, 5, 6, 7, -1, -2, -3, -6, -7, 100, -100, Integer.MAX_VALUE, Integer.MIN_VALUE }) {
			o.row(Out.i32(d), Out.join(Out.i32(Direction.from3DDataValue(d).ordinal()),
					Out.i32(Direction.from2DDataValue(d).ordinal())));
		}
		o.blank();

		o.fn("direction.rotations", "i32", "i32 i32 i32 i32 i32 i32");
		for (Direction d : Direction.values()) {
			List<Integer> cw = new ArrayList<>();
			for (Direction.Axis axis : Direction.Axis.values()) {
				try {
					cw.add(d.getClockWise(axis).ordinal());
				} catch (IllegalStateException e) {
					cw.add(-1);
				}
				try {
					cw.add(d.getCounterClockWise(axis).ordinal());
				} catch (IllegalStateException e) {
					cw.add(-1);
				}
			}
			int cwY = -1;
			int ccwY = -1;
			try {
				cwY = d.getClockWise().ordinal();
			} catch (IllegalStateException e) {
				cwY = -1;
			}
			try {
				ccwY = d.getCounterClockWise().ordinal();
			} catch (IllegalStateException e) {
				ccwY = -1;
			}
			o.row(Out.i32(d.ordinal()), Out.join(Out.i32(cw.get(0)), Out.i32(cw.get(1)), Out.i32(cw.get(2)),
					Out.i32(cw.get(3)), Out.i32(cw.get(4)), Out.i32(cw.get(5)), Out.i32(cwY), Out.i32(ccwY)));
		}
		o.blank();

		o.fn("direction.opposite", "i32", "i32");
		for (Direction d : Direction.values()) {
			o.row(Out.i32(d.ordinal()), Out.i32(d.getOpposite().ordinal()));
		}
		o.blank();

		o.fn("direction.getNearest", "i32 i32 i32", "i32");
		for (int x : new int[] { 0, 1, -1, 2, -2, 5, -5, 100 }) {
			for (int y : new int[] { 0, 1, -1, 2, -2 }) {
				for (int z : new int[] { 0, 1, -1, 2, -2 }) {
					// The no-fallback overload: ties resolve to a real direction or null.
					Direction r = Direction.getNearest(x, y, z, null);
					o.row(Out.join(Out.i32(x), Out.i32(y), Out.i32(z)), Out.i32(r == null ? -1 : r.ordinal()));
				}
			}
		}
		o.blank();

		o.fn("direction.getApproximateNearest_f", "f32 f32 f32", "i32");
		for (float f : APPROX_INPUTS) {
			for (float g : APPROX_INPUTS) {
				for (float h : APPROX_INPUTS) {
					o.row(Out.join(Out.f32(f), Out.f32(g), Out.f32(h)),
							Out.i32(Direction.getApproximateNearest(f, g, h).ordinal()));
				}
			}
		}
		o.blank();

		// Separate loop again -- see the note on vec3i.distChessboard above.
		o.fn("direction.getApproximateNearest_d", "f64 f64 f64", "i32");
		for (float f : APPROX_INPUTS) {
			for (float g : APPROX_INPUTS) {
				for (float h : APPROX_INPUTS) {
					o.row(Out.join(Out.f64(f), Out.f64(g), Out.f64(h)),
							Out.i32(Direction.getApproximateNearest((double) f, (double) g, (double) h).ordinal()));
				}
			}
		}
		o.blank();

		o.fn("direction.fromYRot", "f64", "i32");
		for (double r : new double[] { 0.0, 45.0, 90.0, -45.0, -90.0, 180.0, -180.0, 270.0, -270.0, 360.0, 1.0e9 }) {
			o.row(Out.f64(r), Out.i32(Direction.fromYRot(r).ordinal()));
		}
		o.blank();

		o.fn("direction.axisGet", "", "i32 i32 i32 i32");
		for (Direction.Axis axis : Direction.Axis.values()) {
			o.row("", Out.join(Out.i32(axis.ordinal()), Out.i32(axis.getPositive().ordinal()),
					Out.i32(axis.getNegative().ordinal()), Out.i32(axis.isVertical() ? 1 : 0)));
		}
		o.blank();

		o.fn("direction.axisDirection", "", "i32 i32");
		for (Direction.AxisDirection ad : Direction.AxisDirection.values()) {
			o.row("", Out.join(Out.i32(ad.ordinal()), Out.i32(ad.getStep())));
		}
		o.blank();

		// -----------------------------------------------------------------------
		// BlockPos -- bit packing is the load-bearing part
		// -----------------------------------------------------------------------
		o.fn("blockpos.asLong", "i32 i32 i32", "i64");
		for (int[] a : new int[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 }, { 30_000_000, 30_000_000, 30_000_000 },
				{ -30_000_000, -30_000_000, -30_000_000 }, { 33_554_431, -33_554_432, 0 }, { Integer.MAX_VALUE, Integer.MIN_VALUE, 0 },
				{ 0, -1, 1 }, { 1_000_000, -1_000_000, 5 } }) {
			o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2])),
					Out.i64(BlockPos.asLong(a[0], a[1], a[2])));
		}
		o.blank();

		o.fn("blockpos.fromLong", "i64", "i32 i32 i32");
		for (long l : Vals.LONGS) {
			o.row(Out.i64(l), Out.join(Out.i32(BlockPos.getX(l)), Out.i32(BlockPos.getY(l)), Out.i32(BlockPos.getZ(l))));
		}
		// Plus round-trips of every asLong above, which is where a wrong SHIFT count or
		// a signed/unsigned mix-up shows up immediately.
		for (int[] a : new int[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 }, { 30_000_000, -30_000_000, 7 },
				{ Integer.MAX_VALUE, Integer.MIN_VALUE, 12345 }, { -33_554_432, 33_554_431, -1 } }) {
			long l = BlockPos.asLong(a[0], a[1], a[2]);
			o.row(Out.i64(l), Out.join(Out.i32(BlockPos.getX(l)), Out.i32(BlockPos.getY(l)), Out.i32(BlockPos.getZ(l))));
		}
		o.blank();

		o.fn("blockpos.hashCode", "i32 i32 i32", "i32");
		for (int[] a : new int[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 }, { 30_000_000, -30_000_000, 7 }, { Integer.MAX_VALUE, 0, 0 } }) {
			o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2])),
					Out.i32(new BlockPos(a[0], a[1], a[2]).hashCode()));
		}
		o.blank();

		o.fn("blockpos.withOffset", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] a : new int[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 } }) {
			BlockPos p = new BlockPos(a[0], a[1], a[2]);
			for (int[] d : new int[][] { { 0, 0, 0 }, { 1, 0, 0 }, { 0, -1, 0 }, { 0, 0, 1 }, { 7, -7, 7 } }) {
				for (Direction dir : Direction.values()) {
					BlockPos r = p.offset(d[0], d[1], d[2]).relative(dir, 3);
					o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]), Out.i32(d[0]), Out.i32(d[1]), Out.i32(d[2]), Out.i32(dir.ordinal())),
							Out.join(Out.i32(r.getX()), Out.i32(r.getY()), Out.i32(r.getZ())));
				}
			}
		}
		o.blank();

		// -----------------------------------------------------------------------
		// ChunkPos
		// -----------------------------------------------------------------------
		o.fn("chunkpos.toLong", "i32 i32", "i64");
		for (int[] a : CHUNK_INPUTS) {
			o.row(Out.join(Out.i32(a[0]), Out.i32(a[1])), Out.i64(new ChunkPos(a[0], a[1]).pack()));
		}
		o.blank();

		// Separate loops again -- see the note on vec3i.distChessboard.
		o.fn("chunkpos.hashCode", "i32 i32", "i32");
		for (int[] a : CHUNK_INPUTS) {
			o.row(Out.join(Out.i32(a[0]), Out.i32(a[1])), Out.i32(new ChunkPos(a[0], a[1]).hashCode()));
		}
		o.blank();

		o.fn("chunkpos.accessors", "i32 i32", "i32 i32");
		for (int[] a : CHUNK_INPUTS) {
			ChunkPos c = new ChunkPos(a[0], a[1]);
			o.row(Out.join(Out.i32(a[0]), Out.i32(a[1])), Out.join(Out.i32(c.x()), Out.i32(c.z())));
		}
		o.blank();

		o.fn("chunkpos.fromLong", "i64", "i32 i32");
		for (long l : Vals.LONGS) {
			ChunkPos c = ChunkPos.unpack(l);
			o.row(Out.i64(l), Out.join(Out.i32(c.x()), Out.i32(c.z())));
		}
		for (int[] a : new int[][] { { 0, 0 }, { 1, 2 }, { -1, -2 }, { Integer.MAX_VALUE, Integer.MIN_VALUE } }) {
			ChunkPos original = new ChunkPos(a[0], a[1]);
			ChunkPos roundTripped = ChunkPos.unpack(original.pack());
			o.row(Out.i64(original.pack()), Out.join(Out.i32(roundTripped.x()), Out.i32(roundTripped.z())));
		}
		o.blank();

		o.fn("chunkpos.minMaxBlock", "i32 i32", "i32 i32 i32 i32");
		for (int[] a : new int[][] { { 0, 0 }, { 1, 2 }, { -1, -2 }, { 1_875_000, -1_875_000 }, { Integer.MAX_VALUE, Integer.MIN_VALUE } }) {
			ChunkPos c = new ChunkPos(a[0], a[1]);
			o.row(Out.join(Out.i32(a[0]), Out.i32(a[1])),
					Out.join(Out.i32(c.getMinBlockX()), Out.i32(c.getMinBlockZ()),
							Out.i32(c.getMaxBlockX()), Out.i32(c.getMaxBlockZ())));
		}
		o.blank();

		// -----------------------------------------------------------------------
		// Vec3 -- double precision throughout
		// -----------------------------------------------------------------------
		o.fn("vec3.arith", "f64 f64 f64 f64 f64 f64", "f64 f64 f64");
		for (double a : new double[] { 0.0, -0.0, 1.0, -1.0, 0.5, -0.5, 2.0, 1e30, -1e30 }) {
			for (double b : new double[] { 0.0, 1.0, -1.0, 2.0, 0.25 }) {
				Vec3 v = new Vec3(a, b, -b);
				o.row(Out.join(Out.f64(a), Out.f64(b), Out.f64(-b), Out.f64(a), Out.f64(b), Out.f64(-b)),
						Out.join(Out.f64(v.x), Out.f64(v.y), Out.f64(v.z())));
				Vec3 s = v.add(a, b, -b);
				o.row(Out.join(Out.f64(a), Out.f64(b), Out.f64(-b), Out.f64(a), Out.f64(b), Out.f64(-b)),
						Out.join(Out.f64(s.x), Out.f64(s.y), Out.f64(s.z())));
				Vec3 m = v.scale(b);
				o.row(Out.join(Out.f64(a), Out.f64(b), Out.f64(-b), Out.f64(a), Out.f64(b), Out.f64(-b)),
						Out.join(Out.f64(m.x), Out.f64(m.y), Out.f64(m.z())));
				Vec3 d = v.multiply(new Vec3(a, b, -b));
				o.row(Out.join(Out.f64(a), Out.f64(b), Out.f64(-b), Out.f64(a), Out.f64(b), Out.f64(-b)),
						Out.join(Out.f64(d.x), Out.f64(d.y), Out.f64(d.z())));
			}
		}
		o.blank();

		o.fn("vec3.dot_cross_len", "f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64");
		for (double[] p : new double[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 }, { 0.5, 0, -0.5 }, { 1e30, 1e-30, 0 } }) {
			for (double[] q : new double[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 }, { 0.25, 0, -0.25 } }) {
				Vec3 v = new Vec3(p[0], p[1], p[2]);
				Vec3 w = new Vec3(q[0], q[1], q[2]);
				Vec3 c = v.cross(w);
				o.row(Out.join(Out.f64(p[0]), Out.f64(p[1]), Out.f64(p[2]), Out.f64(q[0]), Out.f64(q[1]), Out.f64(q[2])),
						Out.join(Out.f64(v.dot(w)), Out.f64(v.length()), Out.f64(v.lengthSqr()),
								Out.f64(c.x), Out.f64(c.y), Out.f64(c.z())));
				o.row(Out.join(Out.f64(p[0]), Out.f64(p[1]), Out.f64(p[2]), Out.f64(q[0]), Out.f64(q[1]), Out.f64(q[2])),
						Out.join(Out.f64(v.distanceTo(w)), Out.f64(v.distanceToSqr(w)),
								Out.f64(v.lerp(w, 0.5).x), Out.f64(v.lerp(w, 0.5).y), Out.f64(v.lerp(w, 0.5).z)));
			}
		}
		o.blank();

		o.fn("vec3.normalize", "f64 f64 f64", "f64 f64 f64");
		for (double[] p : new double[][] { { 1, 0, 0 }, { -1, 0, 0 }, { 0, 0, 0 }, { 3, 4, 0 }, { 1e30, 1e-30, 0 },
				{ 1, 1, 1 }, { Double.MIN_VALUE, 0, 0 } }) {
			Vec3 n = new Vec3(p[0], p[1], p[2]).normalize();
			o.row(Out.join(Out.f64(p[0]), Out.f64(p[1]), Out.f64(p[2])), Out.join(Out.f64(n.x), Out.f64(n.y), Out.f64(n.z)));
		}
		o.blank();

		o.fn("vec3.hashCode", "f64 f64 f64", "i32");
		for (double[] p : new double[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 }, { 0.5, 0, -0.5 } }) {
			o.row(Out.join(Out.f64(p[0]), Out.f64(p[1]), Out.f64(p[2])),
					Out.i32(new Vec3(p[0], p[1], p[2]).hashCode()));
		}
		o.blank();

		// -----------------------------------------------------------------------
		// AABB
		// -----------------------------------------------------------------------
		o.fn("aabb.minmax", "f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64 f64 f64 f64");
		for (double[] p : new double[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 }, { 3, 4, 5 }, { 5, 4, 3 },
				{ -0.0, 0.0, -0.0 } }) {
			for (double[] q : new double[][] { { 1, 1, 1 }, { -1, -1, -1 }, { 0.5, 3.5, 2.5 } }) {
				AABB b = new AABB(p[0], p[1], p[2], q[0], q[1], q[2]);
				o.row(Out.join(Out.f64(p[0]), Out.f64(p[1]), Out.f64(p[2]), Out.f64(q[0]), Out.f64(q[1]), Out.f64(q[2])),
						Out.join(Out.f64(b.minX), Out.f64(b.minY), Out.f64(b.minZ),
								Out.f64(b.maxX), Out.f64(b.maxY), Out.f64(b.maxZ),
								Out.f64(b.getXsize()), Out.f64(b.getYsize()), Out.f64(b.getZsize())));
			}
		}
		o.blank();

		o.fn("aabb.contains", "f64 f64 f64 f64 f64 f64 f64 f64 f64", "b b b");
		for (double[] p : new double[][] { { 0, 0, 0 }, { 1, 1, 1 }, { -1, -1, -1 }, { 5, 5, 5 } }) {
			for (double[] q : new double[][] { { 1, 1, 1 }, { -1, -1, -1 }, { 2, 2, 2 }, { 0, 0, 0 } }) {
				AABB b = new AABB(p[0], p[1], p[2], q[0], q[1], q[2]);
				o.row(Out.join(Out.f64(p[0]), Out.f64(p[1]), Out.f64(p[2]), Out.f64(q[0]), Out.f64(q[1]), Out.f64(q[2]),
								Out.f64(0.5), Out.f64(0.5), Out.f64(0.5)),
						Out.join(Out.b(b.contains(0.5, 0.5, 0.5)), Out.b(b.contains(new Vec3(p[0], p[1], p[2]))),
								Out.b(b.intersects(b))));
			}
		}
		o.blank();

		o.fn("aabb.inflate", "f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] p : new double[][] { { 0, 0, 0 }, { 1, 2, 3 }, { -1, -2, -3 } }) {
			for (double d : new double[] { 0.0, 1.0, -1.0, 0.5, 1e30 }) {
				AABB b = new AABB(p[0], p[1], p[2], p[0] + 1, p[1] + 1, p[2] + 1);
				AABB r = b.inflate(d);
				o.row(Out.join(Out.f64(p[0]), Out.f64(p[1]), Out.f64(p[2]), Out.f64(d), Out.f64(d), Out.f64(d), Out.f64(0)),
						Out.join(Out.f64(r.minX), Out.f64(r.minY), Out.f64(r.minZ),
								Out.f64(r.maxX), Out.f64(r.maxY), Out.f64(r.maxZ)));
			}
		}
		o.blank();

		// -----------------------------------------------------------------------
		// ARGB
		// -----------------------------------------------------------------------
		o.fn("argb.channels", "i32", "i32 i32 i32 i32");
		for (int c : new int[] { 0, 1, -1, 255, 256, 0x7F, 0x80, 0xFF00_0000, Integer.MAX_VALUE, Integer.MIN_VALUE,
				0x1234_5678, 0xABCD_EF01 }) {
			o.row(Out.i32(c), Out.join(Out.i32(ARGB.red(c)), Out.i32(ARGB.green(c)), Out.i32(ARGB.blue(c)), Out.i32(ARGB.alpha(c))));
		}
		o.blank();

		o.fn("argb.combine", "i32 i32 i32 i32", "i32");
		// ARGB.color(alpha, red, green, blue) -- four channels, alpha FIRST.
		// (Note the argument order differs from colorFromFloat, which is also
		// alpha-first but takes floats. This asymmetry is easy to get wrong.)
		for (int[] c : new int[][] { { 0, 0, 0, 0 }, { 255, 255, 128, 64 }, { 128, 1, 2, 3 }, { -1, -1, -1, -1 },
				{ 255, 256, -1, 300 } }) {
			o.row(Out.join(Out.i32(c[0]), Out.i32(c[1]), Out.i32(c[2]), Out.i32(c[3])),
					Out.i32(ARGB.color(c[0], c[1], c[2], c[3])));
		}
		o.blank();

		o.fn("argb.fromFloat", "f32 f32 f32 f32", "i32");
		for (float[] c : new float[][] { { 0, 0, 0, 0 }, { 1, 1, 1, 1 }, { 0.5f, 0.25f, 1.0f, 0.0f },
				{ -1, 2, 0.5f, 1 }, { Float.NaN, 0, 0, 0 } }) {
			o.row(Out.join(Out.f32(c[0]), Out.f32(c[1]), Out.f32(c[2]), Out.f32(c[3])),
					Out.i32(ARGB.colorFromFloat(c[0], c[1], c[2], c[3])));
		}
		o.blank();

		// -----------------------------------------------------------------------
		// Identifier -- parsing, validation, and the ERROR CASES
		//
		// `tryParse` is annotated @Nullable and normally returns null, but it can still
		// THROW: `tryBySeparator` reaches `assertValidPath` on one branch, and an input
		// like "minecraft:minecraft:stone" hits it. A nullable-returning method that
		// throws is a vanilla quirk worth pinning, so every call below is wrapped and
		// the exception recorded as data rather than crashing the oracle.
		// -----------------------------------------------------------------------
		o.fn("identifier.tryParse", "str", "str");
		for (String s : IDENT_INPUTS) {
			o.row(Out.str(s), Out.str(describeTryParse(s)));
		}
		o.blank();

		o.fn("identifier.parse", "str", "str");
		for (String s : IDENT_INPUTS) {
			o.row(Out.str(s), Out.str(describe(() -> Identifier.parse(s).toString())));
		}
		o.blank();

		o.fn("identifier.withDefaultNamespace", "str", "str");
		for (String s : IDENT_INPUTS) {
			o.row(Out.str(s), Out.str(describe(() -> Identifier.withDefaultNamespace(s).toString())));
		}
		o.blank();

		o.fn("identifier.tryBuild", "str str", "str");
		for (String[] p : new String[][] { { "a", "b" }, { "A", "b" }, { "a", "B" }, { "a b", "c" }, { "", "b" },
				{ "a", "" }, { "a:b", "c" }, { "minecraft", "stone" }, { "a", "b/c" }, { "a", "b.c-d_e" } }) {
			o.row(Out.join(Out.str(p[0]), Out.str(p[1])), Out.str(describe(() -> {
				Identifier id = Identifier.tryBuild(p[0], p[1]);
				return id == null ? "<null>" : id.toString();
			})));
		}
		o.blank();

		// `withDefaultNamespace` treats its argument as a PATH, so an input that itself
		// contains a colon ("a:b") is rejected -- the colon is not in [a-z0-9/._-].
		// Wrapped, because that rejection is the interesting part.
		o.fn("identifier.hashEquals", "str str", "str");
		for (String[] p : new String[][] {
			{ "minecraft:stone", "minecraft:stone" }, { "stone", "stone" }, { "stone", "dirt" },
			{ "a:b", "a:c" }, { "minecraft:stone", "minecraft:dirt" }, { "a:b", "b:a" },
			{ "a:b", "A:B" }, { "a b", "a b" }, { "", "" }
		}) {
			o.row(Out.join(Out.str(p[0]), Out.str(p[1])), Out.str(describe(() -> {
				Identifier x = Identifier.withDefaultNamespace(p[0]);
				Identifier y = Identifier.withDefaultNamespace(p[1]);
				return x.hashCode() + ":" + x.equals(y);
			})));
		}
		o.blank();

		o.fn("identifier.namespacePath", "str", "str str");
		for (String s : new String[] { "minecraft:stone", "stone", "a:b", "a:b/c", "ns:path.with.dots", "a b" }) {
			o.row(Out.str(s), Out.str(describe(() -> {
				Identifier id = Identifier.withDefaultNamespace(s);
				return id.getNamespace() + "|" + id.getPath();
			})));
		}
		o.blank();

		o.write(out);
	}

	/** Inputs chosen to cover the separator, the character class, and the empty cases. */
	private static final String[] IDENT_INPUTS = {
		"minecraft:stone", "stone", "minecraft:deepslate", "a:b", "ns:path/to/thing",
		"minecraft:models/block/stone", "some-namespace:a_path.with.dots", "x:y",
		"Minecraft:Stone", "minecraft:", ":path", "minecraft:a:b", "a:b:c", "", " ", "a b:c",
		"A:B", "minecraft:Ore_Block.123", "namespace:path!", "namespace:path with space",
		"::", "a::b", "a:b:", "UPPER:lower", "lower:UPPER", "namespace with space:path",
		"minecraft:minecraft:stone", "-", "_", ".", "/", "a:b/c/d/e"
	};

	/** Run a supplier, encoding either its result or `ExceptionType:message` as a string. */
	private static String describe(final java.util.function.Supplier<String> op) {
		try {
			return op.get();
		} catch (Exception e) {
			return e.getClass().getSimpleName() + ":" + e.getMessage();
		}
	}

	/** `tryParse` is @Nullable but can still throw, so it gets its own wrapper. */
	private static String describeTryParse(final String s) {
		try {
			Identifier id = Identifier.tryParse(s);
			return id == null ? "<null>" : id.toString();
		} catch (Exception e) {
			return e.getClass().getSimpleName() + ":" + e.getMessage();
		}
	}
}
