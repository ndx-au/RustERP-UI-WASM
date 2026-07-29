# RustERP-UI-WASM by NDX Pty Ltd

**Official site:** [https://RustERP.biz](https://RustERP.biz)

RustERP-UI-WASM by NDX Pty Ltd is a **reference UI consumer** of
[RustERP](https://github.com/ndx-video/RustERP) — a modular, snug-fit,
open-source ERP written in Rust. This repository focuses on browser/WASM
client surfaces that talk to RustERP as a **headless API** (gRPC-over-WebSocket
via **slozhn** + protobufs). It does **not** contain ERP domain logic or storage.

This repository is the open-source project source for the reference UI.
Product information, documentation, and project news for the RustERP family
live on the official site.

## Source & community

| | |
|--|--|
| **Website** | [RustERP.biz](https://RustERP.biz) |
| **Source (this UI)** | [github.com/ndx-video/RustERP-UI-WASM](https://github.com/ndx-video/RustERP-UI-WASM) |
| **Source (ERP core)** | [github.com/ndx-video/RustERP](https://github.com/ndx-video/RustERP) |
| **Contributing** | [CONTRIBUTING.md](./CONTRIBUTING.md) |

## Status

**Phase 2 — App shell chrome (egui) + schema wireframe IA.**

Permanent **egui/eframe** application chrome mapping the RustERP PostgreSQL schema
([`docs/schema.md`](../RustERP/docs/schema.md) in the core repo). Every domain and page
is navigable; content areas show **wireframe stubs** (schema path + placeholder) except
**Parties list pages** (live gRPC) and **Settings → Connection** (live).

| Chrome | Role |
|--------|------|
| **Icon rail** (~56px) | MVP domains (Home → Inventory), divider, post-MVP stubs, Settings pinned bottom |
| **Domain menu** (~200px) | Pages for the selected domain |
| **Top bar** (~48px) | Logo slot + connection status (honest empty/error; no fabricated rows) |
| **Content host** | Live Parties grid, Settings panes, or wireframe stub |

### Icon rail (top → bottom)

| Domain | Icon | Tier | Schema |
|--------|------|------|--------|
| Home | `⌂` | Wireframe | `core` |
| Parties | `♟` | **Live list** | `party` |
| Catalog | `☰` | Wireframe | `catalog` |
| Sales | `¤` | Wireframe | `sales` |
| Payments | `⊕` | Wireframe | `payment` |
| Inventory | `⌗` | Wireframe | `inventory` |
| — divider — | | | |
| Purchasing | `⇩` | Post-MVP stub | `purchase` |
| Accounting | `Σ` | Post-MVP stub | `accounting` |
| CRM | `☎` | Post-MVP stub | `crm` |
| Projects | `▣` | Post-MVP stub | `project` |
| HR | `⚲` | Post-MVP stub | `hr` |
| Manufacturing | `⚒` | Post-MVP stub | `manufacturing` |
| Settings | `⚙` | Live + wireframe tabs | platform / `auth` |

| Domain | Pages / panes |
|--------|----------------|
| **Home** | Dashboard (wireframe) |
| **Parties** | All Parties / Customers / Suppliers (**live**); Prospects / Contacts / Addresses (wireframe) |
| **Catalog** | Products, Categories, Units of Measure, Price Lists |
| **Sales** | Quotes, Orders, Invoices, Credit Notes |
| **Payments** | Payments, Bank Accounts, Allocations |
| **Inventory** | Warehouses, Stock Levels, Stock Moves |
| **Purchasing … Manufacturing** | Post-MVP stub pages (one page each) |
| **Settings** (sprocket, rail bottom) | Tabs **Connection \| Modules \| Users & Roles \| About** |

Design/IA reference (not runtime): local [`stitch/`](./stitch/) (`screen.png`, `DESIGN.md`, `code.html`). Approximate tokens only — no HTML/CSS/Tailwind shell, no Stitch codegen, no Material Symbols.

### Live RPC

| Surface | Live RPC to core? | Notes |
|---------|-------------------|--------|
| **Native** (`cargo run -p rusterp-ui --features native`) | **Yes** | slozhn → `ws://127.0.0.1:8123/rpc` (Health + ListParties) |
| **WASM** (`trunk build` / served from core `dist/`) | **Yes** | Page-relative `ws(s)://host/rpc`; same-origin when core serves static |

No create/update/delete Party UI, no auth, no Catalog/Sales product screens. Empty list and
errors never invent sample rows.

### RPC URL configuration

| Mechanism | Detail |
|-----------|--------|
| Env (primary) | `RUSTERP_RPC_URL` |
| Env (legacy) | `RUSTERP_GRPC_ENDPOINT` — `http://host:50051` maps to `ws://host:8123/rpc` |
| CLI (native) | `--endpoint <uri>` or `--endpoint=<uri>` |
| Default | `ws://127.0.0.1:8123/rpc` |
| WASM | Page-relative `/rpc` when served from core HTTP listener |
| In-UI | **Settings → Connection** (RPC URL field + Refresh / Retry); status also in top bar |

## Intended stack

| Layer | Choice |
|-------|--------|
| Language | Rust |
| UI | [egui](https://github.com/emilk/egui) immediate-mode via [eframe](https://github.com/emilk/egui/tree/main/crates/eframe) |
| Browser target | WASM (`wasm32-unknown-unknown`) on canvas |
| Bundler | [trunk](https://trunkrs.dev/) (`Trunk.toml` + root `index.html`) |
| Core transport | **slozhn** gRPC-over-WebSocket (Macaron pattern) |
| Async bridge | `spawn_local_fut` + `SharedResult` poll (never block egui loop) |
| Role | API **consumer** only — authority stays on the core |
| Protos | Vendored copies under [`proto/`](./proto/) (from RustERP); client codegen in `rusterp-api-client` |
| Design ref | [`stitch/`](./stitch/) — IA/tokens only |

## Crate layout

Cargo workspace (virtual root):

| Crate | Path | Role |
|-------|------|------|
| `rusterp-ui` | `crates/rusterp-ui` | eframe app shell (chrome + Parties + Settings) |
| `rusterp-api-client` | `crates/rusterp-api-client` | slozhn client, status/view-models, Macaron conn helpers |

## Prerequisites

- Rust toolchain (edition 2021; current stable is fine)
- [`protoc`](https://grpc.io/docs/protoc-installation/) on `PATH` (client codegen)
- A checkout of [RustERP](https://github.com/ndx-video/RustERP) to run the core server (separate repo)
- `wasm32-unknown-unknown` target (WASM builds):
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- [trunk](https://trunkrs.dev/) for WASM bundling:
  ```bash
  cargo install trunk
  ```

## Run core + UI together (manual smoke)

### 1. Start RustERP core (separate repo)

```bash
cd /path/to/RustERP
cargo run -p rusterp-server
```

This starts **dual transport**:

- TCP gRPC: `127.0.0.1:50051` (grpcurl / API tools)
- HTTP + slozhn: `127.0.0.1:8123` with WebSocket at `/rpc`

Optional seed via grpcurl on TCP port:

```bash
grpcurl -plaintext -d '{"display_name":"Demo Co","roles":["PARTY_ROLE_CUSTOMER"]}' \
  127.0.0.1:50051 rusterp.party.v1.PartyService/CreateParty
```

### 2. Run this UI (native — shell + live slozhn)

```bash
cd /path/to/RustERP-UI-WASM
cargo run -p rusterp-ui --features native
# or:
RUSTERP_RPC_URL=ws://127.0.0.1:8123/rpc cargo run -p rusterp-ui --features native
```

**Expected (shell smoke)**

1. Window shows **icon rail** + **domain menu** + **top bar** + content.
   Unicode rail icons (`⌂ ♟ ☰ ¤ ⚙`) render correctly thanks to embedded DejaVu Sans fallback.
2. **Parties → All Parties**: status moves to **connecting**, then **connected**
2. **Parties → All Parties**: status moves to **connecting**, then **connected**
   (or honest **empty** list). Top bar shows connection status.
3. **Settings sprocket** (rail bottom) → Settings menu → tabs **Connection | About**.
   Connection shows RPC URL / env hints; **Refresh / Retry** uses existing client.
4. Stop the core server (or set a bad RPC URL) and Refresh/Retry → **error** in
   top bar and content, **no fabricated rows**.
5. Customers / Suppliers menu rows show the same list with a short “not filtered yet” note.

### 3. WASM (live slozhn, same-origin)

Build UI into RustERP `dist/` for single-origin smoke:

```bash
cd /path/to/RustERP-UI-WASM
trunk build --release
cp -r dist/* /path/to/RustERP/dist/
cd /path/to/RustERP
cargo run -p rusterp-server
# open http://127.0.0.1:8123/
```

Or use `trunk serve` with a proxy to core `:8123` during development.
Same chrome; status stays honest (no invented Connected/rows on failure).

## Build & check

From the repository root:

```bash
# Native / workspace typecheck
cargo check

# Client unit tests (no live server required)
cargo test -p rusterp-api-client

# Shell navigation unit tests
cargo test -p rusterp-ui

# Native window shell
cargo run -p rusterp-ui --features native

# WASM typecheck / bundle
cargo check -p rusterp-ui --target wasm32-unknown-unknown
trunk build
```

Requires `protoc` for `rusterp-api-client` codegen (all targets).

## License

**RustERP-UI-WASM by NDX Pty Ltd** is licensed under the **Apache License,
Version 2.0**. See [LICENSE](./LICENSE) and [NOTICE](./NOTICE).

Copyright 2026 NDX Pty Ltd and contributors.

You are free to use, modify, and redistribute RustERP-UI-WASM — including in
commercial and internal products — under those terms. Contributions are welcome
under the same license; see [CONTRIBUTING.md](./CONTRIBUTING.md).

Third-party Rust dependencies (for example egui/eframe, tonic/prost, slozhn;
typically MIT OR Apache-2.0) are pulled from crates.io; see each crate’s license
metadata and `Cargo.lock` for exact versions. Vendored `.proto` contracts retain
their upstream Apache-2.0 headers from RustERP.

## Static asset compression

WASM binaries are large (~5.5 MB for the current shell). The deploy script
(`RustERP/dist/deploy-ui-stack.sh`) generates **zstd-precompressed** `.zst`
siblings for every static asset (`.wasm`, `.js`, `.html`) at deploy time.

Caddy serves these via `file_server { precompressed zstd }` — zero runtime
compression cost. Browsers that send `Accept-Encoding: zstd` receive the
precompressed file directly (~2.0 MB, a 63% reduction).

**Why zstd over brotli or gzip:**

| Codec | Size | Compress time | Notes |
|-------|------|---------------|-------|
| raw | 5.51 MB | — | no compression |
| zstd -19 | 2.03 MB (63.2% saved) | 4s | best balanced; no extra deps |
| brotli -q 11 | 1.91 MB (65.3% saved) | 19s | marginal gain, slow, needs brotli CLI |
| gzip -9 | 2.42 MB (56.1% saved) | 1s | worst ratio |

zstd wins on the size/effort tradeoff. Brotli q11 only saves ~117 KB more for
5x the compress time and an extra binary dependency. The Caddy image already
has `http.precompressed.zstd` built in — no plugin rebuild needed.
