// ============================================================================
// ORACLE ENTRY POINT -- not game code.
//
// Generates golden data by running Minecraft 26.2's own classes.
//
// Build & run:  _porting/java-oracle/run.ps1
//
// ---------------------------------------------------------------------------
// THE REAL JAR IS GROUND TRUTH
// ---------------------------------------------------------------------------
// In Jar mode (the default, and the only mode that writes the committed golden files)
// NOTHING from minecraft-decompiled/ is compiled. Every game class comes from
// minecraft-merged-deobf-26.2.jar -- Mojang's own bytecode.
//
// That is deliberate. Session 03 recompiled the decompiled sources and treated them as
// authoritative, which was wrong in principle: a decompiler can emit source that
// compiles to different behaviour, or (as with Util.java under JDK 25) does not compile
// at all. The thing we are porting is the game's behaviour, and the jar is where that
// behaviour actually lives.
//
// In Source mode the decompiled sources are compiled and put first on the classpath,
// purely so the two can be diffed. Any row that differs is a DECOMPILER ARTIFACT and is
// recorded in _porting/DESIGN_DECISIONS.md under #decompiler-artifacts.
//
// `--expect-origin` enforces whichever mode is active: the run FAILS if any tracked
// class did not load from the expected artifact, so a stale classes/ entry can never
// silently shadow the jar.
// ============================================================================
package oracle;

import java.net.URI;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

public final class Oracle {
	/**
	 * Every class this oracle is supposed to be testing.
	 *
	 * Each MUST load from the artifact `--expect-origin` names. In Jar mode that is the
	 * Minecraft jar; if a class were to come from `classes/` instead, we would be
	 * measuring a recompilation while believing we were measuring vanilla.
	 */
	private static final String[] TRACKED = {
		// --- session 02: util + RNG ---
		"net.minecraft.util.Mth",
		"net.minecraft.util.RandomSource",
		"net.minecraft.util.LinearCongruentialGenerator",
		"net.minecraft.world.level.levelgen.LegacyRandomSource",
		"net.minecraft.world.level.levelgen.XoroshiroRandomSource",
		"net.minecraft.world.level.levelgen.Xoroshiro128PlusPlus",
		"net.minecraft.world.level.levelgen.RandomSupport",
		"net.minecraft.world.level.levelgen.BitRandomSource",
		"net.minecraft.world.level.levelgen.PositionalRandomFactory",
		"net.minecraft.world.level.levelgen.MarsagliaPolarGaussian",
		"net.minecraft.world.level.levelgen.SingleThreadedRandomSource",
		"net.minecraft.world.level.levelgen.ThreadSafeLegacyRandomSource",
		"net.minecraft.world.level.levelgen.WorldgenRandom",
		// --- session 03/04: core value types ---
		"net.minecraft.core.Vec3i",
		"net.minecraft.core.BlockPos",
		"net.minecraft.core.BlockPos$MutableBlockPos",
		"net.minecraft.core.Direction",
		"net.minecraft.core.Direction$Axis",
		"net.minecraft.core.Direction$AxisDirection",
		"net.minecraft.core.Direction$Plane",
		"net.minecraft.world.level.ChunkPos",
		"net.minecraft.core.SectionPos",
		"net.minecraft.world.phys.Vec3",
		"net.minecraft.world.phys.AABB",
		"net.minecraft.util.ARGB",
		"net.minecraft.core.Rotations",
		"net.minecraft.resources.Identifier",
		// --- session 05: batch 2 additions ---
		"net.minecraft.world.phys.Vec2",
	};

	public static void main(final String[] args) throws Exception {
		Path testData = Path.of(args.length > 0 ? args[0] : "../test-data");
		boolean noWrite = List.of(args).contains("--no-write");
		// Optional stage filter: `oracle.Oracle <outDir> --only batch2` runs just that emitter.
		// Useful for isolating a crash without paying for the other four.
		//
		// It MUST be an explicit `--only <name>` flag. I first wrote this as "the first
		// argument that does not start with --", which silently matched the JAR PATH in
		// `--expect-origin <jar>`: every stage was skipped, the run printed "done.", and
		// the golden files were left untouched at their previous contents. A harness that
		// reports success while doing nothing is the worst kind of harness bug, and it is
		// invisible unless you check the OUTPUT FILES rather than the exit code.
		String only = null;
		for (int i = 0; i < args.length - 1; i++) {
			if (args[i].equals("--only")) {
				only = args[i + 1];
			}
		}
		Path expectFrom = null;
		for (int i = 0; i < args.length - 1; i++) {
			if (args[i].equals("--expect-origin")) {
				expectFrom = Path.of(args[i + 1]).toAbsolutePath().normalize();
			}
		}
		System.out.println("oracle -> " + testData.toAbsolutePath());

		List<String> wrongOrigin = new ArrayList<>();
		for (String name : TRACKED) {
			Class<?> c;
			try {
				c = Class.forName(name);
			} catch (ClassNotFoundException e) {
				wrongOrigin.add(name + " <- NOT FOUND on the classpath");
				continue;
			}
			String where = String.valueOf(c.getProtectionDomain().getCodeSource().getLocation());
			if (expectFrom != null) {
				try {
					Path actual = Path.of(new URI(where)).toAbsolutePath().normalize();
					if (!actual.equals(expectFrom)) {
						wrongOrigin.add(name + " <- " + actual + " (expected " + expectFrom + ")");
					}
				} catch (Exception e) {
					wrongOrigin.add(name + " <- " + where + " (unparseable)");
				}
			}
		}

		if (!wrongOrigin.isEmpty()) {
			System.err.println("FAIL: these classes did NOT load from " + expectFrom + ":");
			wrongOrigin.forEach(s -> System.err.println("    " + s));
			System.err.println("The golden data below would describe the wrong artifact.");
			System.exit(2);
		}
		if (expectFrom != null) {
			System.out.println("  all " + TRACKED.length + " tracked classes came from " + expectFrom);
		}

		if (noWrite) {
			System.out.println("--no-write: origin check only, no golden data emitted.");
			return;
		}

		// Each emitter is wrapped so a failure names WHICH one died, and prints a real
		// stack trace.
		//
		// Without this, a failure inside an emitter surfaced only as
		// `Exception in thread "main"` with a truncated toString: no type, no stack, no
		// hint which of five emitters was responsible. That happened twice while
		// building the batch-2 oracle. Batch 2 in particular reaches `Bootstrap` and
		// `commons-lang3` initialisers, whose failures are not self-explanatory, so
		// "which stage" is the first thing you need to know.
		if (only == null || only.equals("mth")) {
			stage("mth", () -> MthOracle.emit(testData.resolve("mth.txt")));
		}
		if (only == null || only.equals("mth_tables")) {
			stage("mth_tables", () -> MthOracle.emitTables(testData.resolve("mth_tables.txt")));
		}
		if (only == null || only.equals("random")) {
			stage("random", () -> RandomOracle.emit(testData.resolve("random.txt")));
		}
		if (only == null || only.equals("core")) {
			stage("core", () -> CoreOracle.emit(testData.resolve("core.txt")));
		}
		if (only == null || only.equals("jvm_math")) {
			stage("jvm_math", () -> JvmMathOracle.emitTo(testData.resolve("jvm_math.txt")));
		}
		if (only == null || only.equals("batch2")) {
			stage("batch2", () -> Batch2Oracle.emit(testData.resolve("batch2.txt")));
		}

		System.out.println("done.");
	}

	/** One emitter's worth of work. */
	private interface Emit {
		void run() throws Exception;
	}

	/** Run one emitter; on failure say which one, print the stack trace, and exit 3. */
	private static void stage(final String name, final Emit body) throws Exception {
		System.out.println("--- stage: " + name);
		try {
			body.run();
		} catch (Throwable t) {
			// Write the trace to a FILE as well as stderr. Bootstrap installs
			// logging that can swallow stderr, and a stage failure with no
			// visible message is exactly as useless as no failure at all.
			String msg = "STAGE FAILED: " + name + ": " + t;
			System.err.println(msg);
			try (java.io.PrintWriter w = new java.io.PrintWriter(
					new java.io.FileWriter("stage-error.txt", true))) {
				w.println(msg);
				t.printStackTrace(w);
			}
			System.err.flush();
			System.exit(3);
		}
	}
}