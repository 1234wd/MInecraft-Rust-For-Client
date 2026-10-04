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
		"net.minecraft.world.phys.Vec2",
		"net.minecraft.world.phys.AABB",
		"net.minecraft.util.ARGB",
		"net.minecraft.core.Rotations",
		"net.minecraft.resources.Identifier",
	};

	public static void main(final String[] args) throws Exception {
		Path testData = Path.of(args.length > 0 ? args[0] : "../test-data");
		boolean noWrite = List.of(args).contains("--no-write");
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

		MthOracle.emit(testData.resolve("mth.txt"));
		MthOracle.emitTables(testData.resolve("mth_tables.txt"));
		RandomOracle.emit(testData.resolve("random.txt"));
		CoreOracle.emit(testData.resolve("core.txt"));

		System.out.println("done.");
	}
}
