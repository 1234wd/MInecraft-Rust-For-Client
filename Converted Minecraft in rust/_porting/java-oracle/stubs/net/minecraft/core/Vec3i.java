// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `net.minecraft.core.Vec3i`, reduced to what the ported batch
// actually touches: the x/y/z fields and getX()/getY()/getZ().
// Field names and accessor names match the real class exactly, so
// Mth.getSeed(Vec3i) compiles and runs unmodified.
//
// On a parity-tested path: Mth.getSeed(Vec3i) is tested.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package net.minecraft.core;

public class Vec3i implements Comparable<Vec3i> {
	private int x;
	private int y;
	private int z;

	public Vec3i(final int x, final int y, final int z) {
		this.x = x;
		this.y = y;
		this.z = z;
	}

	public int getX() {
		return this.x;
	}

	public int getY() {
		return this.y;
	}

	public int getZ() {
		return this.z;
	}

	@Override
	public int compareTo(final Vec3i o) {
		if (this.y != o.y) {
			return Integer.compare(this.y, o.y);
		} else if (this.z != o.z) {
			return Integer.compare(this.z, o.z);
		} else {
			return Integer.compare(this.x, o.x);
		}
	}
}