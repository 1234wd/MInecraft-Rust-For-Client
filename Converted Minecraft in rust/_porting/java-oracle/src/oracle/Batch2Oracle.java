// ============================================================================
// BATCH 2 ORACLE -- not game code.
//
// Emits golden rows for every file in batch 2: BlockPos, ChunkPos, SectionPos,
// Vec3, Vec2, AABB, ARGB, Identifier, Rotations, Direction.Plane, plus the Mth
// methods that were blocked on unported types.
//
// ---------------------------------------------------------------------------
// THREE RULES THIS FILE FOLLOWS, EACH LEARNED THE HARD WAY
// ---------------------------------------------------------------------------
//
// 1. THE JAR IS THE ONLY SOURCE OF TRUTH. Every expected value below is whatever
//    minecraft-merged-deobf-26.2.jar computes. Nothing is hand-derived, not even a
//    hex constant. Where the decompiled source and the jar disagree, the jar wins
//    (DESIGN_DECISIONS.md #decompiler-artifacts) -- which already caught four real
//    API differences in this file alone: findClosestMatch's arity, two private
//    ChunkPos constants, a private Identifier predicate, and two Mth method names
//    that do not exist in 26.2 at all.
//
// 2. EXACTLY ONE HEADER PER EMISSION LOOP.
//
//    Declaring several `o.fn(...)` headers and then running ONE loop that emits for
//    all of them puts every row under the LAST header. I did that in the first draft
//    of this file: `blockpos.facing` collected 66,420 rows that actually belonged to
//    four other groups, and those four groups were silently EMPTY. An empty group
//    looks identical to a passing one until a test claims it. So: a header is
//    immediately followed by the only loop that writes to it, always.
//
// 3. KEEP EACH GROUP SMALL ENOUGH TO READ.
//
//    Rule 2 only pays off if a failing test names the operation, which requires the
//    group to exist and be tractable. The first draft emitted 814 MB and was killed
//    partway through. Corpora are therefore split by ARITY: the full sets for
//    low-arity groups, smaller ones where the cross product would explode.
// ============================================================================
package oracle;

import java.util.ArrayList;
import java.util.EnumSet;
import java.util.List;
import java.util.Optional;
import java.util.function.BiFunction;

import net.minecraft.IdentifierException;
import net.minecraft.core.BlockPos;
import net.minecraft.core.Direction;
import net.minecraft.core.Rotations;
import net.minecraft.core.SectionPos;
import net.minecraft.core.Vec3i;
import net.minecraft.resources.Identifier;
import net.minecraft.util.ARGB;
import net.minecraft.util.Mth;
import net.minecraft.util.RandomSource;
import net.minecraft.world.level.ChunkPos;
import net.minecraft.world.level.block.Rotation;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.minecraft.world.phys.Vec2;

import org.apache.commons.lang3.math.Fraction;
import org.joml.Quaternionf;
import org.joml.Vector3f;

/** Coordinate and value corpora, split by how large a cross product they can afford. */
final class V2 {
	private V2() {
	}

	/**
	 * Full integer corpus. Used for groups taking ONE or TWO ints, where the row
	 * count is O(n) or O(n^2) and the breadth is free.
	 *
	 * The negative values are not decoration: `BlockPos`'s unpack side is a SIGNED
	 * right shift, so it sign-extends and every negative must round-trip exactly. The
	 * 2^23/2^24 values are where widening to float starts losing integers.
	 */
	static final int[] INTS = {
		0, 1, -1, 2, -2, 3, -3, 15, -15, 16, -16, 17, -17,
		2047, -2048, 2048, -2049,
		33_554_431, -33_554_432, 33_554_432, -33_554_433,
		30_000_000, -30_000_000, 30_000_001, -30_000_001,
		-64, 320, -65, 321,
		1 << 23, -(1 << 23), (1 << 23) + 1, 1 << 24, -(1 << 24), (1 << 24) + 1,
		Integer.MAX_VALUE, Integer.MIN_VALUE, Integer.MAX_VALUE - 1, Integer.MIN_VALUE + 1,
	};

	/** Small int set for the THREE-int groups, where O(n^3) is the cost. */
	static final int[] INTS3 = {
		0, 1, -1, 15, -16, 2047, -2048, 2048,
		33_554_431, -33_554_432, 30_000_000, -30_000_000,
		-64, 320, -65, 321,
		Integer.MAX_VALUE, Integer.MIN_VALUE,
		1 << 23, -(1 << 23), 1 << 24, -(1 << 24),
	};

	static final long[] LONGS = {
		0L, 1L, -1L, Long.MAX_VALUE, Long.MIN_VALUE,
		1L << 26, 1L << 38, 1L << 51, 1L << 62,
		-1L >>> 26, -1L >>> 12, -1L,
	};

	/** Full double corpus: one or two args only. */
	static final double[] DOUBLES = {
		0.0, -0.0, 1.0, -1.0, 0.5, -0.5, 1.5, -1.5, 2.0, -2.0, 0.1, -0.1, 0.25, -0.25,
		-0.9999999, 0.9999999, 1.0e30, -1.0e30, 1.0e-30,
		Double.MIN_VALUE, Double.MAX_VALUE, Double.MIN_NORMAL,
		Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY, Double.NaN,
	};

	/** Three or more doubles: 8 values, every value class still represented. */
	static final double[] DOUBLES3 = {
		0.0, -0.0, 1.0, -1.0, 0.5, -0.25, 2.0, Double.NaN,
	};

	static final float[] FLOATS = {
		0.0F, -0.0F, 1.0F, -1.0F, 0.5F, -0.5F, 2.0F, -2.0F, 0.1F, -0.1F,
		45.0F, 90.0F, 180.0F, -180.0F, 270.0F, 360.0F, 359.9F, -359.9F,
		Float.MIN_VALUE, Float.MAX_VALUE,
		Float.POSITIVE_INFINITY, Float.NEGATIVE_INFINITY, Float.NaN,
	};

	/** Two or more floats. */
	/** FLOATS2 minus the negatives, for methods that index an array with the result. */
	static final float[] NON_NEGATIVE_FLOATS = {
		0.0F, 1.0F, 0.5F, 180.0F, 360.0F, Float.NaN,
	};

	/**
	 * FLOATS2 minus the negatives, for methods that index an array with the result.
	 *
	 * `ARGB.linearLerp` is the reason this exists: see the note at its group.
	 */

	static final float[] FLOATS2 = {
		0.0F, -0.0F, 1.0F, -1.0F, 0.5F, 180.0F, 360.0F, Float.NaN,
	};

	static final long[] SEEDS = {
		0L, 1L, -1L, 42L, -42L, 123456789L, Integer.MAX_VALUE, Integer.MIN_VALUE,
	};

	/** Packed-long corpus: both packing functions, both signs, all-ones per field. */
	static final long[] PACKED = {
		0L, 1L, -1L, Long.MAX_VALUE, Long.MIN_VALUE,
		BlockPos.asLong(1, 2, 3), SectionPos.asLong(1, 2, 3),
		BlockPos.asLong(-1, -2, -3), SectionPos.asLong(-1, -2, -3),
		1L << 26, 1L << 38, 1L << 51, 1L << 62,
		-1L >>> 26, -1L >>> 12, -1L,
	};

	/** Section coordinates: straddle the 20-bit Y and 22-bit X/Z fields. */
	static final int[] SECTIONS = {
		0, 1, -1, 15, 16, -16, 2047, -2048, 1048575, -1048576,
		2097151, -2097152, 4194303, -4194304,
		Integer.MAX_VALUE, Integer.MIN_VALUE,
	};

	/** AABB corner pairs. NaN and Inf first: the constructor sorts, so they matter. */
	static final double[][] EDGES = {
		{0.0, 1.0}, {-1.0, 0.0}, {1.0, -1.0}, {-0.0, 0.0},
		{Double.NaN, 1.0}, {1.0, Double.NaN}, {Double.NaN, Double.NaN},
		{Double.POSITIVE_INFINITY, 1.0}, {Double.NEGATIVE_INFINITY, 1.0},
		{Double.MIN_VALUE, Double.MAX_VALUE},
	};

	/** Smaller corner pairs for the groups whose cross product would be O(n^4). */
	static final double[][] EDGES2 = {
		{0.0, 1.0}, {1.0, -1.0}, {Double.NaN, 1.0},
		{Double.POSITIVE_INFINITY, 1.0}, {Double.MIN_VALUE, Double.MAX_VALUE},
	};

	static final double[] PROBES = {
		0.0, -0.0, 0.5, 1.0, -1.0, 2.0, 0.9999999999, 1.0000000001, Double.NaN,
	};

	static final Direction[] DIRECTIONS = Direction.values();
	static final Rotation[] ROTATIONS = Rotation.values();
	static final Direction.Axis[] AXES = Direction.Axis.values();
	static final Direction.Plane[] PLANES = Direction.Plane.values();
}

final class Batch2Oracle {

	// =========================================================================
	// BlockPos
	// =========================================================================
	static void blockPos(Out o) {
		// Their own group because every other packing group is meaningless if these
		// are wrong: PACKED_HORIZONTAL_LENGTH is derived at class-init time from
		// Mth.log2(smallestEncompassingPowerOfTwo(30_000_000)).
		o.fn("blockpos.constants", "", "i32 i32 i32 i32 i32 i32");
		o.row(Out.join(
				Out.i32(BlockPos.PACKED_HORIZONTAL_LENGTH),
				Out.i32(BlockPos.PACKED_Y_LENGTH),
				Out.i32(BlockPos.MAX_HORIZONTAL_COORDINATE),
				Out.i32(BlockPos.ZERO.getX()),
				Out.i32(BlockPos.ZERO.getY()),
				Out.i32(BlockPos.ZERO.getZ())),
			Out.str("constants"));

		// PACK and UNPACK ARE SEPARATE ON PURPOSE. A round-trip test
		// `of(asLong(x,y,z)) == (x,y,z)` passes for many wrong shift constants,
		// because a pack and its matching unpack are self-consistent. Only comparing
		// against the jar's `asLong` AND its `getX/getY/getZ` separately proves the
		// layout is the one vanilla uses.
		o.fn("blockpos.asLong", "i32 i32 i32", "i64");
		for (int x : V2.INTS3) {
			for (int y : V2.INTS3) {
				for (int z : V2.INTS3) {
					o.row(Out.join(Out.i32(x), Out.i32(y), Out.i32(z)), Out.i64(BlockPos.asLong(x, y, z)));
				}
			}
		}

		o.fn("blockpos.getXYZ", "i64", "i32 i32 i32");
		for (long p : V2.PACKED) {
			o.row(Out.i64(p), Out.join(Out.i32(BlockPos.getX(p)), Out.i32(BlockPos.getY(p)),
				Out.i32(BlockPos.getZ(p))));
		}

		o.fn("blockpos.of", "i64", "i32 i32 i32");
		for (long p : V2.PACKED) {
			BlockPos b = BlockPos.of(p);
			o.row(Out.i64(p), out3i(b));
		}

		o.fn("blockpos.offsetLong", "i64 i32 i32 i32", "i64");
		for (long p : V2.PACKED) {
			for (int[] d : DELTAS) {
				o.row(Out.join(Out.i64(p), Out.i32(d[0]), Out.i32(d[1]), Out.i32(d[2])),
					Out.i64(BlockPos.offset(p, d[0], d[1], d[2])));
			}
		}

		o.fn("blockpos.offsetLongDir", "i64 i32", "i64");
		for (long p : V2.PACKED) {
			for (int di = 0; di < V2.DIRECTIONS.length; di++) {
				o.row(Out.join(Out.i64(p), Out.i32(di)), Out.i64(BlockPos.offset(p, V2.DIRECTIONS[di])));
			}
		}

		o.fn("blockpos.getFlatIndex", "i64", "i64");
		for (long p : V2.PACKED) {
			o.row(Out.i64(p), Out.i64(BlockPos.getFlatIndex(p)));
		}

		o.fn("blockpos.containing", "f64 f64 f64", "i32 i32 i32");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					BlockPos b = BlockPos.containing(x, y, z);
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)),
						Out.join(Out.i32(b.getX()), Out.i32(b.getY()), Out.i32(b.getZ())));
				}
			}
		}

		o.fn("blockpos.offset", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int[] d : DELTAS) {
				BlockPos b = new BlockPos(p[0], p[1], p[2]);
				o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]),
					Out.i32(d[0]), Out.i32(d[1]), Out.i32(d[2])),
					out3i(b.offset(d[0], d[1], d[2])));
			}
		}

		o.fn("blockpos.subtract", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int[] d : DELTAS) {
				BlockPos b = new BlockPos(p[0], p[1], p[2]);
				o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]),
					Out.i32(d[0]), Out.i32(d[1]), Out.i32(d[2])),
					out3i(b.subtract(new Vec3i(d[0], d[1], d[2]))));
			}
		}

		o.fn("blockpos.cross", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int[] d : DELTAS) {
				BlockPos b = new BlockPos(p[0], p[1], p[2]);
				o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]),
					Out.i32(d[0]), Out.i32(d[1]), Out.i32(d[2])),
					out3i(b.cross(new Vec3i(d[0], d[1], d[2]))));
			}
		}

		o.fn("blockpos.multiply", "i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int s : SCALES) {
				o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]), Out.i32(s)),
					out3i(new BlockPos(p[0], p[1], p[2]).multiply(s)));
			}
		}

		o.fn("blockpos.atY", "i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int s : SCALES) {
				o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]), Out.i32(s)),
					out3i(new BlockPos(p[0], p[1], p[2]).atY(s)));
			}
		}

		o.fn("blockpos.minmax", "i32 i32 i32 i32 i32 i32", "i32 i32 i32 i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int[] d : DELTAS) {
				BlockPos a = new BlockPos(p[0], p[1], p[2]);
				BlockPos b = new BlockPos(d[0], d[1], d[2]);
				o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]),
					Out.i32(d[0]), Out.i32(d[1]), Out.i32(d[2])),
					Out.join(out3i(BlockPos.min(a, b)), out3i(BlockPos.max(a, b))));
			}
		}

		o.fn("blockpos.relativeDir", "i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int di = 0; di < V2.DIRECTIONS.length; di++) {
				o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]), Out.i32(di)),
					out3i(new BlockPos(p[0], p[1], p[2]).relative(V2.DIRECTIONS[di])));
			}
		}

		o.fn("blockpos.relativeDirSteps", "i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int di = 0; di < V2.DIRECTIONS.length; di++) {
				for (int s : SCALES) {
					o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]), Out.i32(di), Out.i32(s)),
						out3i(new BlockPos(p[0], p[1], p[2]).relative(V2.DIRECTIONS[di], s)));
				}
			}
		}

		o.fn("blockpos.relativeAxis", "i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int ai = 0; ai < V2.AXES.length; ai++) {
				for (int s : SCALES) {
					o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]), Out.i32(ai), Out.i32(s)),
						out3i(new BlockPos(p[0], p[1], p[2]).relative(V2.AXES[ai], s)));
				}
			}
		}

		o.fn("blockpos.rotate", "i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int ri = 0; ri < V2.ROTATIONS.length; ri++) {
				o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]), Out.i32(ri)),
					out3i(new BlockPos(p[0], p[1], p[2]).rotate(V2.ROTATIONS[ri])));
			}
		}

		o.fn("blockpos.facing", "i32 i32 i32", "i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32");
		for (int[] p : CORNERS) {
			BlockPos b = new BlockPos(p[0], p[1], p[2]);
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2])),
				Out.join(out3i(b.above()), out3i(b.below()), out3i(b.north()),
					out3i(b.south()), out3i(b.west()), out3i(b.east())));
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2])),
				Out.join(out3i(b.above(4)), out3i(b.below(4)), out3i(b.north(4)),
					out3i(b.south(4)), out3i(b.west(4)), out3i(b.east(4))));
		}

		o.fn("blockpos.hashCode", "i32 i32 i32", "i32");
		for (int[] p : CORNERS) {
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2])),
				Out.i32(new BlockPos(p[0], p[1], p[2]).hashCode()));
		}

		o.fn("blockpos.equals", "i32 i32 i32 i32 i32 i32", "bool");
		for (int[] p : CORNERS) {
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]),
					Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2])),
				Out.b(new BlockPos(p[0], p[1], p[2]).equals(new BlockPos(p[0], p[1], p[2]))));
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]),
					Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2] + 1)),
				Out.b(new BlockPos(p[0], p[1], p[2]).equals(new BlockPos(p[0], p[1], p[2] + 1))));
		}

		o.fn("blockpos.toString", "i32 i32 i32", "str");
		for (int[] p : CORNERS) {
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2])),
				Out.str(new BlockPos(p[0], p[1], p[2]).toString()));
		}

		o.fn("blockpos.clampLocationWithin", "i32 f64", "f64 f64 f64");
		for (int x : new int[] {0, 1, -1, 30_000_000, -30_000_000}) {
			for (double v : V2.DOUBLES) {
				o.row(Out.join(Out.i32(x), Out.f64(v)),
					out3(new BlockPos(x, 0, x).clampLocationWithin(new Vec3(v, v, v))));
			}
		}

		// Deprecated in Java but still live, and a 4-element SEQUENCE whose order
		// callers rely on.
		o.fn("blockpos.squareOutSouthEast", "i32 i32 i32", "i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32");
		for (int[] p : CORNERS) {
			List<BlockPos> sq = BlockPos.squareOutSouthEast(new BlockPos(p[0], p[1], p[2])).toList();
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2])),
				Out.join(out3i(sq.get(0)), out3i(sq.get(1)), out3i(sq.get(2)), out3i(sq.get(3))));
		}

		blockPosIterators(o);
		blockPosMutable(o);
		blockPosTraversal(o);
	}

	private static final int[][] DELTAS = {
		{0, 0, 0}, {1, 0, 0}, {-1, 0, 0}, {0, 1, 0}, {0, 0, -1}, {5, -5, 5},
	};

	private static final int[] SCALES = {0, 1, -1, 2, -2, 3, 16, -16, Integer.MAX_VALUE, Integer.MIN_VALUE};

	/** Base positions: one per interesting axis value, crossed with the others. */
	private static final int[][] CORNERS = buildCorners();

	private static int[][] buildCorners() {
		int[] axis = {0, 1, -1, 15, -16, 2047, -2048, 30_000_000, -30_000_000, -64, 320,
			Integer.MAX_VALUE, Integer.MIN_VALUE, 1 << 23, -(1 << 23)};
		int[] other = {0, 1, -1};
		List<int[]> out = new ArrayList<>();
		for (int a : axis) {
			for (int b : other) {
				for (int c : other) {
					out.add(new int[] {a, b, c});
					out.add(new int[] {b, a, c});
					out.add(new int[] {b, c, a});
				}
			}
		}
		return out.toArray(new int[0][]);
	}

	/**
	 * The iterator families.
	 *
	 * IMPORTANT, and the reason these are worth their own groups: every one of these
	 * Java iterators returns the SAME `MutableBlockPos` instance on every step -- a
	 * reused cursor. A caller that stores the reference rather than the value sees
	 * every element collapse to the LAST one. The Rust port yields owned values, so
	 * that aliasing is not reproducible; what is pinned here is the SEQUENCE OF VALUES
	 * and its ORDER, which is what every caller in vanilla actually consumes. The
	 * aliasing is documented rather than silently dropped.
	 */
	static void blockPosIterators(Out o) {
		o.fn("blockpos.betweenClosed", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] a : new int[][] {{0, 0, 0}, {1, 1, 1}, {5, 0, 3}, {3, 0, 5}, {-2, 4, 7}}) {
			for (int[] b : new int[][] {{0, 0, 0}, {1, 2, 3}, {4, 4, 4}}) {
				emitSeq(o, a, b, (x, y) -> BlockPos.betweenClosed(x, y), 200);
			}
		}
		// Inverted argument order: betweenClosed normalises with min/max, so
		// betweenClosed(3,3,3, 0,0,0) must equal betweenClosed(0,0,0, 3,3,3).
		for (int[] a : new int[][] {{3, 3, 3}, {7, 1, 4}}) {
			for (int[] b : new int[][] {{0, 0, 0}, {-5, 1, 2}}) {
				emitSeq(o, a, b, (x, y) -> BlockPos.betweenClosed(x, y), 200);
			}
		}

		o.fn("blockpos.betweenClosedAABB", "f64 f64 f64 f64 f64 f64", "i32 i32 i32");
		double[][] boxes = {
			{0, 0, 0, 1, 1, 1}, {0, 0, 0, 2, 2, 2}, {-1.5, -1.5, -1.5, 1.5, 1.5, 1.5},
			{0, 0, 0, 0, 0, 0}, {-0.5, 0.5, -0.5, 0.5, 0.5, 0.5}, {0.25, 0.25, 0.25, 1.75, 1.75, 1.75},
		};
		for (double[] bb : boxes) {
			List<BlockPos> list = new ArrayList<>();
			BlockPos.betweenClosed(new AABB(bb[0], bb[1], bb[2], bb[3], bb[4], bb[5])).forEach(list::add);
			String args = Out.join(Out.f64(bb[0]), Out.f64(bb[1]), Out.f64(bb[2]),
				Out.f64(bb[3]), Out.f64(bb[4]), Out.f64(bb[5]));
			for (int i = 0; i < Math.min(list.size(), 200); i++) {
				o.row(args, out3i(list.get(i)));
			}
		}

		// A shell walk with a z-mirror pass: the ORDER is the whole point and is not
		// sorted.
		o.fn("blockpos.withinManhattan", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] c : new int[][] {{0, 0, 0}, {1, 2, 3}, {-5, -5, -5}, {30_000_000, 0, -30_000_000}}) {
			for (int[] r : new int[][] {{0, 0, 0}, {1, 1, 1}, {2, 1, 3}, {3, 0, 0}, {0, 2, 0}, {2, 2, 2}, {4, 2, 1}}) {
				emitSeq(o, c, new int[] {c[0] + r[0], c[1] + r[1], c[2] + r[2]},
					(x, y) -> BlockPos.withinManhattan(x, r[0], r[1], r[2]), 200);
			}
		}

		o.fn("blockpos.neighborColumn", "i32 i32 i32 i32", "i32 i32 i32");
		for (int[] a : new int[][] {{0, 0, 0}, {1, 2, 3}, {-4, 10, 7}}) {
			for (int endY : new int[] {0, 5, -5, 20, -20, 1}) {
				List<BlockPos> list = new ArrayList<>();
				BlockPos.neighborColumn(a[0], a[1], a[2], endY).forEach(list::add);
				for (int i = 0; i < Math.min(list.size(), 200); i++) {
					o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]), Out.i32(endY)),
						out3i(list.get(i)));
				}
			}
		}

		// spiralAround yields MutableBlockPos, so a caller MUST copy each element.
		o.fn("blockpos.spiralAround", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] c : new int[][] {{0, 0, 0}, {1, 2, 3}, {-5, 64, 7}}) {
			for (int radius : new int[] {0, 1, 2, 3}) {
				for (int d1 = 0; d1 < V2.DIRECTIONS.length; d1++) {
					for (int d2 = 0; d2 < V2.DIRECTIONS.length; d2++) {
						Direction a1 = V2.DIRECTIONS[d1], a2 = V2.DIRECTIONS[d2];
						if (a1.getAxis() == a2.getAxis()) {
							continue; // Validate rejects same-axis pairs.
						}
						List<BlockPos> list = new ArrayList<>();
						BlockPos center = new BlockPos(c[0], c[1], c[2]);
						for (BlockPos.MutableBlockPos p : BlockPos.spiralAround(center, radius, a1, a2)) {
							list.add(new BlockPos(p)); // copy: the cursor is reused
						}
						String args = Out.join(Out.i32(c[0]), Out.i32(c[1]), Out.i32(c[2]),
							Out.i32(radius), Out.i32(d1), Out.i32(d2));
						for (int i = 0; i < Math.min(list.size(), 120); i++) {
							o.row(args, out3i(list.get(i)));
						}
					}
				}
			}
		}

		o.fn("blockpos.spiralAroundError", "str", "str");
		try {
			BlockPos.spiralAround(BlockPos.ZERO, 1, Direction.UP, Direction.DOWN);
			o.row(Out.str("no throw"), Out.str("no throw"));
		} catch (RuntimeException ex) {
			o.row(Out.str(ex.getClass().getName()), Out.str(ex.getMessage()));
		}

		// The axis ORDER depends on the direction vector.
		o.fn("blockpos.betweenCornersInDirection", "i32 i32 i32 i32 i32 i32 f64 f64 f64", "i32 i32 i32");
		double[][] dirs = {
			{1, 0, 0}, {-1, 0, 0}, {0, 1, 0}, {0, -1, 0}, {0, 0, 1}, {0, 0, -1},
			{1, 1, 1}, {-1, 1, -1}, {1, 0, 2}, {-2, 0, 1}, {0.5, 0.5, 0.5}, {0, 0, 0},
		};
		for (int[] a : new int[][] {{0, 0, 0}, {2, 3, 4}, {-1, -1, -1}}) {
			for (int[] b : new int[][] {{0, 0, 0}, {1, 2, 3}, {3, 0, 1}, {-2, -3, -4}}) {
				for (double[] d : dirs) {
					List<BlockPos> list = new ArrayList<>();
					BlockPos.betweenCornersInDirection(
						new BlockPos(a[0], a[1], a[2]), new BlockPos(b[0], b[1], b[2]),
						new Vec3(d[0], d[1], d[2])).forEach(list::add);
					String args = Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]),
						Out.i32(b[0]), Out.i32(b[1]), Out.i32(b[2]),
						Out.f64(d[0]), Out.f64(d[1]), Out.f64(d[2]));
					for (int i = 0; i < Math.min(list.size(), 120); i++) {
						o.row(args, out3i(list.get(i)));
					}
				}
			}
		}

		// SEEDED, full sequence: both the draw ORDER (x, then y, then z) and the
		// width/height/depth bounds are load-bearing.
		o.fn("blockpos.randomBetweenClosed", "i64", "i32 i32 i32");
		for (long seed : V2.SEEDS) {
			RandomSource r = RandomSource.create(seed);
			List<BlockPos> list = new ArrayList<>();
			BlockPos.randomBetweenClosed(r, 12, -3, -2, -1, 4, 5, 6).forEach(list::add);
			for (int i = 0; i < list.size(); i++) {
				o.row(Out.i64(seed), out3i(list.get(i)));
			}
		}

		// width/height/depth of exactly 1: `nextInt(1)` still consumes a draw, which
		// shows up in the NEXT element. Pinning the degenerate case catches a port
		// that "optimises" the draw away.
		o.fn("blockpos.randomBetweenClosedDegenerate", "i64", "i32 i32 i32");
		for (long seed : V2.SEEDS) {
			RandomSource r = RandomSource.create(seed);
			List<BlockPos> list = new ArrayList<>();
			BlockPos.randomBetweenClosed(r, 6, 5, 5, 5, 5, 5, 5).forEach(list::add);
			for (int i = 0; i < list.size(); i++) {
				o.row(Out.i64(seed), out3i(list.get(i)));
			}
		}

		o.fn("blockpos.randomInCube", "i64", "i32 i32 i32");
		for (long seed : V2.SEEDS) {
			RandomSource r = RandomSource.create(seed);
			List<BlockPos> list = new ArrayList<>();
			BlockPos.randomInCube(r, 10, new BlockPos(0, 0, 0), 2).forEach(list::add);
			for (int i = 0; i < list.size(); i++) {
				o.row(Out.i64(seed), out3i(list.get(i)));
			}
		}
	}

	/** findClosestMatch returns the FIRST hit in withinManhattan order, not the nearest. */
	static void blockPosTraversal(Out o) {
		o.fn("blockpos.findClosestMatch", "i32 i32 i32 i32 i32", "bool i32 i32 i32");
		for (int[] c : new int[][] {{0, 0, 0}, {5, -5, 5}}) {
			for (int[] r : new int[][] {{1, 1, 1}, {2, 2, 2}, {3, 1, 2}}) {
				for (int limit : new int[] {0, 1, 2, 5, 10}) {
					Optional<BlockPos> found = BlockPos.findClosestMatch(
						new BlockPos(c[0], c[1], c[2]), r[0], r[1],
						// Matches at a KNOWN offset from the origin, so the answer depends
						// on traversal order rather than on luck.
						p -> p.getX() == c[0] + limit && p.getZ() == c[2] - limit);
					o.row(Out.join(Out.i32(c[0]), Out.i32(c[1]), Out.i32(c[2]),
							Out.i32(r[0]), Out.i32(r[1])),
						found.map(v -> Out.join(Out.b(true), Out.i32(v.getX()), Out.i32(v.getY()),
								Out.i32(v.getZ())))
							.orElseGet(() -> Out.join(Out.b(false), Out.i32(0), Out.i32(0), Out.i32(0))));
				}
			}
		}

		// Exercises the visited-set (keyed on asLong), the depth cap, the maxCount
		// early exit, and the three node statuses.
		o.fn("blockpos.breadthFirstTraversal", "i32 i32 i32 i32 i32 i32", "i32 i32 i32 i32 i32 i32 i32 i32 i32 i32");
		for (int[] start : new int[][] {{0, 0, 0}, {3, -2, 7}}) {
			for (int maxDepth : new int[] {0, 1, 2, 3}) {
				for (int maxCount : new int[] {1, 3, 10, 1000}) {
					for (int mode = 0; mode < 3; mode++) {
						int[] order = new int[64];
						int[] vi = {0};
						final int m = mode;
						int count = BlockPos.breadthFirstTraversal(
							new BlockPos(start[0], start[1], start[2]), maxDepth, maxCount,
							(pos, consumer) -> {
								for (Direction d : Direction.values()) {
									consumer.accept(pos.relative(d));
								}
							},
							pos -> {
								if (vi[0] < order.length) {
									order[vi[0]++] = pos.getX() * 1000000 + pos.getY() * 1000 + pos.getZ();
								}
								if (m == 0) return BlockPos.TraversalNodeStatus.ACCEPT;
								if (m == 1) return BlockPos.TraversalNodeStatus.SKIP;
								// STOP on the 3rd visit, to exercise the early break.
								return vi[0] >= 3 ? BlockPos.TraversalNodeStatus.STOP
									: BlockPos.TraversalNodeStatus.ACCEPT;
							});
						o.row(Out.join(Out.i32(start[0]), Out.i32(start[1]), Out.i32(start[2]),
								Out.i32(maxDepth), Out.i32(maxCount), Out.i32(mode)),
							Out.join(Out.i32(count),
								Out.i32(vi[0] > 0 ? order[0] : 0), Out.i32(vi[0] > 1 ? order[1] : 0),
								Out.i32(vi[0] > 2 ? order[2] : 0), Out.i32(vi[0] > 3 ? order[3] : 0),
								Out.i32(vi[0] > 4 ? order[4] : 0), Out.i32(vi[0] > 5 ? order[5] : 0),
								Out.i32(vi[0] > 6 ? order[6] : 0), Out.i32(vi[0] > 7 ? order[7] : 0),
								Out.i32(vi[0] > 8 ? order[8] : 0)));
					}
				}
			}
		}
	}

	static void blockPosMutable(Out o) {
		o.fn("blockpos.mutableSet", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]), Out.i32(p[2]),
					Out.i32(p[1]), Out.i32(p[0])),
				out3i(new BlockPos.MutableBlockPos(p[0], p[1], p[2]).set(p[2], p[1], p[0])));
		}

		o.fn("blockpos.mutableMove", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int[] d : DELTAS) {
				o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]),
						Out.i32(d[0]), Out.i32(d[1]), Out.i32(d[2])),
					out3i(new BlockPos.MutableBlockPos(p[0], p[1], p[2]).move(d[0], d[1], d[2])));
			}
		}

		o.fn("blockpos.mutableMoveDir", "i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int di = 0; di < V2.DIRECTIONS.length; di++) {
				for (int s : new int[] {0, 1, -1, 16}) {
					o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]), Out.i32(di), Out.i32(s)),
						out3i(new BlockPos.MutableBlockPos(p[0], p[1], p[2]).move(V2.DIRECTIONS[di], s)));
				}
			}
		}

		o.fn("blockpos.mutableSetWithOffset", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int[] d : DELTAS) {
				String args = Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]),
					Out.i32(d[0]), Out.i32(d[1]), Out.i32(d[2]));
				Vec3i base = new Vec3i(p[2], p[1], p[0]);
				o.row(args, out3i(new BlockPos.MutableBlockPos(p[0], p[1], p[2])
					.setWithOffset(base, d[0], d[1], d[2])));
				o.row(args, out3i(new BlockPos.MutableBlockPos(p[0], p[1], p[2])
					.setWithOffset(base, new Vec3i(d[0], d[1], d[2]))));
			}
		}

		o.fn("blockpos.mutableSetWithOffsetDir", "i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int di = 0; di < V2.DIRECTIONS.length; di++) {
				o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]), Out.i32(di)),
					out3i(new BlockPos.MutableBlockPos(p[0], p[1], p[2])
						.setWithOffset(new Vec3i(p[2], p[1], p[0]), V2.DIRECTIONS[di])));
			}
		}

		o.fn("blockpos.mutableClamp", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] p : CORNERS) {
			for (int ai = 0; ai < V2.AXES.length; ai++) {
				for (int[] lim : new int[][] {{-1, 1}, {0, 0}, {-2048, 2047}, {5, -5}}) {
					o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2]), Out.i32(ai),
							Out.i32(lim[0]), Out.i32(lim[1])),
						out3i(new BlockPos.MutableBlockPos(p[0], p[1], p[2])
							.clamp(V2.AXES[ai], lim[0], lim[1])));
				}
			}
		}

		// The OVERRIDES. MutableBlockPos#offset/relative/rotate/multiply call super and
		// then `.immutable()`, so they return an IMMUTABLE BlockPos while `this` stays
		// mutable. That asymmetry is easy to get wrong, and so is the fact that
		// `this` is left UNCHANGED -- all six columns are emitted so both are visible.
		o.fn("blockpos.mutableDetach", "i32 i32 i32", "i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32");
		for (int[] p : CORNERS) {
			BlockPos.MutableBlockPos m = new BlockPos.MutableBlockPos(p[0], p[1], p[2]);
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2])),
				Out.join(out3i(m.offset(1, 0, 0)),
					out3i(m.relative(Direction.UP, 2)),
					out3i(m.relative(Direction.Axis.X, 3)),
					out3i(m.rotate(Rotation.CLOCKWISE_90)),
					out3i(m.multiply(2)),
					out3i(m.immutable()),
					out3i(m)));
		}

		o.fn("blockpos.mutableSetPacked", "i64", "i32 i32 i32");
		for (long p : V2.PACKED) {
			o.row(Out.i64(p), out3i(new BlockPos.MutableBlockPos().set(p)));
		}

		o.fn("blockpos.mutableSetDouble", "f64 f64 f64", "i32 i32 i32");
		for (double x : V2.DOUBLES) {
			o.row(Out.join(Out.f64(x), Out.f64(x + 0.5), Out.f64(x)),
				out3i(new BlockPos.MutableBlockPos().set(x, x + 0.5, x)));
		}

		o.fn("blockpos.mutableCtor", "f64 f64 f64", "i32 i32 i32");
		for (double x : V2.DOUBLES) {
			o.row(Out.join(Out.f64(x), Out.f64(x), Out.f64(x)),
				out3i(new BlockPos.MutableBlockPos(x, x, x)));
		}
	}

	// =========================================================================
	// ChunkPos
	// =========================================================================
	static void chunkPos(Out o) {
		// REGION_BITS and REGION_MASK are PRIVATE in 26.2, so they are read by
		// reflection -- still the jar's value, still not hardcoded here.
		o.fn("chunkpos.constants", "", "i32 i32 i32 i32 i32 i32 i64 i32 i32");
		o.row(Out.join(
				Out.i32(priv(ChunkPos.class, "REGION_BITS")), Out.i32(ChunkPos.REGION_SIZE),
				Out.i32(priv(ChunkPos.class, "REGION_MASK")), Out.i32(ChunkPos.REGION_MAX_INDEX),
				Out.i32(ChunkPos.ZERO.x()), Out.i32(ChunkPos.ZERO.z()),
				Out.i64(ChunkPos.INVALID_CHUNK_POS), Out.i32(1875066), Out.i32(-559038737)),
			Out.str("constants"));

		o.fn("chunkpos.pack", "i32 i32", "i64");
		for (int x : V2.INTS3) {
			for (int z : V2.INTS3) {
				o.row(Out.join(Out.i32(x), Out.i32(z)), Out.i64(ChunkPos.pack(x, z)));
			}
		}

		o.fn("chunkpos.unpackRoundTrip", "i32 i32", "i32 i32");
		for (int x : V2.INTS3) {
			for (int z : V2.INTS3) {
				o.row(Out.join(Out.i32(x), Out.i32(z)), out2i(ChunkPos.unpack(ChunkPos.pack(x, z))));
			}
		}

		o.fn("chunkpos.hash", "i32 i32", "i32");
		for (int x : V2.INTS) {
			for (int z : new int[] {0, 1, -1, 31, -32, 30_000_000, -30_000_000}) {
				o.row(Out.join(Out.i32(x), Out.i32(z)), Out.i32(ChunkPos.hash(x, z)));
			}
		}

		o.fn("chunkpos.packBlockPos", "i32 i32 i32", "i64 i32 i32");
		for (int x : V2.INTS3) {
			for (int z : V2.INTS3) {
				o.row(Out.join(Out.i32(x), Out.i32(0), Out.i32(z)),
					Out.join(Out.i64(ChunkPos.pack(new BlockPos(x, 0, z))),
						Out.i32(ChunkPos.containing(new BlockPos(x, 0, z)).x()),
						Out.i32(ChunkPos.containing(new BlockPos(x, 0, z)).z())));
			}
		}

		o.fn("chunkpos.unpack", "i64", "i32 i32");
		for (long p : V2.PACKED) {
			o.row(Out.i64(p), out2i(ChunkPos.unpack(p)));
		}

		o.fn("chunkpos.getXZ", "i64", "i32 i32");
		for (long p : V2.PACKED) {
			o.row(Out.i64(p), Out.join(Out.i32(ChunkPos.getX(p)), Out.i32(ChunkPos.getZ(p))));
		}

		o.fn("chunkpos.fromSectionNode", "i64", "i64");
		for (long p : V2.PACKED) {
			o.row(Out.i64(p), Out.i64(ChunkPos.fromSectionNode(p)));
		}

		o.fn("chunkpos.regionOfPacked", "i64", "i32 i32");
		for (long p : V2.PACKED) {
			o.row(Out.i64(p), Out.join(Out.i32(ChunkPos.getRegionX(p)), Out.i32(ChunkPos.getRegionZ(p))));
		}

		o.fn("chunkpos.hashCode", "i32 i32", "i32");
		for (int[] p : CHUNK_CORNERS) {
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1])), Out.i32(new ChunkPos(p[0], p[1]).hashCode()));
		}

		o.fn("chunkpos.toString", "i32 i32", "str");
		for (int[] p : CHUNK_CORNERS) {
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1])), Out.str(new ChunkPos(p[0], p[1]).toString()));
		}

		o.fn("chunkpos.equals", "i32 i32 i32 i32", "bool");
		for (int[] p : CHUNK_CORNERS) {
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[0]), Out.i32(p[1])),
				Out.b(new ChunkPos(p[0], p[1]).equals(new ChunkPos(p[0], p[1]))));
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[0]), Out.i32(p[1] + 1)),
				Out.b(new ChunkPos(p[0], p[1]).equals(new ChunkPos(p[0], p[1] + 1))));
		}

		// isValid reads ChunkPyramid.MAX_CHUNK_COORDINATE_VALUE, which is computed from
		// the built-in registry and therefore needs Bootstrap to have run.
		o.fn("chunkpos.isValid", "i32 i32", "bool");
		for (int[] p : CHUNK_CORNERS) {
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1])), Out.b(new ChunkPos(p[0], p[1]).isValid()));
		}

		o.fn("chunkpos.blockCoords", "i32 i32", "i32 i32 i32 i32 i32 i32 i32 i32 i32");
		for (int[] p : CHUNK_CORNERS) {
			ChunkPos c = new ChunkPos(p[0], p[1]);
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1])),
				Out.join(Out.i32(c.getMinBlockX()), Out.i32(c.getMaxBlockX()),
					Out.i32(c.getMiddleBlockX()), Out.i32(c.getBlockX(5)),
					Out.i32(c.getMinBlockZ()), Out.i32(c.getMaxBlockZ()),
					Out.i32(c.getMiddleBlockZ()), Out.i32(c.getBlockZ(5)),
					Out.i32(c.getWorldPosition().getY())));
		}

		o.fn("chunkpos.region", "i32 i32", "i32 i32 i32 i32");
		for (int[] p : CHUNK_CORNERS) {
			ChunkPos c = new ChunkPos(p[0], p[1]);
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1])),
				Out.join(Out.i32(c.getRegionX()), Out.i32(c.getRegionZ()),
					Out.i32(c.getRegionLocalX()), Out.i32(c.getRegionLocalZ())));
		}

		o.fn("chunkpos.distances", "i32 i32 i32 i32", "bool i32 i32 i32");
		for (int[] p : CHUNK_CORNERS) {
			for (int[] o2 : new int[][] {{0, 0}, {1, 1}, {-1, -1}, {31, 0}, {15, 15}}) {
				ChunkPos c = new ChunkPos(p[0], p[1]);
				o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(o2[0]), Out.i32(o2[1])),
					Out.join(Out.b(c.contains(new BlockPos(p[0] * 16 + o2[0], 0, p[1] * 16 + o2[1]))),
						Out.i32(c.getChessboardDistance(new ChunkPos(o2[0], o2[1]))),
						Out.i32(c.distanceSquared(new ChunkPos(o2[0], o2[1]))),
						Out.i32(c.distanceSquared(ChunkPos.pack(o2[0], o2[1])))));
			}
		}

		o.fn("chunkpos.minMaxFromRegion", "i32 i32", "i32 i32 i32 i32");
		for (int rx : new int[] {0, 1, -1, 31, -32, 1_000_000, -1_000_000,
				Integer.MAX_VALUE, Integer.MIN_VALUE}) {
			o.row(Out.join(Out.i32(rx), Out.i32(rx)),
				Out.join(out2i(ChunkPos.minFromRegion(rx, rx)), out2i(ChunkPos.maxFromRegion(rx, rx))));
		}

		// rangeClosed: a boustrophedon (x-major, z alternating), which is neither
		// row-major nor sorted.
		o.fn("chunkpos.rangeClosed", "i32 i32 i32", "i32 i32");
		for (int[] c : new int[][] {{0, 0}, {5, -3}}) {
			for (int r : new int[] {0, 1, 2, 3}) {
				List<ChunkPos> list = new ArrayList<>();
				ChunkPos.rangeClosed(new ChunkPos(c[0], c[1]), r).forEach(list::add);
				for (int i = 0; i < Math.min(list.size(), 60); i++) {
					o.row(Out.join(Out.i32(c[0]), Out.i32(c[1]), Out.i32(r)), out2i(list.get(i)));
				}
			}
		}

		o.fn("chunkpos.rangeClosedFromTo", "i32 i32 i32 i32", "i32 i32");
		for (int[] a : new int[][] {{0, 0}, {2, 1}, {-1, -1}, {3, 0}}) {
			for (int[] b : new int[][] {{0, 0}, {1, 2}, {3, -1}}) {
				List<ChunkPos> list =
					ChunkPos.rangeClosed(new ChunkPos(a[0], a[1]), new ChunkPos(b[0], b[1])).toList();
				for (int i = 0; i < Math.min(list.size(), 60); i++) {
					o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(b[0]), Out.i32(b[1])),
						out2i(list.get(i)));
				}
			}
		}
	}

	private static final int[][] CHUNK_CORNERS = {
		{0, 0}, {1, 1}, {-1, -1}, {31, 0}, {0, 31}, {-32, 32}, {15, 15},
		{30_000_000, -30_000_000}, {-30_000_000, 30_000_000},
		{Integer.MAX_VALUE, Integer.MIN_VALUE}, {Integer.MIN_VALUE, Integer.MAX_VALUE},
	};

	// =========================================================================
	// SectionPos
	// =========================================================================
	static void sectionPos(Out o) {
		o.fn("sectionpos.constants", "", "i32 i32 i32 i32 i32 i32 i32 i32 i32");
		o.row(Out.join(
				Out.i32(SectionPos.SECTION_BITS), Out.i32(SectionPos.SECTION_SIZE),
				Out.i32(SectionPos.SECTION_BLOCK_COUNT), Out.i32(SectionPos.SECTION_MASK),
				Out.i32(SectionPos.SECTION_HALF_SIZE), Out.i32(SectionPos.SECTION_MAX_INDEX),
				Out.i32(22), Out.i32(20), Out.i32(22)),
			Out.str("constants"));

		// The layout constants are private, so the layout is recovered by observing
		// BEHAVIOUR: pack a range of section coords, then unpack them separately.
		o.fn("sectionpos.asLong", "i32 i32 i32", "i64");
		for (int x : V2.SECTIONS) {
			for (int y : V2.SECTIONS) {
				o.row(Out.join(Out.i32(x), Out.i32(y), Out.i32(x)),
					Out.i64(SectionPos.asLong(x, y, x)));
			}
		}

		o.fn("sectionpos.getXYZ", "i64", "i32 i32 i32");
		for (long p : V2.PACKED) {
			o.row(Out.i64(p), Out.join(Out.i32(SectionPos.x(p)), Out.i32(SectionPos.y(p)),
				Out.i32(SectionPos.z(p))));
		}

		o.fn("sectionpos.ofLong", "i64", "i32 i32 i32");
		for (long p : V2.PACKED) {
			o.row(Out.i64(p), out3s(SectionPos.of(p)));
		}

		o.fn("sectionpos.offsetLong", "i64 i32 i32 i32", "i64");
		for (long p : V2.PACKED) {
			for (int[] d : DELTAS) {
				o.row(Out.join(Out.i64(p), Out.i32(d[0]), Out.i32(d[1]), Out.i32(d[2])),
					Out.i64(SectionPos.offset(p, d[0], d[1], d[2])));
			}
		}

		o.fn("sectionpos.offsetLongDir", "i64 i32", "i64");
		for (long p : V2.PACKED) {
			for (int di = 0; di < V2.DIRECTIONS.length; di++) {
				o.row(Out.join(Out.i64(p), Out.i32(di)), Out.i64(SectionPos.offset(p, V2.DIRECTIONS[di])));
			}
		}

		o.fn("sectionpos.blockToSection", "i64", "i64");
		for (long p : V2.PACKED) {
			o.row(Out.i64(p), Out.i64(SectionPos.blockToSection(p)));
		}

		o.fn("sectionpos.getZeroNode", "i64", "i64");
		for (long p : V2.PACKED) {
			o.row(Out.i64(p), Out.i64(SectionPos.getZeroNode(p)));
		}

		o.fn("sectionpos.sectionToChunk", "i64", "i64");
		for (long p : V2.PACKED) {
			o.row(Out.i64(p), Out.i64(SectionPos.sectionToChunk(p)));
		}

		o.fn("sectionpos.getZeroNodeXZ", "i32 i32", "i64");
		for (int x : V2.SECTIONS) {
			for (int z : V2.SECTIONS) {
				o.row(Out.join(Out.i32(x), Out.i32(z)), Out.i64(SectionPos.getZeroNode(x, z)));
			}
		}

		o.fn("sectionpos.asLongBlockPos", "i32 i32 i32", "i64");
		for (int bx : V2.INTS) {
			o.row(Out.join(Out.i32(bx), Out.i32(-64), Out.i32(bx)),
				Out.i64(SectionPos.asLong(new BlockPos(bx, -64, bx))));
		}

		// `blockToSectionCoord` is an ARITHMETIC `>> 4` (floor divide by 16) and
		// `sectionToBlockCoord` is `<< 4`. Both wrap, so `Integer.MIN_VALUE >> 4` is
		// +134217728 rather than negative.
		o.fn("sectionpos.blockToSectionCoord", "i32", "i32");
		for (int x : V2.INTS) {
			o.row(Out.i32(x), Out.i32(SectionPos.blockToSectionCoord(x)));
		}

		o.fn("sectionpos.sectionRelative", "i32", "i32");
		for (int x : V2.INTS) {
			o.row(Out.i32(x), Out.i32(SectionPos.sectionRelative(x)));
		}

		o.fn("sectionpos.sectionToBlockCoord", "i32", "i32 i32");
		for (int x : V2.INTS) {
			o.row(Out.i32(x), Out.join(Out.i32(SectionPos.sectionToBlockCoord(x)),
				Out.i32(SectionPos.sectionToBlockCoord(x, 5))));
		}

		o.fn("sectionpos.posToSectionCoord", "f64", "i32");
		for (double d : V2.DOUBLES) {
			o.row(Out.f64(d), Out.i32(SectionPos.posToSectionCoord(d)));
		}

		o.fn("sectionpos.blockToSectionCoordD", "f64", "i32");
		for (double d : V2.DOUBLES) {
			o.row(Out.f64(d), Out.i32(SectionPos.blockToSectionCoord(d)));
		}

		// sectionRelativePos packs x<<8 | z<<4 | y into a SHORT.
		o.fn("sectionpos.sectionRelativePos", "i32 i32 i32", "i32 i32 i32 i32");
		for (int x : new int[] {0, 1, 15, 16, -1, 2047, -2048, 30_000_000}) {
			for (int y : new int[] {0, 1, 15, -1, 64, -64}) {
				for (int z : new int[] {0, 1, 15, -1}) {
					short rel = SectionPos.sectionRelativePos(new BlockPos(x, y, z));
					o.row(Out.join(Out.i32(x), Out.i32(y), Out.i32(z)),
						Out.join(Out.i32(rel), Out.i32(SectionPos.sectionRelativeX(rel)),
							Out.i32(SectionPos.sectionRelativeY(rel)), Out.i32(SectionPos.sectionRelativeZ(rel))));
				}
			}
		}

		o.fn("sectionpos.blockCoords", "i32 i32 i32", "i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32");
		for (int x : V2.SECTIONS) {
			for (int y : new int[] {0, 1, -1, 15, -16}) {
				for (int z : new int[] {0, 1, -1, 31}) {
					SectionPos s = SectionPos.of(x, y, z);
					o.row(Out.join(Out.i32(x), Out.i32(y), Out.i32(z)),
						Out.join(Out.i32(s.x()), Out.i32(s.y()), Out.i32(s.z()),
							Out.i32(s.minBlockX()), Out.i32(s.minBlockY()), Out.i32(s.minBlockZ()),
							Out.i32(s.maxBlockX()), Out.i32(s.maxBlockY()), Out.i32(s.maxBlockZ()),
							Out.i32(s.origin().getY()), Out.i32(s.center().getX()),
							Out.i32(s.chunk().x()), Out.i32(s.chunk().z())));
				}
			}
		}

		o.fn("sectionpos.instanceMisc", "i32 i32 i32", "i32 i32 i32 i32 i32 i32 i32 i32 i32");
		for (int x : V2.SECTIONS) {
			for (int y : new int[] {0, 1, -16}) {
				SectionPos s = SectionPos.of(x, y, x);
				SectionPos fromBlock = SectionPos.of(new BlockPos(x * 16, y * 16, x * 16));
				SectionPos fromVec = SectionPos.of(new Vec3(x * 16.0, y * 16.0, x * 16.0));
				o.row(Out.join(Out.i32(x), Out.i32(y), Out.i32(x)),
					Out.join(
						Out.i32(s.offset(1, 0, 0).x()), Out.i32(s.offset(0, 1, 0).y()),
						Out.i32(s.offset(0, 0, 1).z()), Out.i32(s.offset(0, 0, 0).x()),
						Out.i32(s.asLong() == SectionPos.asLong(x, y, x) ? 1 : 0),
						Out.i32(s.origin().getX()), Out.i32(s.origin().getZ()),
						Out.i32(s.center().getY()), Out.i32(s.center().getZ()),
						Out.i32(fromBlock.x()), Out.i32(fromBlock.y()), Out.i32(fromBlock.z()),
						Out.i32(fromVec.x()), Out.i32(fromVec.y()), Out.i32(fromVec.z()),
						Out.i32(SectionPos.of(new ChunkPos(x, x), y).y())));
			}
		}

		o.fn("sectionpos.relativeToBlock", "i32 i32 i32 i32", "i32 i32 i32 i32 i32 i32 i32 i32 i32");
		for (int x : V2.SECTIONS) {
			for (int y : new int[] {0, 1, -16}) {
				SectionPos s = SectionPos.of(x, y, x);
				for (int rel : new int[] {0, 1, -1, 0x1234, 0xFFF0, 0x8000, 0x7FFF}) {
					short r = (short) rel;
					o.row(Out.join(Out.i32(x), Out.i32(y), Out.i32(x), Out.i32(rel)),
						Out.join(Out.i32(s.relativeToBlockX(r)), Out.i32(s.relativeToBlockY(r)),
							Out.i32(s.relativeToBlockZ(r)),
							Out.i32(s.relativeToBlockPos(r).getX()),
							Out.i32(s.relativeToBlockPos(r).getY()),
							Out.i32(s.relativeToBlockPos(r).getZ()),
							Out.i32(SectionPos.sectionRelativeX(r)),
							Out.i32(SectionPos.sectionRelativeY(r)),
							Out.i32(SectionPos.sectionRelativeZ(r))));
				}
			}
		}

		// blocksInside is 4096 positions; the first few and the wrap points are what
		// differ between implementations, so index rather than enumerate in full.
		o.fn("sectionpos.blocksInside", "i32 i32 i32 i32", "i32 i32 i32");
		for (int[] s : new int[][] {{0, 0, 0}, {1, 2, 3}, {-1, -1, -1}}) {
			List<BlockPos> inside = SectionPos.of(s[0], s[1], s[2]).blocksInside().toList();
			for (int i : new int[] {0, 1, 2, 3, 15, 16, 100, 2047, 2048, 4095}) {
				if (i < inside.size()) {
					o.row(Out.join(Out.i32(s[0]), Out.i32(s[1]), Out.i32(s[2]), Out.i32(i)),
						out3i(inside.get(i)));
				}
			}
		}

		o.fn("sectionpos.cube", "i32 i32 i32 i32", "i32 i32 i32");
		for (int[] c : new int[][] {{0, 0, 0}, {5, -3, 7}}) {
			for (int r : new int[] {0, 1, 2}) {
				List<SectionPos> list = SectionPos.cube(SectionPos.of(c[0], c[1], c[2]), r).toList();
				for (int i = 0; i < Math.min(list.size(), 120); i++) {
					o.row(Out.join(Out.i32(c[0]), Out.i32(c[1]), Out.i32(c[2]), Out.i32(r), Out.i32(i)),
						out3s(list.get(i)));
				}
			}
		}

		o.fn("sectionpos.aroundChunk", "i32 i32 i32", "i32 i32 i32");
		for (int[] c : new int[][] {{0, 0}, {5, -3}}) {
			for (int r : new int[] {0, 1, 2}) {
				List<SectionPos> list =
					SectionPos.aroundChunk(new ChunkPos(c[0], c[1]), r, -4, 4).toList();
				for (int i = 0; i < Math.min(list.size(), 120); i++) {
					o.row(Out.join(Out.i32(c[0]), Out.i32(c[1]), Out.i32(r), Out.i32(i)),
						out3s(list.get(i)));
				}
			}
		}

		o.fn("sectionpos.betweenClosedStream", "i32 i32 i32 i32 i32 i32", "i32 i32 i32");
		for (int[] a : new int[][] {{0, 0, 0}, {-1, -2, -3}}) {
			for (int[] b : new int[][] {{0, 0, 0}, {1, 2, 3}, {2, 0, 1}}) {
				List<SectionPos> list =
					SectionPos.betweenClosedStream(a[0], a[1], a[2], b[0], b[1], b[2]).toList();
				for (int i = 0; i < Math.min(list.size(), 120); i++) {
					o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2]),
							Out.i32(b[0]), Out.i32(b[1]), Out.i32(b[2]), Out.i32(i)),
						out3s(list.get(i)));
				}
			}
		}

		// aroundAndAtBlockPos: 1 or 8 longs, order is x-outer, y, z-inner.
		o.fn("sectionpos.aroundAndAtBlockPos", "i32 i32 i32", "i32 i64 i64 i64 i64 i64 i64 i64 i64 i64");
		for (int[] a : new int[][] {
			{0, 0, 0}, {1, 1, 1}, {15, 15, 15}, {16, 16, 16}, {-1, -1, -1},
			{30_000_000, 0, -30_000_000}, {-2048, 2047, 0},
			{Integer.MAX_VALUE, Integer.MIN_VALUE, 0},
		}) {
			List<Long> got = new ArrayList<>();
			SectionPos.aroundAndAtBlockPos(new BlockPos(a[0], a[1], a[2]), got::add);
			o.row(Out.join(Out.i32(a[0]), Out.i32(a[1]), Out.i32(a[2])),
				Out.join(Out.i32(got.size()),
					Out.i64(got.size() > 0 ? got.get(0) : 0L), Out.i64(got.size() > 1 ? got.get(1) : 0L),
					Out.i64(got.size() > 2 ? got.get(2) : 0L), Out.i64(got.size() > 3 ? got.get(3) : 0L),
					Out.i64(got.size() > 4 ? got.get(4) : 0L), Out.i64(got.size() > 5 ? got.get(5) : 0L),
					Out.i64(got.size() > 6 ? got.get(6) : 0L), Out.i64(got.size() > 7 ? got.get(7) : 0L)));
		}
	}

	// =========================================================================
	// Vec3
	// =========================================================================
	static void vec3(Out o) {
		o.fn("vec3.identity", "f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)), out3(new Vec3(x, y, z)));
				}
			}
		}

		o.fn("vec3.hashCode", "f64 f64 f64", "i32");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)),
						Out.i32(new Vec3(x, y, z).hashCode()));
				}
			}
		}

		o.fn("vec3.equals", "f64 f64 f64 f64 f64 f64", "bool");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					Vec3 a = new Vec3(x, y, z);
					for (double s : V2.DOUBLES3) {
						o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.f64(s), Out.f64(y), Out.f64(z)),
							Out.b(a.equals(new Vec3(s, y, z))));
						o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.f64(x), Out.f64(x), Out.f64(z)),
							Out.b(a.equals(new Vec3(x, x, z))));
						o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.f64(x), Out.f64(y), Out.f64(z)),
							Out.b(a.equals(new Vec3(x, y, z))));
					}
				}
			}
		}

		// `Double.compare(NaN, NaN) == 0` is TRUE, so a Vec3 of NaN equals itself, and
		// +0.0 equals -0.0. Both surprise enough to deserve explicit rows rather than
		// being left for the reader to notice inside `vec3.equals`.
		o.fn("vec3.equalsSpecial", "f64 f64 f64 f64 f64 f64", "bool");
		double nan = Double.NaN;
		o.row(Out.join(Out.f64(nan), Out.f64(nan), Out.f64(nan),
				Out.f64(nan), Out.f64(nan), Out.f64(nan)),
			Out.b(new Vec3(nan, nan, nan).equals(new Vec3(nan, nan, nan))));
		o.row(Out.join(Out.f64(0.0), Out.f64(0.0), Out.f64(0.0),
				Out.f64(-0.0), Out.f64(-0.0), Out.f64(-0.0)),
			Out.b(new Vec3(0.0, 0.0, 0.0).equals(new Vec3(-0.0, -0.0, -0.0))));
		o.row(Out.join(Out.f64(0.0), Out.f64(0.0), Out.f64(0.0),
				Out.f64(0.0), Out.f64(0.0), Out.f64(1.0)),
			Out.b(new Vec3(0.0, 0.0, 0.0).equals(new Vec3(0.0, 0.0, 1.0))));

		o.fn("vec3.addScalar", "f64 f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.f64(s)),
							out3(new Vec3(x, y, z).add(s)));
					}
				}
			}
		}

		o.fn("vec3.subtractScalar", "f64 f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.f64(s)),
							out3(new Vec3(x, y, z).subtract(s)));
					}
				}
			}
		}

		o.fn("vec3.scale", "f64 f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.f64(s)),
							out3(new Vec3(x, y, z).scale(s)));
					}
				}
			}
		}

		o.fn("vec3.addVec3", "f64 f64 f64 f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(six(x, y, z, s, s, s), out3(new Vec3(x, y, z).add(new Vec3(s, s, s))));
					}
				}
			}
		}

		o.fn("vec3.subtractVec3", "f64 f64 f64 f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(six(x, y, z, s, s, s), out3(new Vec3(x, y, z).subtract(new Vec3(s, s, s))));
					}
				}
			}
		}

		o.fn("vec3.multiplyVec3", "f64 f64 f64 f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(six(x, y, z, s, s, s), out3(new Vec3(x, y, z).multiply(new Vec3(s, s, s))));
					}
				}
			}
		}

		o.fn("vec3.vectorTo", "f64 f64 f64 f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(six(x, y, z, s, s, s), out3(new Vec3(x, y, z).vectorTo(new Vec3(s, s, s))));
					}
				}
			}
		}

		o.fn("vec3.reverse", "f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)),
						out3(new Vec3(x, y, z).reverse()));
				}
			}
		}

		o.fn("vec3.horizontal", "f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)),
						out3(new Vec3(x, y, z).horizontal()));
				}
			}
		}

		o.fn("vec3.dot", "f64 f64 f64 f64 f64 f64", "f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(six(x, y, z, s, y, z), Out.f64(new Vec3(x, y, z).dot(new Vec3(s, y, z))));
					}
				}
			}
		}

		o.fn("vec3.cross", "f64 f64 f64 f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(six(x, y, z, s, y, z), out3(new Vec3(x, y, z).cross(new Vec3(s, y, z))));
					}
				}
			}
		}

		// `length()`/`normalize()` go through Math.sqrt, which is correctly rounded, so
		// these SHOULD be bit-exact and act as a control group. `normalize` also has
		// the `dist < 1.0E-5F` ZERO shortcut where a FLOAT literal is compared against
		// a double -- that widening is load-bearing.
		o.fn("vec3.length", "f64 f64 f64", "f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)),
						Out.f64(new Vec3(x, y, z).length()));
				}
			}
		}

		o.fn("vec3.lengthSqr", "f64 f64 f64", "f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)),
						Out.f64(new Vec3(x, y, z).lengthSqr()));
				}
			}
		}

		o.fn("vec3.normalize", "f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)),
						out3(new Vec3(x, y, z).normalize()));
				}
			}
		}

		o.fn("vec3.horizontalDistance", "f64 f64 f64", "f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)),
						Out.f64(new Vec3(x, y, z).horizontalDistance()));
				}
			}
		}

		o.fn("vec3.horizontalDistanceSqr", "f64 f64 f64", "f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)),
						Out.f64(new Vec3(x, y, z).horizontalDistanceSqr()));
				}
			}
		}

		o.fn("vec3.distanceTo", "f64 f64 f64 f64 f64 f64", "f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(six(x, y, z, s, y, z),
							Out.f64(new Vec3(x, y, z).distanceTo(new Vec3(s, y, z))));
					}
				}
			}
		}

		o.fn("vec3.distanceToSqr", "f64 f64 f64 f64 f64 f64", "f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(six(x, y, z, s, y, z),
							Out.f64(new Vec3(x, y, z).distanceToSqr(new Vec3(s, y, z))));
						o.row(six(x, y, z, s, y, z), Out.f64(new Vec3(x, y, z).distanceToSqr(s, y, z)));
					}
				}
			}
		}

		o.fn("vec3.closerThan", "f64 f64 f64 f64 f64 f64", "bool");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(six(x, y, z, s, y, z),
							Out.b(new Vec3(x, y, z).closerThan(new Vec3(s, y, z), 1.0)));
					}
				}
			}
		}

		o.fn("vec3.closerThanXZ", "f64 f64 f64 f64 f64 f64 f64 f64", "bool");
		// HEADER ARITY CORRECTION (session 06): the row below carries 8 arguments, not 6.
		// closerThan(Vec3, double, double) is called with both distances appended to the six
		// coordinates, so 8 args reach the row.
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(six(x, y, z, s, y, z) + " " + Out.f64(1.0) + " " + Out.f64(1.0),
							Out.b(new Vec3(x, y, z).closerThan(new Vec3(s, y, z), 1.0, 1.0)));
					}
				}
			}
		}

		// Rotations from Mth's EMBEDDED tables (not host transcendentals), so these
		// SHOULD be bit-exact and prove the tables ported.
		o.fn("vec3.xRot", "f64 f64 f64 f32", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (float r : V2.FLOATS2) {
						o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.f32(r)),
							out3(new Vec3(x, y, z).xRot(r)));
					}
				}
			}
		}

		o.fn("vec3.yRot", "f64 f64 f64 f32", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (float r : V2.FLOATS2) {
						o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.f32(r)),
							out3(new Vec3(x, y, z).yRot(r)));
					}
				}
			}
		}

		o.fn("vec3.zRot", "f64 f64 f64 f32", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (float r : V2.FLOATS2) {
						o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.f32(r)),
							out3(new Vec3(x, y, z).zRot(r)));
					}
				}
			}
		}

		o.fn("vec3.rotateClockwise90", "f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)),
						out3(new Vec3(x, y, z).rotateClockwise90()));
				}
			}
		}

		o.fn("vec3.directionFromRotation", "f32 f32", "f64 f64 f64");
		for (float rx : V2.FLOATS) {
			for (float ry : V2.FLOATS) {
				o.row(Out.join(Out.f32(rx), Out.f32(ry)), out3(Vec3.directionFromRotation(rx, ry)));
			}
		}

		o.fn("vec3.get", "f64 f64 f64 i32", "f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (int ai = 0; ai < V2.AXES.length; ai++) {
						o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.i32(ai)),
							Out.f64(new Vec3(x, y, z).get(V2.AXES[ai])));
					}
				}
			}
		}

		o.fn("vec3.with", "f64 f64 f64 i32 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (int ai = 0; ai < V2.AXES.length; ai++) {
						for (double v : V2.DOUBLES3) {
							o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.i32(ai), Out.f64(v)),
								out3(new Vec3(x, y, z).with(V2.AXES[ai], v)));
						}
					}
				}
			}
		}

		o.fn("vec3.relative", "f64 f64 f64 i32 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (int di = 0; di < V2.DIRECTIONS.length; di++) {
						for (double dist : V2.DOUBLES3) {
							o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.i32(di), Out.f64(dist)),
								out3(new Vec3(x, y, z).relative(V2.DIRECTIONS[di], dist)));
						}
					}
				}
			}
		}

		o.fn("vec3.lerp", "f64 f64 f64 f64 f64 f64", "f64 f64 f64");
		// HEADER ARITY CORRECTION (session 06): the row below carries 8 arguments, not 6.
		// lerp(Vec3, double) takes a vector plus a scalar: 6, not 7.
		for (double a : V2.DOUBLES3) {
			for (double x : V2.DOUBLES3) {
				for (double y : V2.DOUBLES3) {
					o.row(Out.join(Out.f64(a), Out.f64(x), Out.f64(y), Out.f64(x), Out.f64(y), Out.f64(y)),
						out3(new Vec3(x, y, y).lerp(new Vec3(x, y, y), a)));
				}
			}
		}

		o.fn("vec3.projectedOn", "f64 f64 f64 f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (double s : V2.DOUBLES3) {
						o.row(six(x, y, z, s, y, z),
							out3(new Vec3(x, y, z).projectedOn(new Vec3(s, y, z))));
					}
				}
			}
		}

		o.fn("vec3.isFinite", "f64 f64 f64", "bool");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)),
						Out.b(new Vec3(x, y, z).isFinite()));
				}
			}
		}

		o.fn("vec3.toString", "f64 f64 f64", "str");
		for (double x : V2.DOUBLES) {
			for (double y : new double[] {0.0, -1.0, 0.5}) {
				o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(x)),
					Out.str(new Vec3(x, y, x).toString()));
			}
		}

		o.fn("vec3.fromVec3i", "i32 i32 i32", "f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64");
		// HEADER ARITY CORRECTION (session 06): the row below carries 8 arguments, not 6.
		// NINE Vec3 values are emitted (atLowerCornerOf, atCenterOf, atBottomCenterOf,
			// upFromBottomCenterOf, the raw widening ctor, ZERO, X_AXIS, Y_AXIS, Z_AXIS)
			// = 27 components, not 17.
		for (int ix : V2.INTS) {
			for (int iy : new int[] {0, 1, -1, 64, -64}) {
				for (int iz : new int[] {0, 1, -1}) {
					Vec3i p = new Vec3i(ix, iy, iz);
					o.row(Out.join(Out.i32(ix), Out.i32(iy), Out.i32(iz)),
						Out.join(out3(Vec3.atLowerCornerOf(p)), out3(Vec3.atCenterOf(p)),
							out3(Vec3.atBottomCenterOf(p)), out3(Vec3.upFromBottomCenterOf(p, 0.25)),
							out3(new Vec3(p.getX(), p.getY(), p.getZ())),
							out3(Vec3.ZERO), out3(Vec3.X_AXIS), out3(Vec3.Y_AXIS), out3(Vec3.Z_AXIS)));
				}
			}
		}

		// align takes an EnumSet<Axis>, so the argument is an ASCII mask.
		o.fn("vec3.align", "f64 f64 f64 str", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					for (String mask : new String[] {"", "X", "Y", "Z", "XY", "XZ", "YZ", "XYZ"}) {
						EnumSet<Direction.Axis> set = EnumSet.noneOf(Direction.Axis.class);
						if (mask.contains("X")) set.add(Direction.Axis.X);
						if (mask.contains("Y")) set.add(Direction.Axis.Y);
						if (mask.contains("Z")) set.add(Direction.Axis.Z);
						o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.str(mask)),
							out3(new Vec3(x, y, z).align(set)));
					}
				}
			}
		}

		// `rotation()` calls Math.atan2 and Math.asin -- HOST TRANSCENDENTALS. Emitted
		// anyway; whether they match is not a guess, the golden says.
		o.fn("vec3.rotation", "f64 f64 f64", "f32 f32");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					Vec2 rot = new Vec3(x, y, z).rotation();
					o.row(Out.join(Out.f64(x), Out.f64(y), Out.f64(z)),
						Out.join(Out.f32(rot.x), Out.f32(rot.y)));
				}
			}
		}

		o.fn("vec3.applyLocalCoordinates", "f32 f32 f64 f64 f64", "f64 f64 f64");
		for (float rx : V2.FLOATS2) {
			for (float ry : V2.FLOATS2) {
				o.row(Out.join(Out.f32(rx), Out.f32(ry), Out.f64(0.0), Out.f64(0.0), Out.f64(1.0)),
					out3(Vec3.applyLocalCoordinatesToRotation(new Vec2(rx, ry), new Vec3(0.0, 0.0, 1.0))));
			}
		}

		o.fn("vec3.addLocalCoordinates", "f64 f64 f64 f64 f64 f64", "f64 f64 f64");
		for (double x : V2.DOUBLES3) {
			for (double y : V2.DOUBLES3) {
				for (double z : V2.DOUBLES3) {
					o.row(six(x, y, z, 1.0, 0.0, 0.0),
						out3(new Vec3(x, y, z).addLocalCoordinates(new Vec3(1.0, 0.0, 0.0))));
				}
			}
		}
	}

	private static String six(double x, double y, double z, double a, double b, double c) {
		return Out.join(Out.f64(x), Out.f64(y), Out.f64(z), Out.f64(a), Out.f64(b), Out.f64(c));
	}

	/**
	 * Reads a {@code private static} field off a game class by reflection, failing loudly.
	 *
	 * <p>Used only for {@code ARGB}'s two sRGB tables, which are {@code private static final
	 * byte[]} with no public accessor that enumerates them: {@code linearToSrgbChannel} takes a
	 * float and loses the index to {@code Mth.floor}. Reading the bytes directly is the only way
	 * to capture all 1024 entries of each, and half a table is worse than none, because
	 * {@code meanLinear}, {@code linearChannelMean} and {@code linearLerp} all index up to 1023.
	 *
	 * <p>A rename in a future Minecraft version turns this into a hard failure with the field
	 * name in the message, which is the intended behaviour: a golden that silently emits zeros
	 * because a lookup returned null would be much worse than a stopped build.
	 */
	private static Object readPrivateStatic(Class<?> owner, String field) {
		try {
			java.lang.reflect.Field f = owner.getDeclaredField(field);
			f.setAccessible(true);
			return f.get(null);
		} catch (ReflectiveOperationException | RuntimeException ex) {
			throw new IllegalStateException(
				"could not read " + owner.getName() + "#" + field
					+ " -- if Minecraft renamed or retyped it, fix this oracle rather than the port",
				ex);
		}
	}

	// =========================================================================
	// Vec2
	// =========================================================================
	static void vec2(Out o) {
		// `MIN` is `new Vec2(Float.MIN_VALUE, Float.MIN_VALUE)` -- the SMALLEST POSITIVE
		// normal, not the most negative. That is vanilla's intent (a
		// smallest-brightness sentinel) and is trivially easy to "fix" into
		// Float.MIN_NORMAL or -MAX_VALUE by mistake, so it is pinned.
		o.fn("vec2.constants", "", "f32 f32 f32 f32 f32 f32 f32 f32 f32 f32 f32 f32 f32 f32 f32 f32");
		o.row(Out.join(
				Out.f32(Vec2.ZERO.x), Out.f32(Vec2.ZERO.y),
				Out.f32(Vec2.ONE.x), Out.f32(Vec2.ONE.y),
				Out.f32(Vec2.UNIT_X.x), Out.f32(Vec2.UNIT_X.y),
				Out.f32(Vec2.NEG_UNIT_X.x), Out.f32(Vec2.NEG_UNIT_X.y),
				Out.f32(Vec2.UNIT_Y.x), Out.f32(Vec2.UNIT_Y.y),
				Out.f32(Vec2.NEG_UNIT_Y.x), Out.f32(Vec2.NEG_UNIT_Y.y),
				Out.f32(Vec2.MAX.x), Out.f32(Vec2.MAX.y),
				Out.f32(Vec2.MIN.x), Out.f32(Vec2.MIN.y)),
			Out.str("constants"));

		o.fn("vec2.lengths", "f32 f32", "f32 f32 f32 f32");
		for (float x : V2.FLOATS) {
			for (float y : V2.FLOATS) {
				Vec2 a = new Vec2(x, y);
				o.row(Out.join(Out.f32(x), Out.f32(y)),
					Out.join(Out.f32(a.length()), Out.f32(a.lengthSquared()),
						Out.f32(a.normalized().x), Out.f32(a.normalized().y)));
			}
		}

		o.fn("vec2.hashCode", "f32 f32", "i32");
		for (float x : V2.FLOATS) {
			for (float y : V2.FLOATS) {
				o.row(Out.join(Out.f32(x), Out.f32(y)), Out.i32(new Vec2(x, y).hashCode()));
			}
		}

		o.fn("vec2.equals", "f32 f32 f32 f32", "bool");
		for (float x : V2.FLOATS2) {
			for (float y : V2.FLOATS2) {
				Vec2 a = new Vec2(x, y);
				for (float s : V2.FLOATS2) {
					o.row(Out.join(Out.f32(x), Out.f32(y), Out.f32(s), Out.f32(y)),
						Out.b(a.equals(new Vec2(s, y))));
					o.row(Out.join(Out.f32(x), Out.f32(y), Out.f32(x), Out.f32(s)),
						Out.b(a.equals(new Vec2(x, s))));
					o.row(Out.join(Out.f32(x), Out.f32(y), Out.f32(x), Out.f32(y)),
						Out.b(a.equals(new Vec2(x, y))));
				}
			}
		}

		o.fn("vec2.scaleAdd", "f32 f32 f32 f32", "f32 f32 f32 f32 f32 f32");
		for (float x : V2.FLOATS2) {
			for (float y : V2.FLOATS2) {
				Vec2 a = new Vec2(x, y);
				for (float s : V2.FLOATS2) {
					Vec2 b = new Vec2(s, y);
					o.row(Out.join(Out.f32(x), Out.f32(y), Out.f32(s), Out.f32(y)),
						Out.join(Out.f32(a.scale(s).x), Out.f32(a.scale(s).y),
							Out.f32(a.dot(b)), Out.f32(a.add(b).x), Out.f32(a.add(b).y),
							Out.f32(a.distanceToSqr(b))));
				}
			}
		}

		o.fn("vec2.addScalarNegated", "f32 f32 f32", "f32 f32 f32 f32");
		for (float x : V2.FLOATS2) {
			for (float y : V2.FLOATS2) {
				Vec2 a = new Vec2(x, y);
				for (float s : V2.FLOATS2) {
					o.row(Out.join(Out.f32(x), Out.f32(y), Out.f32(s)),
						Out.join(Out.f32(a.add(s).x), Out.f32(a.add(s).y),
							Out.f32(a.negated().x), Out.f32(a.negated().y)));
				}
			}
		}

		// `rotate` uses Mth.cos/Mth.sin (the embedded tables) with a DOUBLE angle
		// narrowed to float on the way in.
		o.fn("vec2.rotate", "f32 f32 f64", "f32 f32");
		for (float x : V2.FLOATS2) {
			for (float y : V2.FLOATS2) {
				for (double ang : V2.DOUBLES3) {
					Vec2 r = new Vec2(x, y).rotate(ang);
					o.row(Out.join(Out.f32(x), Out.f32(y), Out.f64(ang)),
						Out.join(Out.f32(r.x), Out.f32(r.y)));
				}
			}
		}
	}

	// =========================================================================
	// AABB
	// =========================================================================
	static void aabb(Out o) {
		// The constructor SWAPS: `min = Math.min(a,b)`, `max = Math.max(a,b)`. So
		// `new AABB(1,0,0,-1,0,0)` is a valid box, and a naive port that keeps the
		// argument order produces inverted boxes that still pass some predicates.
		// Inverted and NaN pairs come first for that reason.
		o.fn("aabb.constructor", "f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				AABB box = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
				o.row(boxArgs(a, b), out6(box));
			}
		}

		o.fn("aabb.hashCode", "f64 f64 f64 f64 f64 f64", "i32");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				o.row(boxArgs(a, b), Out.i32(new AABB(a[0], b[0], a[1], b[1], a[1], b[0]).hashCode()));
			}
		}

		o.fn("aabb.toString", "f64 f64 f64 f64 f64 f64", "str");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				o.row(boxArgs(a, b), Out.str(new AABB(a[0], b[0], a[1], b[1], a[1], b[0]).toString()));
			}
		}

		o.fn("aabb.hasNaN", "f64 f64 f64 f64 f64 f64", "bool");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				o.row(boxArgs(a, b), Out.b(new AABB(a[0], b[0], a[1], b[1], a[1], b[0]).hasNaN()));
			}
		}

		o.fn("aabb.equals", "f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64", "bool");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				AABB box = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
				String args = boxArgs(a, b);
				o.row(args + " " + args, Out.b(box.equals(new AABB(a[0], b[0], a[1], b[1], a[1], b[0]))));
				o.row(args + " " + args, Out.b(box.equals(new AABB(b[0], a[0], b[1], a[1], b[0], a[1]))));
				o.row(args + " " + Out.join(Out.f64(0.0), Out.f64(0.0), Out.f64(0.0),
						Out.f64(1.0), Out.f64(0.0), Out.f64(1.0)),
					Out.b(box.equals(new AABB(0.0, 0.0, 0.0, 1.0, 0.0, 1.0))));
			}
		}

		o.fn("aabb.sizes", "f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				AABB box = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
				o.row(boxArgs(a, b), Out.join(Out.f64(box.getXsize()), Out.f64(box.getYsize()),
					Out.f64(box.getZsize()), Out.f64(box.getSize())));
			}
		}

		o.fn("aabb.centers", "f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				AABB box = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
				o.row(boxArgs(a, b), Out.join(out3(box.getCenter()), out3(box.getBottomCenter()),
					out3(box.getMinPosition()), out3(box.getMaxPosition())));
			}
		}

		// Reshaping, one group per operation. These build a NEW AABB through the
		// swapping constructor, so the result is not simply arithmetic on the corners:
		// `contract` can produce an inverted pair that the constructor then repairs.
		o.fn("aabb.contract", "f64 f64 f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (double[] d : V2.EDGES2) {
					AABB box = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
					o.row(boxArgs(a, b) + " " + deltaArgs(d), out6(box.contract(d[0], d[0], d[1])));
				}
			}
		}

		o.fn("aabb.expandTowards", "f64 f64 f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (double[] d : V2.EDGES2) {
					AABB box = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
					o.row(boxArgs(a, b) + " " + deltaArgs(d), out6(box.expandTowards(d[0], d[0], d[1])));
				}
			}
		}

		o.fn("aabb.inflate", "f64 f64 f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (double[] d : V2.EDGES2) {
					AABB box = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
					o.row(boxArgs(a, b) + " " + deltaArgs(d), out6(box.inflate(d[0], d[0], d[1])));
				}
			}
		}

		o.fn("aabb.deflate", "f64 f64 f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (double[] d : V2.EDGES2) {
					AABB box = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
					o.row(boxArgs(a, b) + " " + deltaArgs(d), out6(box.deflate(d[0], d[0], d[1])));
				}
			}
		}

		o.fn("aabb.move", "f64 f64 f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (double[] d : V2.EDGES2) {
					AABB box = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
					o.row(boxArgs(a, b) + " " + deltaArgs(d), out6(box.move(d[0], d[0], d[1])));
				}
			}
		}

		o.fn("aabb.moveVec3", "f64 f64 f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				for (double[] d : V2.EDGES2) {
					AABB box = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
					o.row(boxArgs(a, b) + " " + deltaArgs(d),
						out6(box.move(new Vec3(d[0], d[0], d[1]))));
				}
			}
		}

		o.fn("aabb.moveBlockPos", "f64 f64 f64 f64 f64 f64 i32", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				for (int bx : new int[] {0, 1, -1, 30_000_000, -30_000_000}) {
					AABB box = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
					o.row(boxArgs(a, b) + " " + Out.i32(bx), out6(box.move(new BlockPos(bx, 0, 0))));
				}
			}
		}

		o.fn("aabb.intersect", "f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f12", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				for (double[] c : V2.EDGES2) {
					for (double[] d : V2.EDGES2) {
						o.row(boxArgs(a, b) + " " + boxArgs(c, d),
							out6(new AABB(a[0], b[0], a[1], b[1], a[1], b[0])
								.intersect(new AABB(c[0], d[0], c[1], d[1], c[1], d[0]))));
					}
				}
			}
		}

		o.fn("aabb.minmax", "f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f12", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				for (double[] c : V2.EDGES2) {
					for (double[] d : V2.EDGES2) {
						o.row(boxArgs(a, b) + " " + boxArgs(c, d),
							out6(new AABB(a[0], b[0], a[1], b[1], a[1], b[0])
								.minmax(new AABB(c[0], d[0], c[1], d[1], c[1], d[0]))));
					}
				}
			}
		}

		o.fn("aabb.setMinX", "f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (double[] d : V2.EDGES2) {
					o.row(boxArgs(a, b) + " " + Out.f64(d[0]),
						out6(new AABB(a[0], b[0], a[1], b[1], a[1], b[0]).setMinX(d[0])));
				}
			}
		}

		o.fn("aabb.setMinY", "f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (double[] d : V2.EDGES2) {
					o.row(boxArgs(a, b) + " " + Out.f64(d[0]),
						out6(new AABB(a[0], b[0], a[1], b[1], a[1], b[0]).setMinY(d[0])));
				}
			}
		}

		o.fn("aabb.setMinZ", "f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (double[] d : V2.EDGES2) {
					o.row(boxArgs(a, b) + " " + Out.f64(d[0]),
						out6(new AABB(a[0], b[0], a[1], b[1], a[1], b[0]).setMinZ(d[0])));
				}
			}
		}

		o.fn("aabb.setMaxX", "f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (double[] d : V2.EDGES2) {
					o.row(boxArgs(a, b) + " " + Out.f64(d[0]),
						out6(new AABB(a[0], b[0], a[1], b[1], a[1], b[0]).setMaxX(d[0])));
				}
			}
		}

		o.fn("aabb.setMaxY", "f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (double[] d : V2.EDGES2) {
					o.row(boxArgs(a, b) + " " + Out.f64(d[0]),
						out6(new AABB(a[0], b[0], a[1], b[1], a[1], b[0]).setMaxY(d[0])));
				}
			}
		}

		o.fn("aabb.setMaxZ", "f64 f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (double[] d : V2.EDGES2) {
					o.row(boxArgs(a, b) + " " + Out.f64(d[0]),
						out6(new AABB(a[0], b[0], a[1], b[1], a[1], b[0]).setMaxZ(d[0])));
				}
			}
		}

		// `intersects` and `contains` use STRICT inequalities on both sides, so boxes
		// that merely TOUCH do not intersect. Boundary probes are in the corpus for
		// exactly that reason.
		o.fn("aabb.intersects", "f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f12", "bool");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				for (double[] c : V2.EDGES2) {
					for (double[] d : V2.EDGES2) {
						AABB p = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
						AABB q = new AABB(c[0], d[0], c[1], d[1], c[1], d[0]);
						o.row(boxArgs(a, b) + " " + boxArgs(c, d), Out.b(p.intersects(q)));
						o.row(boxArgs(a, b) + " " + boxArgs(c, d),
							Out.b(p.intersects(new Vec3(q.minX, q.minY, q.minZ),
								new Vec3(q.maxX, q.maxY, q.maxZ))));
					}
				}
			}
		}

		o.fn("aabb.contains", "f64 f64 f64 f64 f64 f64 f64 f64 f64", "bool");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				for (double pv : V2.PROBES) {
					AABB p = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
					o.row(boxArgs(a, b) + " " + Out.join(Out.f64(pv), Out.f64(pv), Out.f64(pv)),
						Out.b(p.contains(pv, pv, pv)));
				}
			}
		}

		o.fn("aabb.containsVec3", "f64 f64 f64 f64 f64 f64 f64 f64 f64", "bool");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				for (double pv : V2.PROBES) {
					AABB p = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
					o.row(boxArgs(a, b) + " " + Out.join(Out.f64(pv), Out.f64(pv), Out.f64(pv)),
						Out.b(p.contains(new Vec3(pv, pv, pv))));
				}
			}
		}

		o.fn("aabb.intersectsBlockPos", "f64 f64 f64 f64 f64 f64 i32", "bool");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				for (int bx : new int[] {0, 1, -1, 30_000_000}) {
					AABB p = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
					o.row(boxArgs(a, b) + " " + Out.i32(bx), Out.b(p.intersects(new BlockPos(bx, 0, 0))));
				}
			}
		}

		o.fn("aabb.distanceToSqrPoint", "f64 f64 f64 f64 f64 f64 f64 f64 f64", "f64");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				for (double pv : V2.PROBES) {
					AABB p = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
					o.row(boxArgs(a, b) + " " + Out.join(Out.f64(pv), Out.f64(pv), Out.f64(pv)),
						Out.f64(p.distanceToSqr(new Vec3(pv, pv, pv))));
				}
			}
		}

		o.fn("aabb.distanceToSqrBox", "f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f12", "f64");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				for (double[] c : V2.EDGES2) {
					for (double[] d : V2.EDGES2) {
						AABB p = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
						o.row(boxArgs(a, b) + " " + boxArgs(c, d),
							Out.f64(p.distanceToSqr(new AABB(c[0], d[0], c[1], d[1], c[1], d[0]))));
					}
				}
			}
		}

		o.fn("aabb.minAxis", "f64 f64 f64 f64 f64 f64 i32", "f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (int ai = 0; ai < V2.AXES.length; ai++) {
					o.row(boxArgs(a, b) + " " + Out.i32(ai),
						Out.f64(new AABB(a[0], b[0], a[1], b[1], a[1], b[0]).min(V2.AXES[ai])));
				}
			}
		}

		o.fn("aabb.maxAxis", "f64 f64 f64 f64 f64 f64 i32", "f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				for (int ai = 0; ai < V2.AXES.length; ai++) {
					o.row(boxArgs(a, b) + " " + Out.i32(ai),
						Out.f64(new AABB(a[0], b[0], a[1], b[1], a[1], b[0]).max(V2.AXES[ai])));
				}
			}
		}

		// `clip` is the most order-sensitive method in this batch: six sequential
		// `clipPoint` calls share ONE mutable `scaleReference` array, and the winner is
		// whichever plane matches LAST. So the hit POINT is emitted, not just a bool.
		o.fn("aabb.clipStatic", "f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f12", "bool f64 f64 f64");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				for (double[] c : V2.EDGES2) {
					for (double[] d : V2.EDGES2) {
						double minX = a[0], maxX = b[0], minY = c[0], maxY = d[0], minZ = a[1], maxZ = b[1];
						for (double pv : new double[] {0.0, 0.5, 2.0, -1.0}) {
							Vec3 from = new Vec3(pv, pv, pv);
							Vec3 to = new Vec3(-pv, 2.0 * pv, pv);
							o.row(clipArgs(minX, minY, minZ, maxX, maxY, maxZ, from, to),
								clipOut(AABB.clip(minX, minY, minZ, maxX, maxY, maxZ, from, to)));
						}
					}
				}
			}
		}

		o.fn("aabb.clipInstance", "f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f64 f12", "bool f64 f64 f64");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				AABB box = new AABB(a[0], b[0], a[1], b[1], a[1], b[0]);
				for (double pv : new double[] {0.0, 0.5, 2.0, -1.0}) {
					Vec3 from = new Vec3(pv, pv, pv);
					Vec3 to = new Vec3(-pv, 2.0 * pv, pv);
					o.row(boxArgs(a, b) + " " + Out.join(
							Out.f64(from.x), Out.f64(from.y), Out.f64(from.z),
							Out.f64(to.x), Out.f64(to.y), Out.f64(to.z)),
						clipOut(box.clip(from, to)));
				}
			}
		}

		o.fn("aabb.unitCubeFromLowerCorner", "f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES2) {
				o.row(Out.join(Out.f64(a[0]), Out.f64(a[1]), Out.f64(b[0])),
					out6(AABB.unitCubeFromLowerCorner(new Vec3(a[0], a[1], b[0]))));
			}
		}

		o.fn("aabb.ofSize", "f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				o.row(Out.join(Out.f64(a[0]), Out.f64(a[1]), Out.f64(b[0]),
						Out.f64(b[1]), Out.f64(a[1]), Out.f64(b[0])),
					out6(AABB.ofSize(new Vec3(a[0], a[1], b[0]), b[1], a[1], b[0])));
			}
		}

		o.fn("aabb.encapsulatingFullBlocks", "i32 i32 i32 i32 i32 i32", "f64 f64 f64 f64 f64 f64");
		for (int[] p1 : new int[][] {{0, 0, 0}, {-1, 2, -3}, {30_000_000, -64, -30_000_000}}) {
			for (int[] p2 : new int[][] {{0, 0, 0}, {1, 1, 1}, {-5, 7, 2}}) {
				o.row(Out.join(Out.i32(p1[0]), Out.i32(p1[1]), Out.i32(p1[2]),
						Out.i32(p2[0]), Out.i32(p2[1]), Out.i32(p2[2])),
					out6(AABB.encapsulatingFullBlocks(
						new BlockPos(p1[0], p1[1], p1[2]), new BlockPos(p2[0], p2[1], p2[2]))));
			}
		}

		o.fn("aabb.fromBlockPos", "i32 i32 i32", "f64 f64 f64 f64 f64 f64");
		for (int[] p : new int[][] {{0, 0, 0}, {-1, 2, -3}, {30_000_000, -64, -30_000_000},
				{Integer.MAX_VALUE, Integer.MIN_VALUE, 0}}) {
			o.row(Out.join(Out.i32(p[0]), Out.i32(p[1]), Out.i32(p[2])),
				out6(new AABB(new BlockPos(p[0], p[1], p[2]))));
		}

		o.fn("aabb.fromVec3", "f64 f64 f64 f64 f64 f64", "f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES2) {
			for (double[] b : V2.EDGES2) {
				o.row(Out.join(Out.f64(a[0]), Out.f64(a[1]), Out.f64(b[0]),
						Out.f64(b[1]), Out.f64(a[1]), Out.f64(b[0])),
					out6(new AABB(new Vec3(a[0], a[1], b[0]), new Vec3(b[1], a[1], b[0]))));
			}
		}

		// Builder. `build()` on an undefined Builder throws; the MESSAGE TEXT is part
		// of the contract, so it is captured verbatim.
		o.fn("aabb.builderError", "str", "str");
		try {
			new AABB.Builder().build();
			o.row(Out.str("no throw"), Out.str("no throw"));
		} catch (RuntimeException ex) {
			o.row(Out.str(ex.getClass().getName()), Out.str(ex.getMessage()));
		}

		o.fn("aabb.builder", "f64 f64 f64 f64 f64 f64", "bool f64 f64 f64 f64 f64 f64");
		for (double[] a : V2.EDGES) {
			for (double[] b : V2.EDGES) {
				AABB.Builder bd = new AABB.Builder();
				bd.include(new Vector3f((float) a[0], (float) a[1], (float) b[0]));
				bd.include(new Vector3f((float) b[0], (float) a[1], (float) b[1]));
				AABB built = bd.build();
				o.row(Out.join(Out.f64(a[0]), Out.f64(a[1]), Out.f64(b[0]),
						Out.f64(b[0]), Out.f64(a[1]), Out.f64(b[1])),
					Out.join(Out.b(bd.isDefined()),
						Out.f64(built.minX), Out.f64(built.minY), Out.f64(built.minZ),
						Out.f64(built.maxX), Out.f64(built.maxY), Out.f64(built.maxZ)));
			}
		}
	}

	private static String boxArgs(double[] a, double[] b) {
		return Out.join(Out.f64(a[0]), Out.f64(b[0]), Out.f64(a[1]), Out.f64(b[1]),
			Out.f64(a[1]), Out.f64(b[0]));
	}

	private static String deltaArgs(double[] d) {
		return Out.join(Out.f64(d[0]), Out.f64(d[0]), Out.f64(d[1]));
	}

	private static String clipArgs(double minX, double minY, double minZ,
			double maxX, double maxY, double maxZ, Vec3 from, Vec3 to) {
		return Out.join(Out.f64(minX), Out.f64(minY), Out.f64(minZ),
			Out.f64(maxX), Out.f64(maxY), Out.f64(maxZ),
			Out.f64(from.x), Out.f64(from.y), Out.f64(from.z),
			Out.f64(to.x), Out.f64(to.y), Out.f64(to.z));
	}

	private static String clipOut(Optional<Vec3> hit) {
		return hit.map(v -> Out.join(Out.b(true), Out.f64(v.x), Out.f64(v.y), Out.f64(v.z)))
			.orElseGet(() -> Out.join(Out.b(false), Out.f64(0.0), Out.f64(0.0), Out.f64(0.0)));
	}

	// =========================================================================
	// ARGB
	// =========================================================================
	static void argb(Out o) {
		int[] cs = {
			0, 1, -1, 255, 256, 0x7F7F7F7F, 0xFFFFFFFF, 0x00000000, 0xFF000000, 0x00FFFFFF,
			0x12345678, 0xDEADBEEF, 0x0A0B0C0D, 0x7F808180, Integer.MIN_VALUE, Integer.MAX_VALUE,
			0x01020304, 0xFFFEFFFE, 0x000100FF,
		};

		o.fn("argb.channels", "i32", "i32 i32 i32 i32");
		for (int c : cs) {
			o.row(Out.i32(c), Out.join(Out.i32(ARGB.alpha(c)), Out.i32(ARGB.red(c)),
				Out.i32(ARGB.green(c)), Out.i32(ARGB.blue(c))));
		}

		o.fn("argb.colorFloats", "i32", "f32 f32 f32 f32");
		for (int c : cs) {
			o.row(Out.i32(c), Out.join(Out.f32(ARGB.alphaFloat(c)), Out.f32(ARGB.redFloat(c)),
				Out.f32(ARGB.greenFloat(c)), Out.f32(ARGB.blueFloat(c))));
		}

		o.fn("argb.toABGR", "i32", "i32 i32");
		for (int c : cs) {
			o.row(Out.i32(c), Out.join(Out.i32(ARGB.toABGR(c)), Out.i32(ARGB.fromABGR(c))));
		}

		o.fn("argb.color4", "i32 i32 i32 i32", "i32 i32 i32 i32 i32 i32 i32 i32");
		for (int c : cs) {
			int a = ARGB.alpha(c), r = ARGB.red(c), g = ARGB.green(c), b = ARGB.blue(c);
			o.row(Out.join(Out.i32(a), Out.i32(r), Out.i32(g), Out.i32(b)),
				Out.join(Out.i32(ARGB.color(a, r, g, b)),
					Out.i32(ARGB.color(a, r)), Out.i32(ARGB.color(a, r | 0x100)),
					Out.i32(ARGB.opaque(c)), Out.i32(ARGB.transparent(c)),
					Out.i32(ARGB.white(a)), Out.i32(ARGB.black(a)), Out.i32(ARGB.gray(r))));
		}

		o.fn("argb.color3", "i32 i32 i32", "i32");
		for (int c : cs) {
			o.row(Out.join(Out.i32(ARGB.red(c)), Out.i32(ARGB.green(c)), Out.i32(ARGB.blue(c))),
				Out.i32(ARGB.color(ARGB.red(c), ARGB.green(c), ARGB.blue(c))));
		}

		o.fn("argb.colorFromVec3", "i32", "i32");
		for (int c : cs) {
			o.row(Out.i32(c), Out.i32(ARGB.color(
				new Vec3(ARGB.redFloat(c), ARGB.greenFloat(c), ARGB.blueFloat(c)))));
		}

		o.fn("argb.colorFloat", "f32", "i32 i32 i32 i32 i32");
		for (int c : cs) {
			for (float f : V2.FLOATS) {
				o.row(Out.f32(f), Out.join(
					Out.i32(ARGB.color(f, ARGB.red(c))),
					Out.i32(ARGB.white(f)), Out.i32(ARGB.black(f)), Out.i32(ARGB.gray(f)),
					Out.i32(ARGB.colorFromFloat(f, f, f, f))));
			}
		}

		o.fn("argb.as8BitChannel", "f32", "i32");
		for (float f : V2.FLOATS) {
			o.row(Out.f32(f), Out.i32(ARGB.as8BitChannel(f)));
		}

		o.fn("argb.scaleRGB", "i32 f32", "i32 i32");
		for (int c : cs) {
			for (float f : V2.FLOATS) {
				o.row(Out.join(Out.i32(c), Out.f32(f)),
					Out.join(Out.i32(ARGB.scaleRGB(c, f)), Out.i32(ARGB.scaleRGB(c, f, f, f))));
			}
		}

		o.fn("argb.scaleRGBInt", "i32 i32", "i32");
		for (int c : cs) {
			o.row(Out.join(Out.i32(c), Out.i32(ARGB.red(c))),
				Out.i32(ARGB.scaleRGB(c, (int) (float) ARGB.red(c))));
		}

		o.fn("argb.multiplyAlpha", "i32 f32", "i32 i32 i32 i32");
		for (int c : cs) {
			o.row(Out.join(Out.i32(c), Out.f32(ARGB.alphaFloat(c))),
				Out.join(Out.i32(ARGB.multiplyAlpha(c, ARGB.alphaFloat(c))),
					Out.i32(ARGB.multiplyAlpha(c, 0.0F)),
					Out.i32(ARGB.multiplyAlpha(0, 1.0F)),
					Out.i32(ARGB.multiplyAlpha(c, 2.0F))));
		}

		o.fn("argb.multiply", "i32 i32", "i32");
		for (int c1 : cs) {
			for (int c2 : cs) {
				o.row(Out.join(Out.i32(c1), Out.i32(c2)), Out.i32(ARGB.multiply(c1, c2)));
			}
		}

		o.fn("argb.addRgb", "i32 i32", "i32");
		for (int c1 : cs) {
			for (int c2 : cs) {
				o.row(Out.join(Out.i32(c1), Out.i32(c2)), Out.i32(ARGB.addRgb(c1, c2)));
			}
		}

		o.fn("argb.subtractRgb", "i32 i32", "i32");
		for (int c1 : cs) {
			for (int c2 : cs) {
				o.row(Out.join(Out.i32(c1), Out.i32(c2)), Out.i32(ARGB.subtractRgb(c1, c2)));
			}
		}

		o.fn("argb.alphaBlend", "i32 i32", "i32");
		for (int c1 : cs) {
			for (int c2 : cs) {
				o.row(Out.join(Out.i32(c1), Out.i32(c2)), Out.i32(ARGB.alphaBlend(c1, c2)));
			}
		}

		o.fn("argb.meanLinear", "i32 i32", "i32");
		for (int c1 : cs) {
			for (int c2 : cs) {
				o.row(Out.join(Out.i32(c1), Out.i32(c2)),
					Out.join(Out.i32(ARGB.meanLinear(c1, c2, ~c1, ~c2)),
						Out.i32(ARGB.meanLinear(c1, c1, c1, c1))));
			}
		}

		o.fn("argb.greyscaleAverage", "i32 i32", "i32");
		for (int c1 : cs) {
			for (int c2 : cs) {
				o.row(Out.join(Out.i32(c1), Out.i32(c2)),
					Out.join(Out.i32(ARGB.greyscale(c1)), Out.i32(ARGB.average(c1, c2))));
			}
		}

		o.fn("argb.srgbLerp", "f32 i32 i32", "i32");
		for (int c1 : cs) {
			for (float f : V2.FLOATS2) {
				o.row(Out.join(Out.f32(f), Out.i32(c1), Out.i32(~c1)), Out.i32(ARGB.srgbLerp(f, c1, ~c1)));
			}
		}

		// NEGATIVE ALPHAS ARE EXCLUDED HERE, and that is a vanilla behaviour rather than
		// a convenience. `linearLerp` does
		//
		//     LINEAR_TO_SRGB[Mth.lerpInt(alpha, SRGB_TO_LINEAR[red(p0)], ...)]
		//
		// and `lerpInt` is `(int)(start + alpha * (end - start))` with no clamping. For a
		// negative `alpha` and two channel values near 0 the interpolated index goes
		// BELOW ZERO, and vanilla 26.2 throws
		//
		//     ArrayIndexOutOfBoundsException: Index -1023 out of bounds for length 1024
		//
		// That is a real crash in vanilla, and the Rust port has to reproduce it rather
		// than quietly clamp -- a clamping "fix" would return a colour where the game
		// crashes. `argb.linearLerpThrows` pins exactly which alphas throw; see
		// DESIGN_DECISIONS.md (#argb-linear-lerp-negative-alpha).
		// ONLY alpha in [0,1] is safe. `linearLerp` does
		//
		//     LINEAR_TO_SRGB[Mth.lerpInt(alpha, SRGB_TO_LINEAR[red(p0)], ...)]
		//
		// and `lerpInt` is `(int)(start + alpha * (end - start))` with no clamping, so:
		//
		//   * negative alpha  -> index -1023, ArrayIndexOutOfBoundsException
		//   * alpha > 1       -> index 184140, ArrayIndexOutOfBoundsException
		//
		// Both are real crashes in vanilla 26.2. A clamping "port" would return a colour
		// where the game throws, so the Rust port must reproduce the throw.
		// `argb.linearLerpThrows` pins which alphas do what.
		o.fn("argb.linearLerp", "f32 i32 i32", "i32");
		for (int c1 : cs) {
			for (float f : new float[] {0.0F, 0.25F, 0.5F, 0.75F, 1.0F}) {
				o.row(Out.join(Out.f32(f), Out.i32(c1), Out.i32(~c1)), Out.i32(ARGB.linearLerp(f, c1, ~c1)));
			}
		}

		o.fn("argb.linearLerpThrows", "f32 i32 i32", "str");
		for (float f : new float[] {-1.0F, -0.25F, 0.0F, 0.25F, 0.5F, 1.0F, 2.0F, 180.0F, 360.0F, Float.NaN, Float.POSITIVE_INFINITY}) {
			try {
				ARGB.linearLerp(f, 0x7F7F7F7F, 0x0A0B0C0D);
				o.row(Out.join(Out.f32(f), Out.i32(0x7F7F7F7F), Out.i32(0x0A0B0C0D)), Out.str("ok"));
			} catch (RuntimeException ex) {
				o.row(Out.join(Out.f32(f), Out.i32(0x7F7F7F7F), Out.i32(0x0A0B0C0D)),
					Out.str(ex.getClass().getName() + ": " + ex.getMessage()));
			}
		}

		// The sRGB lookup tables are built with Math.pow, i.e. a HOST TRANSCENDENTAL.
		//
		// FIXED IN SESSION 09. The comment here used to say "Emitted in full", directly above a
		// loop reading `for (int ch = 0; ch < 256; ch++)`. The tables are `new byte[1024]`. So
		// this group covered a quarter of the domain while claiming to cover all of it, and
		// nothing could tell the difference: 256 rows of plausible-looking data reads exactly
		// like a passing group. That is the failure mode rule 2 at the top of this file warns
		// about, wearing a different hat.
		//
		// The reachable-index probe stays, now honestly labelled: `linearToSrgbChannel` takes a
		// float and does `LINEAR_TO_SRGB[Mth.floor(linear * 1023.0F)]`, so these rows pin the
		// floor behaviour and the two argument scales, NOT the table.
		o.fn("argb.srgbTables", "i32", "f32 i32 i32");
		for (int ch = 0; ch < 256; ch++) {
			o.row(Out.i32(ch), Out.join(
				Out.f32(ARGB.srgbToLinearChannel(ch)),
				Out.i32(ARGB.linearToSrgbChannel(ch / 1023.0F)),
				Out.i32(ARGB.linearToSrgbChannel(ch / 255.0F))));
		}

		// THE TABLES THEMSELVES, all 1024 entries of each, read by reflection.
		//
		// Why reflection: both fields are `private static final byte[]`, and
		// `linearToSrgbChannel` cannot recover index j -- it takes a float and loses the index
		// to `floor`. So there is no public accessor that enumerates the table, and
		// `meanLinear`, `linearChannelMean` and `linearLerp` all index it at up to 1023.
		//
		// Why they must be EMBEDDED in Rust rather than recomputed: the initialisers call
		// `Math.pow`, which is a HotSpot intrinsic (measured: 51,268 of 145 million sweep values
		// differ from FdLibm). A pure-Rust recomputation would be a THIRD unported intrinsic, and
		// would be wrong in a way no test could distinguish from "the table is fine". So this
		// follows the established `embedded-trig-tables` pattern from `Mth`, whose 65,536-entry
		// SIN/COS tables are embedded for exactly the same reason.
		// NOTE: no `o.fn` header here, deliberately. The two table groups below each get their own
		// header immediately followed by their own loop, per rule 2 at the top of this file. An
		// earlier version of this edit left an `argb.srgbTableBytes` header with no loop writing
		// to it, and the golden file duly contained a group header followed immediately by the
		// next one -- an empty group, which is indistinguishable from a passing one until a test
		// claims it. `parity_batch2.rs` does fail on empty groups, which is how it got caught,
		// but the fix belongs here rather than in the test.
		//
		// The two tables are DIFFERENT TYPES AND DIFFERENT LENGTHS, which the decompiled source
		// makes easy to get wrong because they sit next to each other and look symmetric:
		//
		//   SRGB_TO_LINEAR   short[256]  values 0..1023 stored in a short. Indexed by an sRGB
		//                                 CHANNEL (0..255), and divided by 1023.0F on read.
		//   LINEAR_TO_SRGB   byte[1024]  values 0..255 in a SIGNED byte, which is why every
		//                                 read is `LINEAR_TO_SRGB[...] & 0xFF`. Indexed by
		//                                 `Mth.floor(linear * 1023.0F)`, hence 1024 entries.
		//
		// Types confirmed with `javap -p -cp <jar> net.minecraft.util.ARGB`:
		//     private static final short[] SRGB_TO_LINEAR;
		//     private static final byte[]  LINEAR_TO_SRGB;
		//
		// I guessed byte[] and then char[] and got a ClassCastException both times, which
		// `section()` SWALLOWED and continued past -- so `argb.setBrightness` silently stopped
		// being emitted and the golden file SHRANK by 6 KB. No test failed; the only evidence
		// was a file getting smaller. ALWAYS read section-error.txt after an oracle run.
		short[] srgbToLinear = (short[]) readPrivateStatic(ARGB.class, "SRGB_TO_LINEAR");
		byte[] linearToSrgb = (byte[]) readPrivateStatic(ARGB.class, "LINEAR_TO_SRGB");
		if (srgbToLinear.length != 256 || linearToSrgb.length != 1024) {
			throw new IllegalStateException("expected SRGB_TO_LINEAR[256] and LINEAR_TO_SRGB[1024], got "
				+ srgbToLinear.length + " and " + linearToSrgb.length
				+ " -- both lengths are load-bearing and a different size means the jar changed");
		}
		// Two loops, two headers, because rule 2 at the top of this file: a header is followed
		// immediately by the only loop that writes to it. One loop writing both would put every
		// row under the LAST header and leave the other group empty -- and an empty group looks
		// exactly like a passing one.
		o.fn("argb.srgbToLinearTable", "", "i32");
		for (int i = 0; i < 256; i++) {
			// short is signed, but every value here is 0..1023 and therefore non-negative, so no
			// mask is needed. Asserted below rather than assumed.
			if (srgbToLinear[i] < 0) {
				throw new IllegalStateException("SRGB_TO_LINEAR[" + i + "] = " + srgbToLinear[i]
					+ " is negative; the 0..1023 assumption this emitter relies on is wrong");
			}
			o.row(Out.i32(i), Out.i32(srgbToLinear[i]));
		}
		o.fn("argb.linearToSrgbTable", "", "i32");
		for (int i = 0; i < 1024; i++) {
			// `& 0xFF`: Java's byte is signed and these values go above 127, so the mask is
			// what makes the emitted number the unsigned one the Rust side will store.
			o.row(Out.i32(i), Out.i32(linearToSrgb[i] & 0xFF));
		}

		// setBrightness: pure arithmetic plus Math.round, so it SHOULD be exact. The six
		// hue sectors are separate branches, hence the wide colour corpus.
		o.fn("argb.setBrightness", "i32 f32", "i32");
		for (int c : cs) {
			for (float b : new float[] {0.0F, 0.25F, 0.5F, 0.75F, 1.0F, 2.0F, -1.0F, Float.NaN}) {
				o.row(Out.join(Out.i32(c), Out.f32(b)), Out.i32(ARGB.setBrightness(c, b)));
			}
		}
	}

	// =========================================================================
	// Identifier
	// =========================================================================
	static void identifier(Out o) {
		// The corpus mixes valid, invalid, and the cases the implementation actually
		// branches on: leading ':' (default namespace), no ':' at all, '..' as a
		// namespace (explicitly rejected), and characters legal in a PATH but not a
		// NAMESPACE.
		String[] cases = {
			"minecraft:stone", "stone", ":stone", "minecraft:", ":", "::",
			"a:b:c", "a:", ":a", "..:x", "..", "a/../b", "a/./b", "./a", "a//b",
			"minecraft:stone/cobblestone", "MODNAMESPACE:path", "a.b-c_d:e",
			"a:b/", "a:/b", "a:b.", "a:..", "a:../..", "namespace:path with space",
			"", " ", "\t", "\n", "a:b\n", "Minecraft:Stone", "a:B", "a:b:c:d",
			"minecraft:foo/bar/baz", "foo:bar", "mymod:block", "mymod:",
			"a:0", "0:a", "-:a", "a:-", "_:_", ".:.", "..:..",
		};
		o.fn("identifier.parse", "str", "bool str str i32 i32 i32");
		for (String s : cases) {
			try {
				Identifier id = Identifier.parse(s);
				o.row(Out.str(s), Out.join(Out.b(true), Out.str(id.getNamespace()), Out.str(id.getPath()),
					Out.i32(id.hashCode()),
					Out.i32(Integer.signum(id.compareTo(Identifier.withDefaultNamespace(s)))),
					Out.i32(id.toString().length())));
			} catch (IdentifierException ex) {
				o.row(Out.str(s), Out.join(Out.b(false), Out.str(""), Out.str(""),
					Out.i32(0), Out.i32(0), Out.i32(0)));
			}
		}

		o.fn("identifier.parseError", "str", "str");
		for (String s : cases) {
			try {
				Identifier.parse(s);
			} catch (IdentifierException ex) {
				o.row(Out.str(s), Out.str(ex.getClass().getName() + ": " + ex.getMessage()));
			}
		}

		o.fn("identifier.tryParse", "str", "bool str str");
		for (String s : cases) {
			Identifier id = Identifier.tryParse(s);
			o.row(Out.str(s), id == null
				? Out.join(Out.b(false), Out.str(""), Out.str(""))
				: Out.join(Out.b(true), Out.str(id.getNamespace()), Out.str(id.getPath())));
		}

		String[] namespaces = {"minecraft", "a", "..", "", "A", "a.b", "a-b", "a_b", "a/b", "a b"};
		String[] paths = {"stone", "a/b", "", "A", "a.b-c_d", "a b", "a:b", "..", "0"};

		o.fn("identifier.fromNamespaceAndPath", "str str", "bool str str str");
		for (String ns : namespaces) {
			for (String p : paths) {
				String two = Out.join(Out.str(ns), Out.str(p));
				try {
					Identifier id = Identifier.fromNamespaceAndPath(ns, p);
					o.row(two, Out.join(Out.b(true), Out.str(id.getNamespace()),
						Out.str(id.getPath()), Out.str(id.toString())));
				} catch (IdentifierException ex) {
					o.row(two, Out.join(Out.b(false), Out.str(""), Out.str(""), Out.str("")));
				}
			}
		}

		o.fn("identifier.fromNamespaceAndPathError", "str str", "str");
		for (String ns : namespaces) {
			for (String p : paths) {
				try {
					Identifier.fromNamespaceAndPath(ns, p);
				} catch (IdentifierException ex) {
					o.row(Out.join(Out.str(ns), Out.str(p)),
						Out.str(ex.getClass().getName() + ": " + ex.getMessage()));
				}
			}
		}

		o.fn("identifier.withDefaultNamespace", "str", "bool str str str");
		for (String p : cases) {
			try {
				Identifier id = Identifier.withDefaultNamespace(p);
				o.row(Out.str(p), Out.join(Out.b(true), Out.str(id.getNamespace()),
					Out.str(id.getPath()), Out.str(id.toString())));
			} catch (IdentifierException ex) {
				o.row(Out.str(p), Out.join(Out.b(false), Out.str(""), Out.str(""), Out.str("")));
			}
		}

		o.fn("identifier.withDefaultNamespaceError", "str", "str");
		for (String p : cases) {
			try {
				Identifier.withDefaultNamespace(p);
			} catch (IdentifierException ex) {
				o.row(Out.str(p), Out.str(ex.getClass().getName() + ": " + ex.getMessage()));
			}
		}

		o.fn("identifier.tryBuild", "str str", "bool str str");
		for (String ns : namespaces) {
			for (String p : paths) {
				Identifier id = Identifier.tryBuild(ns, p);
				o.row(Out.join(Out.str(ns), Out.str(p)), id == null
					? Out.join(Out.b(false), Out.str(""), Out.str(""))
					: Out.join(Out.b(true), Out.str(id.getNamespace()), Out.str(id.getPath())));
			}
		}

		// bySeparator / tryBySeparator with a non-':' separator, which vanilla uses for
		// things like NBT compound keys.
		o.fn("identifier.bySeparator", "str str", "bool str str str");
		for (String s : new String[] {"a.b.c", "a.b", ".a", "a.", "a", "", "a..b", "..", "a.b/c"}) {
			for (String sep : new String[] {".", "/", ":"}) {
				try {
					Identifier id = Identifier.bySeparator(s, sep.charAt(0));
					o.row(Out.join(Out.str(s), Out.str(sep)),
						Out.join(Out.b(true), Out.str(id.getNamespace()),
							Out.str(id.getPath()), Out.str(id.toString())));
				} catch (IdentifierException ex) {
					o.row(Out.join(Out.str(s), Out.str(sep)),
						Out.join(Out.b(false), Out.str(""), Out.str(""), Out.str("")));
				}
			}
		}

		o.fn("identifier.bySeparatorError", "str str", "str");
		// These are chosen to actually THROW. `bySeparator` only raises when the text
		// BEFORE the first separator is an invalid NAMESPACE, and a namespace rejects
		// uppercase, spaces and '/'. So "A.b/c" with '/' yields namespace "A", which is
		// invalid, and "a b/c" yields "a b". (My first corpus here was all-lowercase and
		// well-formed, so the group came out EMPTY -- which is indistinguishable from a
		// group that was never claimed by a test.)
		for (String s : new String[] {"A.b/c", "a b/c", "a:B/c", "A:B", "a b:B", "..:x", "A b/c:d"}) {
			for (String sep : new String[] {".", "/", ":"}) {
				try {
					Identifier.bySeparator(s, sep.charAt(0));
				} catch (IdentifierException ex) {
					o.row(Out.join(Out.str(s), Out.str(sep)),
						Out.str(ex.getClass().getName() + ": " + ex.getMessage()));
				}
			}
		}

		o.fn("identifier.tryBySeparator", "str str", "bool str str");
		for (String s : new String[] {"a.b.c", "a.b", ".a", "a.", "a", "", "a..b", "..", "a.b/c"}) {
			for (String sep : new String[] {".", "/", ":"}) {
				Identifier id = Identifier.tryBySeparator(s, sep.charAt(0));
				o.row(Out.join(Out.str(s), Out.str(sep)), id == null
					? Out.join(Out.b(false), Out.str(""), Out.str(""))
					: Out.join(Out.b(true), Out.str(id.getNamespace()), Out.str(id.getPath())));
			}
		}

		// Predicates, character by character over the WHOLE range: these are the
		// gatekeepers, and an off-by-one on the range boundary would silently admit a
		// character vanilla rejects.
		o.fn("identifier.isAllowedInIdentifier", "i32", "bool");
		for (int c = 0; c < 128; c++) {
			o.row(Out.i32(c), Out.b(Identifier.isAllowedInIdentifier((char) c)));
		}
		for (int c : new int[] {0x80, 0xFF, 0x100, 0x2603, 0xFFFF}) {
			o.row(Out.i32(c), Out.b(Identifier.isAllowedInIdentifier((char) c)));
		}

		o.fn("identifier.validPathChar", "i32", "bool");
		for (int c = 0; c < 128; c++) {
			o.row(Out.i32(c), Out.b(Identifier.validPathChar((char) c)));
		}
		for (int c : new int[] {0x80, 0xFF, 0x2603}) {
			o.row(Out.i32(c), Out.b(Identifier.validPathChar((char) c)));
		}

		o.fn("identifier.isValidPath", "str", "bool");
		for (String s : new String[] {"", "a", "a/b", "a..b", "..", "A", "a b", "a:b",
				"a-b_c.d", "/", "//", "."}) {
			o.row(Out.str(s), Out.b(Identifier.isValidPath(s)));
		}

		o.fn("identifier.isValidNamespace", "str", "bool");
		for (String s : new String[] {"", "a", "a/b", "a..b", "..", "A", "a b", "a:b",
				"a-b_c.d", "/", "//", "."}) {
			o.row(Out.str(s), Out.b(Identifier.isValidNamespace(s)));
		}

		o.fn("identifier.strings", "str str", "str str str str str str i32 i32");
		for (String ns : new String[] {"minecraft", "mymod"}) {
			for (String p : new String[] {"stone", "a/b/c", "a.b-c_d"}) {
				Identifier id = Identifier.fromNamespaceAndPath(ns, p);
				o.row(Out.join(Out.str(ns), Out.str(p)),
					Out.join(Out.str(id.toString()), Out.str(id.toDebugFileName()),
						Out.str(id.toLanguageKey()), Out.str(id.toShortLanguageKey()),
						Out.str(id.toShortString()), Out.str(id.toLanguageKey("pre")),
						Out.i32(Integer.signum(id.compareTo(Identifier.fromNamespaceAndPath("minecraft", p)))),
						Out.i32(id.hashCode())));
			}
		}

		o.fn("identifier.withPath", "str str str", "str");
		for (String ns : new String[] {"minecraft", "mymod"}) {
			for (String p : new String[] {"stone", "a/b/c"}) {
				for (String mod : new String[] {"x", "", "a/b", "up_", "A", "a:b", "."}) {
					try {
						o.row(Out.join(Out.str(ns), Out.str(p), Out.str(mod)),
							Out.str(Identifier.fromNamespaceAndPath(ns, p).withPath(mod).toString()));
					} catch (IdentifierException ex) {
						// reported in identifier.withPathError
					}
				}
			}
		}

		o.fn("identifier.withPathError", "str str str", "str");
		for (String ns : new String[] {"minecraft", "mymod"}) {
			for (String p : new String[] {"stone", "a/b/c"}) {
				for (String mod : new String[] {"x", "", "a/b", "up_", "A", "a:b", "."}) {
					try {
						Identifier.fromNamespaceAndPath(ns, p).withPath(mod);
					} catch (IdentifierException ex) {
						o.row(Out.join(Out.str(ns), Out.str(p), Out.str(mod)),
							Out.str(ex.getClass().getName() + ": " + ex.getMessage()));
					}
				}
			}
		}

		o.fn("identifier.withPrefixSuffix", "str str", "str str str");
		for (String ns : new String[] {"minecraft", "mymod"}) {
			for (String p : new String[] {"stone", "a/b/c"}) {
				Identifier id = Identifier.fromNamespaceAndPath(ns, p);
				o.row(Out.join(Out.str(ns), Out.str(p)),
					Out.join(Out.str(id.withPrefix("pre_").toString()),
						Out.str(id.withSuffix("_suf").toString()),
						Out.str(id.toLanguageKey("pre", "suf"))));
			}
		}

		o.fn("identifier.compareTo", "str str str str", "i32 i32 i32");
		// The four columns are (ns, path, ns, path) -- NOT full "ns:path" strings. Feeding
		// "minecraft:a" as a namespace throws, because ':' is legal in a PATH but not in a
		// NAMESPACE. (I made that mistake first; it is why this corpus is written this
		// way and not as whole identifiers.)
		for (String[] pair : new String[][] {
			{"minecraft", "a", "minecraft", "b"},
			{"minecraft", "a", "mymod", "a"},
			{"minecraft", "a/b", "minecraft", "a"},
			{"a", "b", "b", "a"},
			{"minecraft", "a", "minecraft", "a"},
			{"minecraft", "a/b", "minecraft", "a/a"},
		}) {
			Identifier a = Identifier.fromNamespaceAndPath(pair[0], pair[1]);
			Identifier b = Identifier.fromNamespaceAndPath(pair[2], pair[3]);
			o.row(Out.join(Out.str(pair[0]), Out.str(pair[1]), Out.str(pair[2]), Out.str(pair[3])),
				Out.join(Out.i32(Integer.signum(a.compareTo(b))),
					Out.i32(Integer.signum(b.compareTo(a))),
					Out.i32(a.equals(b) ? 1 : 0)));
		}

		o.fn("identifier.constants", "", "str str str str str i32 i32");
		o.row(Out.join(
				Out.str(String.valueOf(Identifier.NAMESPACE_SEPARATOR)),
				Out.str(Identifier.DEFAULT_NAMESPACE), Out.str(Identifier.REALMS_NAMESPACE),
				Out.str(Identifier.ALLOWED_NAMESPACE_CHARACTERS),
				Out.str(Identifier.ERROR_INVALID.getClass().getName()),
				Out.i32(Identifier.ERROR_INVALID.hashCode()),
				Out.i32(Identifier.ERROR_INVALID.toString().length())),
			Out.str("constants"));
	}

	// =========================================================================
	// Rotations
	// =========================================================================
	static void rotations(Out o) {
		// The canonicalising constructor. `x % 360.0F` is the interesting part: the
		// literal is a FLOAT, so the remainder is computed in single precision, and
		// infinities and NaN are special-cased to 0. The float corpus contains both.
		o.fn("rotations.constructor", "f32 f32 f32", "f32 f32 f32");
		for (float x : V2.FLOATS) {
			for (float y : V2.FLOATS) {
				for (float z : V2.FLOATS) {
					Rotations r = new Rotations(x, y, z);
					o.row(Out.join(Out.f32(x), Out.f32(y), Out.f32(z)),
						Out.join(Out.f32(r.x()), Out.f32(r.y()), Out.f32(r.z())));
				}
			}
		}

		o.fn("rotations.hashCode", "f32 f32 f32", "i32");
		for (float x : V2.FLOATS) {
			for (float y : V2.FLOATS2) {
				for (float z : V2.FLOATS2) {
					o.row(Out.join(Out.f32(x), Out.f32(y), Out.f32(z)),
						Out.i32(new Rotations(x, y, z).hashCode()));
				}
			}
		}

		o.fn("rotations.equals", "f32 f32 f32 f32 f32 f32", "bool");
		for (float x : V2.FLOATS2) {
			for (float y : V2.FLOATS2) {
				for (float z : V2.FLOATS2) {
					Rotations r = new Rotations(x, y, z);
					o.row(Out.join(Out.f32(x), Out.f32(y), Out.f32(z), Out.f32(0.0F), Out.f32(0.0F), Out.f32(0.0F)),
						Out.b(r.equals(new Rotations(0.0F, 0.0F, 0.0F))));
					o.row(Out.join(Out.f32(x), Out.f32(y), Out.f32(z),
							Out.f32(x + 1.0F), Out.f32(y), Out.f32(z)),
						Out.b(r.equals(new Rotations(x + 1.0F, y, z))));
				}
			}
		}

		o.fn("rotations.toString", "f32 f32 f32", "str");
		for (float x : V2.FLOATS) {
			o.row(Out.join(Out.f32(x), Out.f32(0.0F), Out.f32(x)), Out.str(new Rotations(x, 0.0F, x).toString()));
		}
	}

	// =========================================================================
	// Direction.Plane
	// =========================================================================
	static void plane(Out o) {
		o.fn("plane.constants", "", "i32 i32 i32 i32 i32");
		o.row(Out.join(
				Out.i32(Direction.Plane.values().length),
				Out.i32(Direction.Plane.HORIZONTAL.ordinal()),
				Out.i32(Direction.Plane.VERTICAL.ordinal()),
				Out.i32(Direction.Plane.HORIZONTAL.length()),
				Out.i32(Direction.Plane.VERTICAL.length())),
			Out.str("plane constants"));

		// Iteration order is the DECLARATION order of `faces`, not a sorted order, and
		// callers that iterate a plane depend on it.
		o.fn("plane.iterate", "i32", "i32 i32 i32 i32 i32");
		for (int p = 0; p < V2.PLANES.length; p++) {
			List<Direction> faces = new ArrayList<>();
			V2.PLANES[p].forEach(faces::add);
			// HORIZONTAL has four faces, VERTICAL only two, so every slot is bounds-checked.
			// -1 means "this plane has no such face", which is itself the interesting fact.
			o.row(Out.i32(p),
				Out.join(Out.i32(faces.size()), Out.i32(ordAt(faces, 0)), Out.i32(ordAt(faces, 1)),
					Out.i32(ordAt(faces, 2)), Out.i32(ordAt(faces, 3))));
		}

		o.fn("plane.test", "i32 i32", "bool i32 i32");
		for (int p = 0; p < V2.PLANES.length; p++) {
			for (int d = -1; d < V2.DIRECTIONS.length; d++) {
				Direction dir = d < 0 ? null : V2.DIRECTIONS[d];
				o.row(Out.join(Out.i32(p), Out.i32(d)),
					Out.join(Out.b(V2.PLANES[p].test(dir)),
						Out.i32(dir == null ? -1 : dir.getAxis().ordinal()),
						Out.i32(dir == null ? -1 : (dir.getAxis().getPlane() == V2.PLANES[p] ? 1 : 0))));
			}
		}

		o.fn("plane.getRandomDirection", "i64 i32 i32", "i32 i32 i32 i32");
		for (long seed : V2.SEEDS) {
			for (int p = 0; p < V2.PLANES.length; p++) {
				for (int i = 0; i < 8; i++) {
					RandomSource r = RandomSource.create(seed);
					Direction d = V2.PLANES[p].getRandomDirection(r);
					o.row(Out.join(Out.i64(seed), Out.i32(p), Out.i32(i)),
						Out.join(Out.i32(d.ordinal()), Out.i32(d.getStepX()),
							Out.i32(d.getStepY()), Out.i32(d.getStepZ())));
				}
			}
		}

		o.fn("plane.getRandomAxis", "i64 i32 i32", "i32");
		for (long seed : V2.SEEDS) {
			for (int p = 0; p < V2.PLANES.length; p++) {
				for (int i = 0; i < 8; i++) {
					RandomSource r = RandomSource.create(seed);
					o.row(Out.join(Out.i64(seed), Out.i32(p), Out.i32(i)),
						Out.i32(V2.PLANES[p].getRandomAxis(r).ordinal()));
				}
			}
		}

		o.fn("plane.shuffledCopy", "i64 i32", "i32 i32 i32 i32");
		for (long seed : V2.SEEDS) {
			for (int p = 0; p < V2.PLANES.length; p++) {
				RandomSource r = RandomSource.create(seed);
				List<Direction> sh = V2.PLANES[p].shuffledCopy(r);
				// VERTICAL has only two faces, so index 2 does not exist. The size column
				// records that, and the third ordinal is -1 rather than an exception.
				o.row(Out.join(Out.i64(seed), Out.i32(p)),
					Out.join(Out.i32(sh.size()), Out.i32(ordAt(sh, 0)), Out.i32(ordAt(sh, 1)),
						Out.i32(ordAt(sh, 2))));
			}
		}
	}

	// =========================================================================
	// The Mth methods that were blocked on unported types
	// =========================================================================
	static void mthBlocked(Out o) {
		// `getSeed` and `lerp` live in MthOracle (and mth.txt), under their REAL 26.2 names.
		// They were first emitted here as `mth.getSeed` / `mth.lerp`, duplicating those
		// groups; one copy is enough. See OPEN_QUESTIONS #19.

		o.fn("mth.mulAndTruncate", "i32 i32 i32", "i32");
		int[][] fracs = {{1, 2}, {1, 3}, {2, 3}, {3, 4}, {-1, 7}, {7, -1}, {1, 1},
			{Integer.MAX_VALUE, 1}, {1, Integer.MAX_VALUE}, {0, 5}};
		for (int[] f : fracs) {
			for (int k : SCALES) {
				o.row(Out.join(Out.i32(f[0]), Out.i32(f[1]), Out.i32(k)),
					Out.i32(Mth.mulAndTruncate(Fraction.getFraction(f[0], f[1]), k)));
			}
		}

		o.fn("mth.rayIntersectsAABB", "f64 f64 f64 f64 f64 f64", "bool");
		double[] coords = {0.0, -1.0, 1.0, 0.5, 2.0, -0.5, 1.0e9, Double.NaN};
		for (double a : coords) {
			for (double b : coords) {
				for (double c : coords) {
					AABB box = new AABB(a, b, c, c, a, b);
					o.row(Out.join(Out.f64(a), Out.f64(b), Out.f64(c),
							Out.f64(box.maxX), Out.f64(box.maxY), Out.f64(box.maxZ)),
						Out.b(Mth.rayIntersectsAABB(new Vec3(c, a, b), new Vec3(a, c, b), box)));
				}
			}
		}

		// Goes through JOML's Quaternionf, so this exercises the FMA-based port.
		o.fn("mth.rotationAroundAxis", "f32 f32 f32 f32 f32 f32 f32 f32 f32 f32", "f32 f32 f32 f32");
		for (float ax : new float[] {0.0F, 1.0F, -1.0F, 0.5F, 0.0F, 1.0F, -1.0F, 0.0F}) {
			for (float ay : new float[] {0.0F, 1.0F, -1.0F}) {
				Vector3f axis = new Vector3f(ax, ay, 0.5F);
				for (float[] q : new float[][] {
					{0.0F, 0.0F, 0.0F, 1.0F},
					{0.0F, 0.0F, 0.0F, -1.0F},
					{0.5F, 0.5F, 0.5F, 0.5F},
					{0.0F, 0.0F, 1.0F, 0.0F},
				}) {
					Quaternionf in = new Quaternionf(q[0], q[1], q[2], q[3]);
					Quaternionf res = Mth.rotationAroundAxis(axis, in, new Quaternionf());
					o.row(Out.join(Out.f32(ax), Out.f32(ay), Out.f32(0.5F),
							Out.f32(q[0]), Out.f32(q[1]), Out.f32(q[2]), Out.f32(q[3]),
							Out.f32(0.0F), Out.f32(0.0F), Out.f32(0.0F)),
						Out.join(Out.f32(res.x()), Out.f32(res.y()), Out.f32(res.z()), Out.f32(res.w())));
				}
			}
		}
	}

	// -------------------------------------------------------------------------
	// helpers
	// -------------------------------------------------------------------------

	/** Read a private static final int off a game class, so the value comes from the jar. */
	private static int priv(Class<?> cls, String name) {
		try {
			java.lang.reflect.Field f = cls.getDeclaredField(name);
			f.setAccessible(true);
			return f.getInt(null);
		} catch (ReflectiveOperationException e) {
			throw new IllegalStateException(cls.getName() + "." + name, e);
		}
	}

	/**
	 * Ordinal of element `i`, or -1 when the list is shorter.
	 *
	 * `Direction.Plane.VERTICAL` has only two faces while `HORIZONTAL` has four, so any
	 * fixed-width row over a plane has to tolerate a short list. Getting this wrong is
	 * not a typo -- it kills the whole Direction.Plane section, which is exactly what
	 * happened the first two times.
	 */
	private static int ordAt(final List<Direction> list, final int i) {
		return i < list.size() ? list.get(i).ordinal() : -1;
	}

	private static String out3s(SectionPos p) {
		return Out.join(Out.i32(p.x()), Out.i32(p.y()), Out.i32(p.z()));
	}

	private static String out3i(BlockPos p) {
		return Out.join(Out.i32(p.getX()), Out.i32(p.getY()), Out.i32(p.getZ()));
	}

	private static String out3(Vec3 v) {
		return Out.join(Out.f64(v.x), Out.f64(v.y), Out.f64(v.z));
	}

	private static String out2i(ChunkPos p) {
		return Out.join(Out.i32(p.x()), Out.i32(p.z()));
	}

	private static String out6(AABB b) {
		return Out.join(Out.f64(b.minX), Out.f64(b.minY), Out.f64(b.minZ),
			Out.f64(b.maxX), Out.f64(b.maxY), Out.f64(b.maxZ));
	}

	/** Emit the first `cap` elements of an iterator as rows. */
	private static void emitSeq(Out o, int[] from, int[] to,
			BiFunction<BlockPos, BlockPos, Iterable<BlockPos>> fn, int cap) {
		List<BlockPos> list = new ArrayList<>();
		fn.apply(new BlockPos(from[0], from[1], from[2]), new BlockPos(to[0], to[1], to[2])).forEach(list::add);
		String args = Out.join(Out.i32(from[0]), Out.i32(from[1]), Out.i32(from[2]),
			Out.i32(to[0]), Out.i32(to[1]), Out.i32(to[2]));
		for (int i = 0; i < Math.min(list.size(), cap); i++) {
			o.row(args, out3i(list.get(i)));
		}
	}

	/**
	 * Run vanilla's own bootstrap.
	 *
	 * `ChunkPos#isValid` reads `ChunkPyramid.MAX_CHUNK_COORDINATE_VALUE`, computed from
	 * the built-in registry. Without this, class-init throws `IllegalArgumentException:
	 * Not bootstrapped (called from registry minecraft:game_event)` -- which surfaces as
	 * an ExceptionInInitializerError on whatever class happened to touch it first, a
	 * genuinely misleading error message.
	 */
	private static void bootstrap() {
		try {
			net.minecraft.SharedConstants.tryDetectVersion();
			net.minecraft.server.Bootstrap.bootStrap();
		} catch (RuntimeException | Error e) {
			// Already bootstrapped, or nothing needed it. The groups that genuinely
			// need the registry fail loudly on their own.
			System.out.println("bootstrap: " + e);
		}
	}

	/**
	 * Each section runs through here so ONE throwing class cannot cost the other eight.
	 *
	 * Not hypothetical: `ARGB.linearLerp` with a negative alpha really does throw
	 * `ArrayIndexOutOfBoundsException` in vanilla, and before this wrapper a single bad
	 * group truncated the whole 34 MB file after 194 of 234 groups -- with nothing at
	 * all indicating what had been lost. A golden file that silently loses its tail is
	 * worse than one that refuses to be written.
	 */
	private static void section(final Out o, final String name,
			final java.util.function.Consumer<Out> body) {
		try {
			body.accept(o);
		} catch (Throwable t) {
			System.out.println("SECTION FAILED (continuing): " + name + ": " + t);
			try (java.io.PrintWriter w = new java.io.PrintWriter(
					new java.io.FileWriter("section-error.txt", true))) {
				w.println("SECTION FAILED: " + name + ": " + t);
				t.printStackTrace(w);
			} catch (java.io.IOException ignored) {
				// Nowhere left to report it; the stdout line above is the record.
			}
		}
	}

	static void emit(final java.nio.file.Path out) throws Exception {
		final Out o = new Out(out);
		bootstrap();
		o.comment("batch 2 -- generated from the Minecraft 26.2 jar, not from decompiled source");
		section(o, "BlockPos", Batch2Oracle::blockPos);
		section(o, "ChunkPos", Batch2Oracle::chunkPos);
		section(o, "SectionPos", Batch2Oracle::sectionPos);
		section(o, "Vec3", Batch2Oracle::vec3);
		section(o, "Vec2", Batch2Oracle::vec2);
		section(o, "AABB", Batch2Oracle::aabb);
		section(o, "ARGB", Batch2Oracle::argb);
		section(o, "Identifier", Batch2Oracle::identifier);
		section(o, "Rotations", Batch2Oracle::rotations);
		section(o, "Direction.Plane", Batch2Oracle::plane);
		section(o, "Mth-blocked", Batch2Oracle::mthBlocked);
		o.write(out);
	}
}
