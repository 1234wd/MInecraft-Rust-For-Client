// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `net.minecraft.resources.Identifier`, reduced to what
// PositionalRandomFactory.fromHashOf(Identifier) touches: toString().
//
// NOT parity-tested: the real Identifier#toString is "namespace + ':' + path",
// but the real class also carries validation/parsing rules that are themselves
// part of the porting work (net/minecraft/resources/Identifier.rs is still
// SKELETON). Stubbing it here would test the stub, not vanilla. The oracle
// therefore exercises fromHashOf(String) -- which is the actual seeded path --
// and fromHashOf(Identifier) is left for a later batch once Identifier is ported.
//
// See _porting/DESIGN_DECISIONS.md (#stubs) and OPEN_QUESTIONS.md.
// ============================================================================
package net.minecraft.resources;

public class Identifier {
	private final String namespace;
	private final String path;

	public Identifier(final String namespace, final String path) {
		this.namespace = namespace;
		this.path = path;
	}

	@Override
	public String toString() {
		return this.namespace + ":" + this.path;
	}
}