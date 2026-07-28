# Contributing to RustERP-UI-WASM by NDX Pty Ltd

Thanks for your interest in **RustERP-UI-WASM by NDX Pty Ltd**. This repository
is a **reference UI implementation** for [RustERP](https://github.com/ndx-video/RustERP)
— browser/WASM-oriented surfaces that consume the ERP as a headless API client.

**Official site:** [https://RustERP.biz](https://RustERP.biz) — product
information, documentation, and project news for the RustERP family. This
repository is the open-source source tree for the reference UI; use the site as
the public face of the product, and GitHub for code, issues, and pull requests.

Adoption and reuse are welcome: run it, fork the patterns into your own
deployment UI, or send improvements back.

## License

RustERP-UI-WASM by NDX Pty Ltd is licensed under the **Apache License,
Version 2.0**. See [LICENSE](./LICENSE) and [NOTICE](./NOTICE).

Copyright 2026 NDX Pty Ltd and contributors.

**By contributing, you agree that your contributions are licensed under the
Apache License 2.0** (the same terms as the rest of the project). You retain
copyright in your contributions; the license grants others the rights they
need to use, modify, and redistribute the combined work — including the
explicit patent grant that makes Apache 2.0 a good fit for commercial and
internal product adoption.

We do **not** require a separate CLA. A pull request is enough.

### Developer Certificate of Origin (DCO)

To keep provenance clear without paperwork, we use the [Developer
Certificate of Origin](https://developercertificate.org/). Sign off each
commit:

```bash
git commit -s -m "Your clear change description"
```

That appends a `Signed-off-by: Your Name <you@example.com>` line. It
certifies that you wrote the change (or have the right to submit it) under
the project license. Use the same name and email as in your git config.

If you forgot `-s` on recent commits:

```bash
git rebase HEAD~N --signoff   # adjust N; only rewrite unpushed history
```

## Ways to participate

You do not have to land a large feature to help:

| Kind | Examples |
|------|----------|
| **Use it** | Run the reference UI against RustERP; file what is unclear or slow |
| **Docs** | Clarify setup, WASM build steps, architecture notes, adoption stories |
| **Bugs** | Minimal repros, rendering/layout issues, API client edge cases |
| **Code** | Screens, UX, tests, packaging, performance, accessibility |
| **Integrations** | Deploy recipes, gRPC/WASM transport glue, multi-tenant shell examples |

If you build a UI on this stack and want it linked or described in the repo,
open an issue or PR — adoption stories help the next person.

## Before you start coding

1. Read [README.md](./README.md) for project intent and any documented setup.
2. Prefer a focused change over a kitchen-sink PR.
3. For non-trivial UX or architecture shifts, open an issue first so we can
   align.
4. Match existing style in the area you touch; avoid drive-by reformatting.
5. Keep the UI a **consumer** of RustERP’s API-first surface where possible —
   prefer shared contracts over ad hoc DTOs.

## Development notes

RustERP-UI-WASM is greenfield and will evolve quickly. As the tree lands:

- Follow build and test steps in the README (and any crate-level docs).
- Prefer clear presentation boundaries; leave domain authority on the server.
- Keep user-facing paths **cognitively efficient** — fewer clicks and less
  ceremony when the business task is simple.
- When you add dependencies, ensure license compatibility with Apache-2.0
  redistribution and update [NOTICE](./NOTICE) if third-party notices must
  be retained at the product level.

## Pull requests

1. Fork (if needed) and branch from `main`.
2. Make the change; keep commits reviewable when practical.
3. Sign off commits (`git commit -s`).
4. Run any available tests or smoke checks for the area you touched.
5. Open a PR that states **what** changed and **why**.
6. Link related issues.

We aim for timely, respectful review. Small, well-described PRs land faster.

## Issues and security

- **Bugs and ideas:** GitHub Issues on this repository.
- **Security-sensitive reports:** Prefer a private channel to the maintainers
  (see the repository owner / contact on the GitHub org) rather than a
  public issue, when disclosure could put users or businesses at risk.

## Code of conduct expectations

Be respectful and constructive. Assume good intent. Harassment, personal
attacks, or deliberate disruption are not acceptable. Maintainers may close
or refuse contributions that violate that bar.

## Questions?

- **Product / project info:** [RustERP.biz](https://RustERP.biz)
- **Code and contribution questions:** open a GitHub issue with the `question`
  label (or a short discussion-style issue).

Curious experimenters and production adopters are both welcome.

Thanks for helping RustERP-UI-WASM become easier to run, fork, and build on.
