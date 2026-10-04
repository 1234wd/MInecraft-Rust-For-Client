// ============================================================================
// ORACLE ENTRY POINT -- not game code.
//
// Compiles the ORIGINAL sources from minecraft-decompiled/ plus the stubs in
// ../stubs and the third-party jars in ../lib, then writes golden data into
// _porting/test-data/.
//
// Build & run:  _porting/java-oracle/run.ps1
// ============================================================================
package oracle;

import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

public final class Oracle {
	/**
	 * Every class this oracle is supposed to be testing.
	 *
	 * Each MUST be loaded from the directory we compiled it into, never from the
	 * prebuilt Minecraft jar. If the jar shadowed one of these, we would be
	 * measuring Mojang's bytecode while believing we were measuring our reading of
	 * the decompiled text -- which is precisely the thing that must not happen.
	 */
	private static final String[] TRACKED = {
		// --- session 02: util + RNG ---
		"net.minecraft.util.Mth",
		"net.minecraft.util.RandomSource",
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
		// --- session 03: core value types ---
		"net.minecraft.core.Vec3i",
		"net.minecraft.core.BlockPos",
		"net.minecraft.core.Direction",
		"net.minecraft.core.Direction$Axis",
		"net.minecraft.world.level.ChunkPos",
		"net.minecraft.world.phys.Vec3",
		"net.minecraft.world.phys.AABB",
		"net.minecraft.util.ARGB",
		"net.minecraft.resources.Identifier",
	};

	public static void main(final String[] args) throws Exception {
		Path testData = Path.of(args.length > 0 ? args[0] : "../test-data");
		boolean printOrigins = List.of(args).contains("--print-origins");
		Path requiredFrom = null;
		for (int i = 0; i < args.length - 1; i++) {
			if (args[i].equals("--require-origins")) {
				requiredFrom = Path.of(args[i + 1]).toAbsolutePath().normalize();
			}
		}
		System.out.println("oracle -> " + testData.toAbsolutePath());

		List<String> shadowed = new ArrayList<>();
		for (String name : TRACKED) {
			Class<?> c = Class.forName(name);
			String where = String.valueOf(c.getProtectionDomain().getCodeSource().getLocation());
			if (printOrigins) {
				System.out.println("  class " + name + " <- " + where);
			}
			if (requiredFrom != null) {
				Path actual;
				try {
					actual = Path.of(new java.net.URI(where)).toAbsolutePath().normalize();
				} catch (Exception e) {
					shadowed.add(name + " <- " + where + " (unparseable)");
					continue;
				}
				if (!actual.equals(requiredFrom)) {
					shadowed.add(name + " <- " + actual + " (expected " + requiredFrom + ")");
				}
			}
		}

		if (!shadowed.isEmpty()) {
			System.err.println("FAIL: these classes did NOT load from " + requiredFrom + ":");
			shadowed.forEach(s -> System.err.println("    " + s));
			System.err.println("The prebuilt jar shadowed the decompiled sources, so the golden");
			System.err.println("data below would describe Mojang's bytecode, not the text we port.");
			System.exit(2);
		}
		if (requiredFrom != null) {
			System.out.println("  all " + TRACKED.length + " tracked classes came from " + requiredFrom);
		}

		MthOracle.emit(testData.resolve("mth.txt"));
		MthOracle.emitTables(testData.resolve("mth_tables.txt"));
		CoreOracle.emit(testData.resolve("core.txt"));
		RandomOracle.emit(testData.resolve("random.txt"));

		System.out.println("done.");
	}
}