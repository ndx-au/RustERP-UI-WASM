# Nucleus Spec — Phase 2: App shell chrome (egui)

## Goal

Implement the permanent **application shell** for the RustERP reference UI in
**egui/eframe only**: left icon rail, domain menu column, top bar, and content
host with optional page header / page tabs. Match the agreed information
architecture and local `stitch/` mockups as **visual/IA reference** (not a
runtime asset pipeline). Reuse Phase 1 Parties list + connection status inside
the chrome. No new ERP domains or transport surface.

## Constraints

- **egui/eframe only** for layout and widgets. No HTML/CSS/DOM shell, dioxus,
  iced, or web-component chrome. Do not wire `stitch/code.html` into the runtime.
- **Forbidden runtime paths:** HTML/CSS/Tailwind, Stitch-to-web codegen, Material
  Symbols (or other web-font icon pipelines).
- **Consumer only:** no ERP domain logic, persistence, or new gRPC methods beyond
  Phase 1 (`Health` + `ListParties` and existing client status/endpoint config).
- **Honesty:** no fabricated party rows; empty/error stay honest. WASM live gRPC
  remains **deferred** — no fake Connected on the browser path.
- **Native primary:** live refresh against core stays the native tonic path from
  Phase 1. WASM shell must keep compiling with shared chrome + honest status.
- **Snug-fit rail:** active domains are **Parties** + **Settings** only. Optional
  Home / Catalog / Sales rail placeholders must be clearly non-functional
  (tooltip “not enabled” or no-op).
- **Design reference (`stitch/`):** `screen.png`, `DESIGN.md`, and `code.html`
  are **IA and token reference only** — structure and hierarchy over pixel
  parity. Approximate in egui (not hard CI metrics):
  - `rail_width` ≈ 56, `menu_width` ≈ 200, `top_bar_height` ≈ 48,
    `pane_padding` ≈ 16, dense row ≈ 28, rounding ≈ 0
  - Color intent: dark surfaces; primary/accent **rust-orange** for active rail
    indicator and primary actions
  - Do **not** require exact hex parity or custom Geist/Inter/JetBrains fonts
    (system / egui defaults OK)
- **Stack split:** presentation in `rusterp-ui`; transport/stubs in
  `rusterp-api-client`. Apache-2.0; new deps redistribution-compatible.
- **Attestation-first:** automated checks via `nucleus_attest`; live core smoke
  remains documented manual proof, not a fake CI claim.

## Acceptance Criteria

- [ ] **Shell state model** in UI code with at least:
  - `selected_domain`
  - `selected_page` (within domain)
  - `selected_tab` (page-local; used on multi-pane Settings)
- [ ] **Icon rail** (fixed width ~56): domain icons via **egui built-ins and/or
  simple unicode** + tooltips; **Settings (sprocket) pinned at bottom**.
  Optional Home/Catalog/Sales entries are non-functional placeholders only.
- [ ] **Domain menu column** (~200) driven by `selected_domain`:
  - **Parties:** **All Parties** (embeds existing Phase 1 list). Optional
    Customers/Suppliers menu rows may route to the same list or a short
    “not filtered yet” placeholder — **no new RPCs**, no Search/Filter product.
  - **Settings:** menu leads into Settings content with **tabbed panes**
    (see below).
- [ ] **Top bar** (~48): white-label logo placeholder; flex status / system-message
  region wired to existing `ConnectionStatus` (short error text when Error;
  stronger/error styling allowed). No pretend live Connected on WASM.
- [ ] **Content host:**
  - **Parties → All Parties** embeds the Phase 1 Parties list inside the chrome
    (native live path still works when core is reachable on plaintext endpoint);
  - **Settings** (via bottom sprocket → Settings domain): **multi-pane tabs**
    **Connection | About**
    - **Connection:** endpoint display; document / surface
      `RUSTERP_GRPC_ENDPOINT`; edit field only if already trivial from Phase 1;
      status + optional **Retry** that triggers existing refresh
    - **About:** version / consumer-only disclaimer (no marketing wall of text)
  - honest **placeholder / empty** states for non-enabled menu rows (no invented
    domain product UIs).
- [ ] **Page tabs:** Settings demonstrates the multi-pane pattern
  (Connection | About). **Single-pane pages (e.g. All Parties) must not show a
  lonely tab strip.**
- [ ] **Error / disconnected UI (acceptance bar):**
  - Top bar shows `ConnectionStatus`; may emphasize Error state
  - Content: honest empty list + short message; optional Retry → existing refresh
  - **Not required:** pulse animations, full marketing copy, or HTML-mock ghost
    table for acceptance
- [ ] **Settings reachable** from the bottom sprocket (selects Settings domain /
  Connection context).
- [ ] **Switching domain** updates the menu column; **switching page** updates
  content; **switching Settings tab** switches pane (manual smoke or cheap
  UI-logic test).
- [ ] **No HTML UI path** added to the product runtime; no Material Symbols /
  Stitch codegen dependency.
- [ ] **WASM** still checks/builds (`cargo check -p rusterp-ui --target
  wasm32-unknown-unknown` **or** `trunk build`); live browser gRPC remains
  deferred with **honest** non-Connected status (no fake Connected).
- [ ] **README updated:** Phase 2 shell description; `stitch/` as design/IA
  reference; chrome → domains/pages/tabs map; Settings Connection | About;
  native vs WASM honesty unchanged from Phase 1; core link retained; no full
  CRM/auth/TLS claims.
- [ ] **Automated (attested via `nucleus_attest`):**
  - `cargo check` (workspace or default members);
  - relevant unit/logic tests (existing client tests still pass; add UI-safe
    pure tests for shell navigation mapping if non-trivial);
  - WASM path as above.
- [ ] **Manual smoke documented** (README or `docs/`): native shell layout
  visible → Parties → All Parties lists live/empty data with core up → status
  in top bar → Settings sprocket → Connection | About tabs → bad endpoint /
  stop core → Error without fake rows; optional Retry.
- [ ] **License/NOTICE** coherent; lockfile updated if deps change.

## Out-of-Scope

- HTML/CSS/Tailwind runtime, Stitch-to-web / `code.html` codegen, Material Symbols
- Real Catalog, Sales, Inventory, Invoices, Home, or other non-Party domain screens
- New Party / Search / Filter **product** behavior (omit or disabled placeholders only)
- Party detail, GetParty, or Create/Update/Delete Party UI
- Auth, full theming productization, white-label asset pipeline beyond logo slot
- Collapsible rail, mobile responsive breakpoints
- Client TLS/HTTPS productization (plaintext local endpoint remains valid)
- Changing core protos or server behavior
- Macaron/slozhn live WASM transport (separate future change)
- Pixel-perfect Stitch parity, custom web fonts, pulse/ghost-table marketing UI
- Claiming Macaron parity or production-readiness

## Decision Log / Open Questions

| Decision / Question | Status | Notes |
|---------------------|--------|-------|
| Primary UI toolkit | **decided** | egui/eframe only; `stitch/` is IA/token reference, not runtime. |
| Design tokens (approx) | **decided** | rail 56 / menu 200 / top 48 / pad 16 / row ~28 / rounding 0; dark + rust-orange accent; egui/system fonts OK. |
| MVP domains | **decided** | **Active:** Parties, Settings. **Optional placeholders:** Home, Catalog, Sales (“not enabled” / no-op). |
| MVP pages | **decided** | Parties: **All Parties** (Phase 1 list). Optional Customers/Suppliers → same list or “not filtered yet” (no new RPCs). Settings: single settings page host with tabs. |
| Multi-tab vehicle | **decided** | **Settings:** panes **Connection \| About**. No Party detail / GetParty this phase. |
| GetParty / Party detail | **out** | Not required; do not expand gRPC surface for tabs. |
| Rail icons | **decided** | egui built-ins and/or simple unicode only; no Material Symbols / web fonts. |
| Domain menu collapsible | **decided** | **No** this phase. |
| Top-bar status wiring | **decided** | Reuse Phase 1 `ConnectionStatus` (+ short error; stronger Error styling OK); WASM stays honest / non-live. |
| Error/disconnected bar | **decided** | Honest empty + short message; optional Retry → existing refresh. No pulse/ghost-table requirement. |
| Endpoint config | **carried from Phase 1** | `RUSTERP_GRPC_ENDPOINT` default `http://127.0.0.1:50051`; optional CLI `--endpoint`; in-UI edit only if trivial. |
| Transport change | **out** | No slozhn / gRPC-Web in this change. |

---

When satisfied: `/spec approve`, then `/implement`.
