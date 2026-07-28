# Nucleus Spec — Phase 0: UI Foundations

## Goal

Establish a verifiable **reference WASM UI skeleton** for RustERP-UI-WASM: an egui/eframe-style app shell that can compile (native check + WASM path declared), with clear crate separation for future gRPC/protobuf client stubs, honest consumer-only docs, and repo hygiene matching the RustERP core policy. No live ERP connectivity and no domain screens.

## Constraints

- **Consumer only:** this repo must not embed RustERP domain logic, storage, or business rules. Placeholder/mock UI data only.
- **Stack direction (locked):** Rust; egui immediate-mode UI via eframe (or equivalent egui app shell); target browser/WASM (canvas). gRPC-over-WebSocket client is **future** — stubs/layout only in this phase.
- **License:** Apache-2.0 remains; do not relicense or add conflicting top-level licenses. New deps must be redistribution-compatible with Apache-2.0.
- **Separation:** UI presentation crate(s) distinct from a future/generated client-stub crate (or clearly reserved path/module boundary even if the stub crate is empty/placeholder).
- **Nucleus hygiene:** `.gitignore` tracks `.nucleus/` except secrets (at least `attest.key`), aligned with core policy; do not commit attestation keys.
- **Honesty:** README must state this is a **reference consumer** of [RustERP](https://github.com/ndx-video/RustERP), link the core repo, and document the intended stack without claiming live API integration.
- **Scope discipline:** foundations only — no Parties/invoices/ERP screens, no auth productization, no multi-tenant shell product claims.
- **Attestation-first:** acceptance checks must be runnable via `nucleus_attest` (no “works on my machine” claims).

## Acceptance Criteria

- [ ] **Cargo workspace/skeleton exists** with at least:
  - one UI app crate (egui/eframe-style shell), and
  - a clearly separated placeholder for future proto/gRPC client stubs (dedicated crate **or** reserved `client`/`proto` path documented in README and empty of domain logic).
- [ ] **Minimal app shell** runs as an empty/hello shell (window or WASM canvas host) with placeholder content only (e.g. title + short “RustERP reference UI — not connected” text). No fabricated ERP entities.
- [ ] **`cargo check`** (workspace or default members) succeeds and is attested via `nucleus_attest`.
- [ ] **WASM toolchain path is declared** in README (rustup target `wasm32-unknown-unknown`, and chosen bundler — default nomination: **trunk** unless Decision Log changes it). If trunk (or equivalent) is wired, **`trunk build`** (or the nominated equivalent) succeeds and is attested; if bundler wiring is deferred, README must say so explicitly and AC is limited to `cargo check --target wasm32-unknown-unknown` attested instead — **one** of these two WASM paths must be attested, not neither.
- [ ] **README** includes: purpose as reference consumer of RustERP; link to `https://github.com/ndx-video/RustERP`; intended stack (Rust, egui/eframe, WASM, future gRPC-over-WebSocket); how to build/check locally; explicit **Status** that live ERP talk is out of scope for this phase.
- [ ] **`.gitignore`** covers Rust (`target/`, etc.), WASM/bundler artifacts (e.g. `dist/`, trunk output), and Nucleus secrets (ignore `.nucleus/attest.key` and similar keys) while allowing `.nucleus/specs/`, attestations metadata policy consistent with tracking `.nucleus/` tree as appropriate.
- [ ] **No domain screens** introduced (no Parties, invoices, inventory, etc.).
- [ ] **License/NOTICE** remain coherent; new third-party notice obligations (if any from deps) reflected per CONTRIBUTING guidance or flagged in Decision Log if deferred with justification.

## Out-of-Scope

- Live gRPC/WebSocket connection to a running RustERP core
- Protobuf codegen from RustERP `.proto` files (beyond empty crate/path reservation)
- Domain UI: parties, invoices, orders, inventory, auth flows, settings product UI
- Design system polish, theming productization, accessibility audit (beyond whatever defaults eframe/egui provide)
- CI configuration (GitHub Actions, etc.) unless required to make local attest commands discoverable — prefer README over CI in Phase 0
- Packaging/publishing to crates.io or npm
- Multi-window desktop product features beyond what eframe needs for a hello shell
- Claiming Macaron feature parity or any production-readiness

## Decision Log / Open Questions

| Decision / Question | Status | Notes |
|---------------------|--------|-------|
| WASM bundler for Phase 0 | **nominated** | **trunk** + `Trunk.toml` as default; implementer may switch only if blocked, and must update README + this log. |
| Dual target (native + WASM) in Phase 0 | **nominated** | `cargo check` native **and** one attested WASM path (`trunk build` **or** `cargo check --target wasm32-unknown-unknown`). Native egui window is fine for dev smoke. |
| Crate layout | **nominated** | Cargo workspace: `crates/rusterp-ui` (eframe app) + `crates/rusterp-api-client` (placeholder, no real RPCs). Root virtual workspace. Names adjustable if clearly equivalent. |
| egui/eframe version pin | **open** | Prefer current stable crates.io releases compatible with WASM; pin in `Cargo.toml`/`Cargo.lock` and commit lockfile. |
| Commit `Cargo.lock` for binary workspace | **nominated** | Yes — app/reference binary workspace should lock deps for reproducible attestations. |
| Proto path source of truth | **deferred** | No submodule/path dep on RustERP protos in Phase 0; document future intent only. |
| `.nucleus/` git policy detail | **nominated** | Track `.nucleus/` (specs, state, attestations as repo policy allows); **ignore** `attest.key` and other secrets. Match core when in doubt. |
| Existing root README/CONTRIBUTING/LICENSE | **keep** | Extend README for stack/build; do not rewrite license or DCO/CONTRIBUTING unless required for accuracy. |

---

When satisfied: `/spec approve`, then `/implement`.
