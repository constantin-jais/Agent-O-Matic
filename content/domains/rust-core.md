# Rust Core

Rust is the core language for this ecosystem because it gives deterministic
binaries, memory safety without a garbage collector, strong typing, excellent
CLI/service ergonomics, and a credible path to portable artifacts.

## Standard choices

- Async runtime: `tokio`.
- Web services: `axum` + `tower`, with graceful shutdown.
- HTTP client: `reqwest` with `default-features = false` and `rustls` features.
- Serialization: `serde` at boundaries only; domain models stay explicit.
- Database: `sqlx` with PostgreSQL; prefer compile-time checked queries.
- Auth: Biscuit tokens with local validation and typed extractors.
- Observability: `tracing`, structured JSON in production, OpenTelemetry when
  distributed traces are needed.
- Object storage: `object_store` over provider-specific SDKs.
- Jobs: `tokio` for non-critical local work; persisted queues for critical work.
- Errors: `thiserror` for libraries, `miette`/`anyhow` for binaries where
  diagnostics matter.

## Domain discipline

- Parse external input into valid domain types at the boundary.
- Use newtypes for identifiers, tenant IDs, email addresses, percentages,
  non-empty names, and other constrained values.
- Keep DTOs separate from domain types.
- Prefer `TryFrom`/constructors that enforce invariants over repeated runtime
  validation.
- `unwrap()` is forbidden in production paths unless the invariant is local,
  documented, and mechanically obvious.

## Portability discipline

- The portable core is pure Rust and dependency-light.
- Native-only dependencies are forbidden in core crates unless an ADR accepts the
  portability cost.
- Prefer Rust-native TLS, crypto, compression, and storage crates.
- No OpenSSL or `native-tls` in portable paths.
- Cross-target builds are release artifacts, not afterthoughts. Each artifact
  records target triple, version, checksum, provenance, and build inputs.

## Release profile

Use explicit release settings for binaries that are shipped:

```toml
[profile.release]
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

Deviations require measurement or a documented operational reason.

## CI gates

- `cargo fmt --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test` or `cargo nextest run`
- `cargo deny check`
- `cargo audit` or an equivalent RustSec gate with documented exceptions
- portable-core compile check when a crate is declared portable, for example
  `wasm32-unknown-unknown` or the supported release target matrix.

## Forbidden without ADR

- Backend or durable business logic implemented in TypeScript.
- Durable automation implemented as shell scripts.
- Direct dependency on OpenSSL/native TLS.
- Provider-specific storage SDKs when a neutral Rust abstraction works.
- Reimplementing Rust core logic in another language instead of binding it.
