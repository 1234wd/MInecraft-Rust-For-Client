// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `net.minecraft.world.phys.AABB`, reduced to the six public final
// fields that Mth.rayIntersectsAABB(Vec3, Vec3, AABB) reads. The real
// constructor normalises with Math.min/Math.max; so does this one.
//
// On a parity-tested path: Mth.rayIntersectsAABB is tested.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package net.minecraft.world.phys;

public class AABB {
	public final double minX;
	public final double minY;
	public final double minZ;
	public final double maxX;
	public final double maxY;
	public final double maxZ;

	public AABB(final double minX, final double minY, final double minZ, final double maxX, final double maxY, final double maxZ) {
		this.minX = Math.min(minX, maxX);
		this.minY = Math.min(minY, maxY);
		this.minZ = Math.min(minZ, maxZ);
		this.maxX = Math.max(minX, maxX);
		this.maxY = Math.max(minY, maxY);
		this.maxZ = Math.max(minZ, maxZ);
	}
}