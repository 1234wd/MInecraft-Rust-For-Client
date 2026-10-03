// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `net.minecraft.util.ARGB.color(int,int,int,int)`. The body below
// is a verbatim transcription of minecraft-decompiled/net/minecraft/util/ARGB.java
// lines 68-70, so Mth.hsvToArgb's return value is bit-identical.
//
// On a parity-tested path: Mth.hsvToArgb is tested.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package net.minecraft.util;

public class ARGB {
	private ARGB() {
	}

	public static int color(final int alpha, final int red, final int green, final int blue) {
		return (alpha & 0xFF) << 24 | (red & 0xFF) << 16 | (green & 0xFF) << 8 | blue & 0xFF;
	}
}