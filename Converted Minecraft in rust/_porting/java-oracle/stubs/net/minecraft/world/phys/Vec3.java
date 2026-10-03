// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `net.minecraft.world.phys.Vec3`, reduced to the three public
// double fields that Mth.rayIntersectsAABB / Mth.lerp(double,Vec3,Vec3) read.
// Field names match the real class exactly.
//
// On a parity-tested path: Mth.rayIntersectsAABB and Mth.lerp(double,Vec3,Vec3)
// are tested.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package net.minecraft.world.phys;

public class Vec3 {
	public static final Vec3 ZERO = new Vec3(0.0, 0.0, 0.0);

	public double x;
	public double y;
	public double z;

	public Vec3(final double x, final double y, final double z) {
		this.x = x;
		this.y = y;
		this.z = z;
	}
}