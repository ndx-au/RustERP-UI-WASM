//! Generate tonic client stubs from vendored RustERP protos (all targets).

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let proto_root = manifest_dir
        .join("../../proto")
        .canonicalize()
        .map_err(|e| format!("proto/ root missing ({e}); expected vendored contracts"))?;

    let protos = [
        proto_root.join("rusterp/party/v1/party.proto"),
        proto_root.join("rusterp/platform/v1/health.proto"),
        proto_root.join("rusterp/platform/v1/modules_auth.proto"),
        proto_root.join("rusterp/catalog/v1/catalog.proto"),
        proto_root.join("rusterp/sales/v1/sales.proto"),
        proto_root.join("rusterp/payment/v1/payment.proto"),
        proto_root.join("rusterp/inventory/v1/inventory.proto"),
    ];

    for p in &protos {
        println!("cargo:rerun-if-changed={}", p.display());
    }

    tonic_prost_build::configure()
        .build_server(false)
        .build_client(true)
        .build_transport(false)
        .compile_protos(&protos, &[proto_root])?;

    Ok(())
}
