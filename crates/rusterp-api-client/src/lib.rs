//! RustERP core API client boundary for the reference UI.
//!
//! **All targets:** slozhn gRPC-over-WebSocket (Macaron pattern) for Health +
//! ListParties. Background work uses `spawn_local_fut`; results land in
//! `SharedResult` slots polled by the egui loop.
//!
//! No domain logic or storage — display mapping and transport only.
//!
//! Protobuf contracts are vendored under repo `proto/` (from RustERP core).

mod config;
mod conn;
mod party;
mod refresh;
mod status;

pub mod proto {
    pub mod party {
        pub mod v1 {
            tonic::include_proto!("rusterp.party.v1");
        }
    }
    pub mod platform {
        pub mod v1 {
            tonic::include_proto!("rusterp.platform.v1");
        }
    }
}

pub use config::{
    default_rpc_url, live_grpc_supported, live_grpc_unavailable_reason, normalize_endpoint,
    normalize_rpc_url, resolve_endpoint, resolve_rpc_url, DEFAULT_ENDPOINT, DEFAULT_RPC_URL,
    ENDPOINT_ENV, RPC_URL_ENV,
};
pub use conn::{shared_result, spawn_local_fut, ConnState, Connection, SharedResult};
pub use party::{party_role_label, party_row_from_parts, PartyRow};
pub use refresh::{refresh, RefreshSnapshot};
pub use status::ConnectionStatus;
