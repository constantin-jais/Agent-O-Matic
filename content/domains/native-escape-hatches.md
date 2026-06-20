# Native Escape Hatches

Zig is not a stack layer and owns no architectural responsibility.

Rust owns portability, build orchestration, release tooling, and native interop
strategy. Zig may appear only as an implementation detail behind Rust-owned
flows.

## Allowed uses

- `cargo-zigbuild` as a Rust cross-compilation helper.
- Temporary native interop spikes when evaluating a C dependency.
- Building or linking a small C dependency when no Rust-native alternative is
  acceptable.
- Low-level asset preparation only when a Rust implementation is measurably
  worse and the output is deterministic.

## Forbidden uses

- Product services.
- Business logic.
- Standalone release tooling.
- Agents or orchestrators.
- Persistence logic.
- Public APIs that consumers depend on.

## ADR trigger

Adding hand-written `.zig` source requires an ADR unless the file is generated,
experimental, or confined to a disposable spike. The ADR must explain why Rust is
not sufficient, how the native boundary is tested, and how the dependency is
removed or contained.

## C interop rules

- Prefer Rust-native crates before C dependencies.
- If C is unavoidable, isolate it behind a small Rust API.
- Document memory ownership at the boundary.
- Test the boundary under sanitizers when possible.
- Never let C/Zig types leak into domain APIs.
