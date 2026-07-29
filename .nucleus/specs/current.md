# Nucleus Spec — Restore Unicode rail icons and font fallback

## Goal

Restore the intended Unicode rail icons and fix egui glyph tofu by embedding DejaVu Sans as a proportional fallback font. Reverse the fake ASCII-letter icon workaround.

## Constraints

- Rail icons MUST be:
  - Home: `⌂`
  - Parties: `♟`
  - Catalog: `☰`
  - Sales: `¤`
  - Settings: `⚙`
- **FORBIDDEN:** ASCII letter icons (`H` / `P` / `C` / `S` / `S2`) as the fix.
- Embed `assets/fonts/DejaVuSans.ttf` with `include_bytes!` (already staged).
- Use `FontDefinitions::default()`; insert DejaVu; push it onto `FontFamily::Proportional` **after** the Ubuntu-Light primary; call `set_fonts` in `ReferenceApp::new` using `&cc` (use the `CreationContext` parameter — stop ignoring `_cc`).
- No emoji color fonts are required; DejaVu Sans is sufficient for these codepoints.
- Apache-2.0 / NOTICE: DejaVu is free (Bitstream Vera derived) — add a brief NOTICE line if needed.
- Consumer-only; no new RPCs or domains.

## Acceptance Criteria

- [ ] `Domain::icon` is restored to `⌂`, `♟`, `☰`, `¤`, and `⚙` (not letters).
- [ ] Fonts are registered: DejaVu Sans is embedded and present in the `FontFamily::Proportional` fallback chain after Ubuntu.
- [ ] `AGENTS.md` font rules remain intact.
- [ ] The following checks are attested via `nucleus_attest`:
  - `cargo check --workspace`
  - `cargo test --workspace`
  - `cargo check -p rusterp-ui --target wasm32-unknown-unknown`
- [ ] Optional unit test or comment documents the embedded font path.
- [ ] README includes a one-line note about the symbol font fallback, if helpful.
- [ ] Manual smoke is documented: rail shows symbols, not letters or `□` (user will verify in browser after deploy).

## Out-of-Scope

- Full custom design fonts (Geist / Inter).
- Material Symbols or web-font icon pipelines.
- Letter icons.
- New domains or CRUD behavior.
- Changing the core server.

## Decision Log / Open Questions

| Decision / Question | Status | Notes |
|---------------------|--------|-------|
| Font fix approach | decided | Embed DejaVu Sans fallback and keep Unicode icons. |
| ASCII workaround | rejected | Fake fix; reverse it. |
