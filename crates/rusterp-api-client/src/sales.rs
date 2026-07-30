//! Sales document list/create helpers over slozhn.

use crate::conn::Connection;
use crate::proto::sales::v1::sales_service_client::SalesServiceClient;
use crate::proto::sales::v1::{
    CreateSalesDocumentRequest, ListSalesDocumentsRequest, SetSalesDocumentStatusRequest,
};
pub use crate::proto::sales::v1::{DocumentKind, DocumentStatus};

#[derive(Debug, Clone)]
pub struct SalesDocRow {
    pub id: String,
    pub kind: String,
    pub status: String,
    pub number: String,
    pub party_id: String,
    pub total_minor: i64,
    pub notes: String,
}

fn kind_label(k: i32) -> &'static str {
    match DocumentKind::try_from(k).unwrap_or(DocumentKind::Unspecified) {
        DocumentKind::Quote => "quote",
        DocumentKind::Order => "order",
        DocumentKind::Invoice => "invoice",
        DocumentKind::CreditNote => "credit_note",
        DocumentKind::Unspecified => "unspecified",
    }
}

fn status_label(s: i32) -> &'static str {
    match DocumentStatus::try_from(s).unwrap_or(DocumentStatus::Unspecified) {
        DocumentStatus::Draft => "draft",
        DocumentStatus::Confirmed => "confirmed",
        DocumentStatus::Posted => "posted",
        DocumentStatus::Cancelled => "cancelled",
        DocumentStatus::Unspecified => "unspecified",
    }
}

pub async fn list_sales_documents(
    conn: &mut Connection,
    kind: DocumentKind,
) -> Result<Vec<SalesDocRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = SalesServiceClient::new(channel);
    let resp = client
        .list_sales_documents(ListSalesDocumentsRequest {
            kind_filter: kind as i32,
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .documents
        .into_iter()
        .map(|d| SalesDocRow {
            id: d.id,
            kind: kind_label(d.kind).into(),
            status: status_label(d.status).into(),
            number: d.number,
            party_id: d.party_id,
            total_minor: d.total_minor,
            notes: d.notes,
        })
        .collect())
}

pub async fn create_sales_document(
    conn: &mut Connection,
    kind: DocumentKind,
    party_id: String,
    description: String,
    unit_price_minor: i64,
) -> Result<SalesDocRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = SalesServiceClient::new(channel);
    let d = client
        .create_sales_document(CreateSalesDocumentRequest {
            kind: kind as i32,
            party_id,
            description,
            unit_price_minor,
            product_id: None,
            notes: String::new(),
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(SalesDocRow {
        id: d.id,
        kind: kind_label(d.kind).into(),
        status: status_label(d.status).into(),
        number: d.number,
        party_id: d.party_id,
        total_minor: d.total_minor,
        notes: d.notes,
    })
}

pub async fn set_sales_document_status(
    conn: &mut Connection,
    id: String,
    status: DocumentStatus,
) -> Result<SalesDocRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = SalesServiceClient::new(channel);
    let d = client
        .set_sales_document_status(SetSalesDocumentStatusRequest {
            id,
            status: status as i32,
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(SalesDocRow {
        id: d.id,
        kind: kind_label(d.kind).into(),
        status: status_label(d.status).into(),
        number: d.number,
        party_id: d.party_id,
        total_minor: d.total_minor,
        notes: d.notes,
    })
}

pub async fn update_sales_document(
    conn: &mut Connection,
    id: String,
    notes: String,
) -> Result<SalesDocRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = SalesServiceClient::new(channel);
    use crate::proto::sales::v1::UpdateSalesDocumentRequest;
    let d = client
        .update_sales_document(UpdateSalesDocumentRequest {
            id,
            notes: Some(notes),
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(SalesDocRow {
        id: d.id,
        kind: kind_label(d.kind).into(),
        status: status_label(d.status).into(),
        number: d.number,
        party_id: d.party_id,
        total_minor: d.total_minor,
        notes: d.notes,
    })
}
