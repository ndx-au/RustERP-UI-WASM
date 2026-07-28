# AGENTS.md — RustERP-UI-WASM

## Font / icon glyphs (mandatory)

- Rail icons MUST stay Unicode: Home `⌂`, Parties `♟`, Catalog `☰`, Sales `¤`, Settings `⚙` (or a Spec-updated set).
- **FORBIDDEN:** replacing icons with ASCII letters as a “font fix”.
- **REQUIRED when tofu appears:** register an embedded fallback font covering those codepoints. DejaVu Sans at `assets/fonts/DejaVuSans.ttf` is staged and covers `⌂♟☰¤⚙` and geometric squares. Use `FontDefinitions::default()`; insert `FontData::from_static(include_bytes!(...))`; push the family name onto `FontFamily::Proportional` after `Ubuntu-Light`; call `cc.egui_ctx.set_fonts` in `App::new`.
- **Reviewer:** FAIL any diff that swaps icon glyphs to Latin letters without a Spec explicitly requiring letter icons.
- Residual `□` means another codepoint is still missing from fallback coverage — extend font coverage; do not delete the character.
