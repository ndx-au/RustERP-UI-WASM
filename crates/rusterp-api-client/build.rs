//! Generate tonic client stubs from vendored RustERP protos (all targets).

use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let proto_root = manifest_dir
        .join("../../proto")
        .canonicalize()
        .map_err(|e| format!("proto/ root missing ({e}); expected vendored contracts"))?;

    let party = proto_root.join("rusterp/party/v1/party.proto");
    let health = proto_root.join("rusterp/platform/v1/health.proto");

    println!("cargo:rerun-if-changed={}", party.display());
    println!("cargo:rerun-if-changed={}", health.display());

    tonic_prost_build::configure()
        .build_server(false)
        .build_client(true)
        .build_transport(false)
        .compile_protos(&[party, health], &[proto_root])?;

    Ok(())
}
