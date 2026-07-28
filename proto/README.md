# Vendored RustERP protobuf contracts

These `.proto` files are **copies** of the public contracts from
[RustERP](https://github.com/ndx-video/RustERP) (`proto/`), vendored so this
consumer repo can generate a **client-only** tonic stub without a path
dependency on a local core checkout.

| File | Source (core) |
|------|----------------|
| `rusterp/party/v1/party.proto` | `proto/rusterp/party/v1/party.proto` |
| `rusterp/platform/v1/health.proto` | `proto/rusterp/platform/v1/health.proto` |

- License: Apache-2.0 (same as RustERP / this repo).
- Do **not** edit contracts here to invent APIs — update from core when the
  upstream contract changes.
- Codegen: `crates/rusterp-api-client` `build.rs` (native targets only).

Copyright on the original proto text: NDX Pty Ltd and contributors (see file headers).
