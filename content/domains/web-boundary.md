# Web Boundary

TypeScript is mandatory for frontend source code. JavaScript source files are
forbidden.

Bun is the preferred web toolchain, not an architectural boundary and not a core
runtime. It is acceptable only while it reduces dependency surface and remains
replaceable without changing product contracts.

## TypeScript owns

- Browser-facing UI and UX workflows.
- Presentation state.
- Form validation at the UI boundary.
- Browser APIs.
- Generated clients and generated types for Rust-owned APIs.
- Web tests and E2E flows.
- Design/prototyping surfaces.

## TypeScript does not own

- Durable business logic.
- API contracts written by hand.
- Authorization or policy truth.
- Persistence or database migrations.
- Background jobs and schedulers.
- Agent orchestration.
- LLM runtime decisions.
- Release, signing, provenance, or deployment tooling.

## Bun posture

Bun may own dependency installation, web app scripts, browser bundle builds,
local dev commands, and frontend tests where it is sufficient.

Bun APIs must not become architecture by accident. `Bun.serve` is allowed for
local dev servers, SSR/web-app-local serving, and disposable mocks. Any durable
or externally-consumed TypeScript server boundary requires an ADR explaining why
Rust is not the right seam.

## Framework posture

Use browser/framework primitives for UI. Do not add Hono, Elysia, Express,
Fastify, or another TypeScript backend framework unless an ADR justifies the
server boundary. The issue is not the framework name; it is TypeScript owning a
durable backend capability.

## Generated contracts

Frontend TypeScript consumes Rust-owned contracts. Prefer generated clients and
types from OpenAPI, WIT, or another Rust-owned schema. Do not hand-write a
TypeScript type for a Rust-owned API contract when generation is possible.

## Source rules

- `.ts` and `.tsx` are the frontend source formats.
- `.js`, `.jsx`, `.mjs`, and `.cjs` source files are forbidden unless generated,
  unversioned, or explicitly allowed by policy.
- `strict: true` is mandatory in `tsconfig.json`.
- `allowJs: true` is forbidden.
- Do not create TypeScript server code before challenging whether it belongs in
  Rust.
