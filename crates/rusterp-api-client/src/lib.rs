//! RustERP core API client boundary for the reference UI.
//!
//! **All targets:** slozhn gRPC-over-WebSocket (Macaron pattern) for Health +
//! domain services. Background work uses `spawn_local_fut`; results land in
//! `SharedResult` slots polled by the egui loop.
//!
//! No domain logic or storage — display mapping and transport only.
//!
//! Protobuf contracts are vendored under repo `proto/` (from RustERP core).

mod catalog;
mod config;
mod conn;
mod inventory;
mod party;
mod payment;
mod platform_api;
mod refresh;
mod sales;
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
    pub mod catalog {
        pub mod v1 {
            tonic::include_proto!("rusterp.catalog.v1");
        }
    }
    pub mod sales {
        pub mod v1 {
            tonic::include_proto!("rusterp.sales.v1");
        }
    }
    pub mod payment {
        pub mod v1 {
            tonic::include_proto!("rusterp.payment.v1");
        }
    }
    pub mod inventory {
        pub mod v1 {
            tonic::include_proto!("rusterp.inventory.v1");
        }
    }
}

pub use catalog::{
    create_category, create_product, list_categories, list_products, update_category,
    update_product, CategoryRow, ProductRow,
};
pub use config::{
    default_rpc_url, live_grpc_supported, live_grpc_unavailable_reason, normalize_endpoint,
    normalize_rpc_url, resolve_endpoint, resolve_rpc_url, DEFAULT_ENDPOINT, DEFAULT_RPC_URL,
    ENDPOINT_ENV, RPC_URL_ENV,
};
pub use conn::{shared_result, spawn_local_fut, ConnState, Connection, SharedResult};
pub use inventory::{
    create_stock_move, create_warehouse, list_stock_levels, list_stock_moves, list_warehouses,
    update_warehouse, StockLevelRow, StockMoveRow, WarehouseRow,
};
pub use party::{party_role_label, party_row_from_parts, PartyRow};
pub use payment::{
    create_allocation, create_bank_account, create_payment, list_allocations, list_bank_accounts,
    list_payments, update_bank_account, update_payment, AllocationRow, BankAccountRow, PaymentRow,
};
pub use platform_api::{
    create_user, list_modules, list_permissions, list_roles, list_users, set_module_enabled,
    update_user, ModuleRow, PermissionRow, RoleRow, UserRow,
};
pub use refresh::{
    add_address, add_contact, create_party, list_addresses, list_contacts, refresh, update_address,
    update_contact, update_party, AddressRow, ContactRow, RefreshSnapshot,
};
pub use sales::{
    create_sales_document, list_sales_documents, set_sales_document_status, update_sales_document,
    DocumentKind, DocumentStatus, SalesDocRow,
};
pub use status::ConnectionStatus;
pub use crate::proto::party::v1::{AddressKind, PartyRole};
