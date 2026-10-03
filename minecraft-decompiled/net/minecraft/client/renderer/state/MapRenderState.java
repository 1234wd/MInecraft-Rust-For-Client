package net.minecraft.client.renderer.state;

import java.util.ArrayList;
import java.util.List;
import net.fabricmc.fabric.api.client.rendering.v1.FabricRenderState;
import net.minecraft.client.renderer.texture.TextureAtlasSprite;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import org.jspecify.annotations.Nullable;

public class MapRenderState implements FabricRenderState {
	public @Nullable Identifier texture;
	public final List<MapRenderState.MapDecorationRenderState> decorations = new ArrayList<>();

	public static class MapDecorationRenderState implements FabricRenderState {
		public @Nullable TextureAtlasSprite atlasSprite;
		public byte x;
		public byte y;
		public byte rot;
		public boolean renderOnFrame;
		public @Nullable Component name;
	}
}
