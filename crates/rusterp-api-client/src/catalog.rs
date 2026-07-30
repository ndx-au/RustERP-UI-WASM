//! Catalog list/create helpers over slozhn.

use crate::conn::Connection;
use crate::proto::catalog::v1::catalog_service_client::CatalogServiceClient;
use crate::proto::catalog::v1::{
    CreateCategoryRequest, CreateProductRequest, ListCategoriesRequest, ListProductsRequest,
    ProductType,
};

#[derive(Debug, Clone)]
pub struct ProductRow {
    pub id: String,
    pub sku: String,
    pub name: String,
    pub type_label: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct CategoryRow {
    pub id: String,
    pub name: String,
    pub active: bool,
}

fn product_type_label(t: i32) -> &'static str {
    match ProductType::try_from(t).unwrap_or(ProductType::Unspecified) {
        ProductType::Stock => "stock",
        ProductType::Service => "service",
        ProductType::Consumable => "consumable",
        ProductType::Unspecified => "unspecified",
    }
}

pub async fn list_products(conn: &mut Connection) -> Result<Vec<ProductRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = CatalogServiceClient::new(channel);
    let resp = client
        .list_products(ListProductsRequest {})
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .products
        .into_iter()
        .map(|p| ProductRow {
            id: p.id,
            sku: p.sku,
            name: p.name,
            type_label: product_type_label(p.r#type).into(),
            active: p.active,
        })
        .collect())
}

pub async fn create_product(
    conn: &mut Connection,
    sku: String,
    name: String,
) -> Result<ProductRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = CatalogServiceClient::new(channel);
    let p = client
        .create_product(CreateProductRequest {
            sku,
            name,
            r#type: ProductType::Stock as i32,
            category_id: None,
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(ProductRow {
        id: p.id,
        sku: p.sku,
        name: p.name,
        type_label: product_type_label(p.r#type).into(),
        active: p.active,
    })
}

pub async fn list_categories(conn: &mut Connection) -> Result<Vec<CategoryRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = CatalogServiceClient::new(channel);
    let resp = client
        .list_categories(ListCategoriesRequest {})
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .categories
        .into_iter()
        .map(|c| CategoryRow {
            id: c.id,
            name: c.name,
            active: c.active,
        })
        .collect())
}

pub async fn create_category(conn: &mut Connection, name: String) -> Result<CategoryRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = CatalogServiceClient::new(channel);
    let c = client
        .create_category(CreateCategoryRequest {
            name,
            parent_id: None,
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(CategoryRow {
        id: c.id,
        name: c.name,
        active: c.active,
    })
}
