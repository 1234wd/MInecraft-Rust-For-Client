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

public final class Oracle {
	private static final String[] TRACKED = {
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
	};

	public static void main(final String[] args) throws Exception {
		Path testData = Path.of(args.length > 0 ? args[0] : "../test-data");
		System.out.println("oracle -> " + testData.toAbsolutePath());

		for (String name : TRACKED) {
			Class<?> c = Class.forName(name);
			System.out.println("  class " + name + " <- " + c.getProtectionDomain().getCodeSource().getLocation());
		}

		MthOracle.emit(testData.resolve("mth.txt"));
		MthOracle.emitTables(testData.resolve("mth_tables.txt"));
		RandomOracle.emit(testData.resolve("random.txt"));

		System.out.println("done.");
	}
}