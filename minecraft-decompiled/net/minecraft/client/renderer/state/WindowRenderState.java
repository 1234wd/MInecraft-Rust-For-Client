package net.minecraft.client.renderer.state;

import net.fabricmc.fabric.api.client.rendering.v1.FabricRenderState;

public class WindowRenderState implements FabricRenderState {
	public int width;
	public int height;
	public int guiScale;
	public float appropriateLineWidth;
	public boolean isMinimized;
}
