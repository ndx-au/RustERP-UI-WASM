# Nucleus Spec — Phase 2: Macaron client (slozhn + Tokio patterns)

## Goal

Bring the reference UI in line with [Macaron](https://github.com/ndx-video/macaron)
client patterns: **slozhn** gRPC-over-WebSocket, `spawn_local_fut`, `shared_result`
(Ledger-style unary), feature-gated Tokio, and **live WASM RPC** against the core
HTTP listener (`ws://host/rpc`).

## Constraints

- **Consumer only:** no ERP domain logic; authority stays on core.
- **Honesty:** no fabricated party rows; honest empty/error states.
- **Transport:** primary path is slozhn WebSocket (native + WASM); optional native
  plain tonic to `:50051` may remain for dev parity but UI default is slozhn.
- **Stack:** egui/eframe; `rusterp-ui` vs `rusterp-api-client` separation preserved.
- **Tokio:** WASM gets `sync` only; native `native` feature adds runtime features.
- **Attestation:** `cargo check`, `cargo test`, and `cargo check --target wasm32-unknown-unknown`.

## Acceptance Criteria

- [ ] `rusterp-api-client`: `conn.rs` (`Connection`, `spawn_local_fut`, `shared_result`, `rpc_url`).
- [ ] Unary refresh (Health + ListParties) via slozhn channel on native and WASM.
- [ ] Native `main.rs` enters multi-thread Tokio runtime (`enable_io` + `enable_time`).
- [ ] `app.rs` uses `spawn_local_fut` + `SharedResult` poll (no `std::thread` / per-call runtime).
- [ ] `live_grpc_supported()` true on both targets when slozhn transport is available.
- [ ] Proto codegen enabled for WASM; tonic 0.14.
- [ ] Default RPC URL `ws://127.0.0.1:8080/rpc` (`RUSTERP_RPC_URL` env).
- [ ] README updated: native + WASM live paths; dual-port core setup.
- [ ] Automated checks attested; manual smoke documented.

## Out-of-Scope

- Pulse/Huddle streaming, `bus.rs`, auth, CRUD UI, TLS production setup.
- Macaron parity claims or design-system polish.

## Decision Log

| Decision | Status | Notes |
|----------|--------|-------|
| Primary transport | **decided** | slozhn `ws://…/rpc` |
| Default RPC URL | **decided** | `ws://127.0.0.1:8080/rpc` |
| egui version | **decided** | Keep 0.35 unless build breaks |

---

Approved for implementation.
