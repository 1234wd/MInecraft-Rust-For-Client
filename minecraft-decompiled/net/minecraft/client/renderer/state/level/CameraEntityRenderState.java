package net.minecraft.client.renderer.state.level;

import net.fabricmc.fabric.api.client.rendering.v1.FabricRenderState;

public class CameraEntityRenderState implements FabricRenderState {
	public float hurtTime;
	public int hurtDuration;
	public float deathTime;
	public boolean isSleeping;
	public boolean isLiving;
	public boolean isPlayer;
	public boolean isDeadOrDying;
	public boolean doesMobEffectBlockSky;
	public float hurtDir;
	public float backwardsInterpolatedWalkDistance;
	public float bob;
}
