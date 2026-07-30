# AGENTS.md — RustERP-UI-WASM

Working contract for coding agents in the RustERP reference UI.

**Role:** API **consumer** only — no ERP domain logic or storage here. Authority
stays on [RustERP](https://github.com/ndx-au/RustERP) core.

## Non-negotiables

1. **Honesty over cleverness.** Do not claim builds, tests, or deploys succeeded
   unless you actually ran them and saw them succeed.
2. **Consumer-only.** No new gRPC methods, persistence, or domain rules in this
   repo unless the human explicitly asks and the core already exposes the API.
3. **Apache-2.0** for first-party code; watch third-party license compatibility.

## Font / icon glyphs (mandatory)

- Rail icons MUST stay Unicode. Current set:
  - MVP: Home `⌂`, Parties `♟`, Catalog `☰`, Sales `¤`, Payments `⊕`, Inventory `⌗`, Settings `⚙`
  - Post-MVP: Purchasing `⇩`, Accounting `Σ`, CRM `☎`, Projects `▣`, HR `⚲`, Manufacturing `⚒`
- **FORBIDDEN:** replacing icons with ASCII letters as a “font fix”.
- **REQUIRED when tofu appears:** register an embedded fallback font covering those codepoints. DejaVu Sans at `assets/fonts/DejaVuSans.ttf` is staged and covers the rail set above. Use `FontDefinitions::default()`; insert `FontData::from_static(include_bytes!(...))`; push the family name onto `FontFamily::Proportional` after `Ubuntu-Light`; call `cc.egui_ctx.set_fonts` in `App::new`.
- **After icon changes:** rebuild WASM (`trunk build --release`), redeploy via RustERP `./dist/deploy-ui-stack.sh --bg`, hard-refresh browser — confirm glyphs render (no `□`).
- Residual `□` means another codepoint is still missing from fallback coverage — extend font coverage; do not delete the character.

## Build & deploy

```bash
# Unit tests / check
cargo test -p rusterp-ui
cargo check -p rusterp-ui --features native
cargo check -p rusterp-ui --target wasm32-unknown-unknown

# Browser preview (from sibling RustERP checkout)
cd ../RustERP && ./dist/deploy-ui-stack.sh --bg
```

Do not hand-edit `RustERP/dist/ui/` — that tree is produced by the deploy script.
