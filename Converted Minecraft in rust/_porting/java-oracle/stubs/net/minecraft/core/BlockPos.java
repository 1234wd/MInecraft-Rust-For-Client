// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `net.minecraft.core.BlockPos`, reduced to what
// PositionalRandomFactory.at(BlockPos) touches. The real BlockPos extends Vec3i,
// so the x/y/z accessors are inherited exactly as in vanilla.
//
// On a parity-tested path: PositionalRandomFactory.at(x,y,z) is tested through
// the same code path; the BlockPos overload is a one-line delegate and is
// exercised too (with the stub providing only the inherited accessors).
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package net.minecraft.core;

public class BlockPos extends Vec3i {
	public BlockPos(final int x, final int y, final int z) {
		super(x, y, z);
	}

	public BlockPos() {
		super(0, 0, 0);
	}
}