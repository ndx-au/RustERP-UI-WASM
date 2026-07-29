# AGENTS.md — RustERP-UI-WASM

## Font / icon glyphs (mandatory)

- Rail icons MUST stay Unicode. Current set:
  - MVP: Home `⌂`, Parties `♟`, Catalog `☰`, Sales `¤`, Payments `⊕`, Inventory `⌗`, Settings `⚙`
  - Post-MVP: Purchasing `⇩`, Accounting `Σ`, CRM `☎`, Projects `▣`, HR `⚲`, Manufacturing `⚒`
- **FORBIDDEN:** replacing icons with ASCII letters as a “font fix”.
- **REQUIRED when tofu appears:** register an embedded fallback font covering those codepoints. DejaVu Sans at `assets/fonts/DejaVuSans.ttf` is staged and covers the rail set above. Use `FontDefinitions::default()`; insert `FontData::from_static(include_bytes!(...))`; push the family name onto `FontFamily::Proportional` after `Ubuntu-Light`; call `cc.egui_ctx.set_fonts` in `App::new`.
- **After icon changes:** rebuild WASM (`trunk build --release`), redeploy via RustERP `./dist/deploy-ui-stack.sh --bg`, hard-refresh browser — confirm glyphs render (no `□`).
- **Reviewer:** FAIL any diff that swaps icon glyphs to Latin letters without a Spec explicitly requiring letter icons.
- Residual `□` means another codepoint is still missing from fallback coverage — extend font coverage; do not delete the character.
