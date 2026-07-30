//! Inventory helpers over slozhn (gated by core.modules on the server).

use crate::conn::Connection;
use crate::proto::inventory::v1::inventory_service_client::InventoryServiceClient;
use crate::proto::inventory::v1::{
    CreateStockMoveRequest, CreateWarehouseRequest, ListStockLevelsRequest, ListStockMovesRequest,
    ListWarehousesRequest,
};

#[derive(Debug, Clone)]
pub struct WarehouseRow {
    pub id: String,
    pub code: String,
    pub name: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct StockLevelRow {
    pub id: String,
    pub warehouse_id: String,
    pub product_id: String,
    pub qty_on_hand: String,
    pub qty_reserved: String,
}

#[derive(Debug, Clone)]
pub struct StockMoveRow {
    pub id: String,
    pub product_id: String,
    pub qty: String,
    pub state: String,
}

pub async fn list_warehouses(conn: &mut Connection) -> Result<Vec<WarehouseRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = InventoryServiceClient::new(channel);
    let resp = client
        .list_warehouses(ListWarehousesRequest {})
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .warehouses
        .into_iter()
        .map(|w| WarehouseRow {
            id: w.id,
            code: w.code,
            name: w.name,
            active: w.active,
        })
        .collect())
}

pub async fn create_warehouse(
    conn: &mut Connection,
    code: String,
    name: String,
) -> Result<WarehouseRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = InventoryServiceClient::new(channel);
    let w = client
        .create_warehouse(CreateWarehouseRequest { code, name })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(WarehouseRow {
        id: w.id,
        code: w.code,
        name: w.name,
        active: w.active,
    })
}

pub async fn update_warehouse(
    conn: &mut Connection,
    id: String,
    code: String,
    name: String,
    active: bool,
) -> Result<WarehouseRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = InventoryServiceClient::new(channel);
    use crate::proto::inventory::v1::UpdateWarehouseRequest;
    let w = client
        .update_warehouse(UpdateWarehouseRequest {
            id,
            code: Some(code),
            name: Some(name),
            active: Some(active),
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(WarehouseRow {
        id: w.id,
        code: w.code,
        name: w.name,
        active: w.active,
    })
}

pub async fn list_stock_levels(conn: &mut Connection) -> Result<Vec<StockLevelRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = InventoryServiceClient::new(channel);
    let resp = client
        .list_stock_levels(ListStockLevelsRequest {
            warehouse_id: None,
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .levels
        .into_iter()
        .map(|l| StockLevelRow {
            id: l.id,
            warehouse_id: l.warehouse_id,
            product_id: l.product_id,
            qty_on_hand: l.qty_on_hand,
            qty_reserved: l.qty_reserved,
        })
        .collect())
}

pub async fn list_stock_moves(conn: &mut Connection) -> Result<Vec<StockMoveRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = InventoryServiceClient::new(channel);
    let resp = client
        .list_stock_moves(ListStockMovesRequest {})
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .moves
        .into_iter()
        .map(|m| StockMoveRow {
            id: m.id,
            product_id: m.product_id,
            qty: m.qty,
            state: m.state,
        })
        .collect())
}

pub async fn create_stock_move(
    conn: &mut Connection,
    product_id: String,
    qty: String,
    to_warehouse_id: String,
) -> Result<StockMoveRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = InventoryServiceClient::new(channel);
    let m = client
        .create_stock_move(CreateStockMoveRequest {
            product_id,
            qty,
            from_warehouse_id: None,
            to_warehouse_id: Some(to_warehouse_id),
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(StockMoveRow {
        id: m.id,
        product_id: m.product_id,
        qty: m.qty,
        state: m.state,
    })
}
