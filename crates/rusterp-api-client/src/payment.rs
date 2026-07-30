//! Payment / bank account / allocation helpers over slozhn.

use crate::conn::Connection;
use crate::proto::payment::v1::payment_service_client::PaymentServiceClient;
use crate::proto::payment::v1::{
    CreateAllocationRequest, CreateBankAccountRequest, CreatePaymentRequest,
    ListAllocationsRequest, ListBankAccountsRequest, ListPaymentsRequest, PaymentDirection,
};

#[derive(Debug, Clone)]
pub struct BankAccountRow {
    pub id: String,
    pub name: String,
    pub currency: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct PaymentRow {
    pub id: String,
    pub direction: String,
    pub party_id: String,
    pub amount_minor: i64,
    pub currency: String,
    pub reference: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct AllocationRow {
    pub id: String,
    pub payment_id: String,
    pub document_id: String,
    pub amount_minor: i64,
}

fn dir_label(d: i32) -> &'static str {
    match PaymentDirection::try_from(d).unwrap_or(PaymentDirection::Unspecified) {
        PaymentDirection::Inbound => "inbound",
        PaymentDirection::Outbound => "outbound",
        PaymentDirection::Unspecified => "unspecified",
    }
}

pub async fn list_bank_accounts(conn: &mut Connection) -> Result<Vec<BankAccountRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PaymentServiceClient::new(channel);
    let resp = client
        .list_bank_accounts(ListBankAccountsRequest {})
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .accounts
        .into_iter()
        .map(|a| BankAccountRow {
            id: a.id,
            name: a.name,
            currency: a.currency,
            active: a.active,
        })
        .collect())
}

pub async fn create_bank_account(
    conn: &mut Connection,
    name: String,
    currency: String,
) -> Result<BankAccountRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PaymentServiceClient::new(channel);
    let a = client
        .create_bank_account(CreateBankAccountRequest { name, currency })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(BankAccountRow {
        id: a.id,
        name: a.name,
        currency: a.currency,
        active: a.active,
    })
}

pub async fn update_bank_account(
    conn: &mut Connection,
    id: String,
    name: String,
    currency: String,
    active: bool,
) -> Result<BankAccountRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PaymentServiceClient::new(channel);
    use crate::proto::payment::v1::UpdateBankAccountRequest;
    let a = client
        .update_bank_account(UpdateBankAccountRequest {
            id,
            name: Some(name),
            currency: Some(currency),
            active: Some(active),
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(BankAccountRow {
        id: a.id,
        name: a.name,
        currency: a.currency,
        active: a.active,
    })
}

pub async fn list_payments(conn: &mut Connection) -> Result<Vec<PaymentRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PaymentServiceClient::new(channel);
    let resp = client
        .list_payments(ListPaymentsRequest {})
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .payments
        .into_iter()
        .map(|p| PaymentRow {
            id: p.id,
            direction: dir_label(p.direction).into(),
            party_id: p.party_id,
            amount_minor: p.amount_minor,
            currency: p.currency,
            reference: p.reference,
            status: p.status,
        })
        .collect())
}

pub async fn create_payment(
    conn: &mut Connection,
    party_id: String,
    amount_minor: i64,
    currency: String,
    reference: String,
) -> Result<PaymentRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PaymentServiceClient::new(channel);
    let p = client
        .create_payment(CreatePaymentRequest {
            direction: PaymentDirection::Inbound as i32,
            party_id,
            bank_account_id: None,
            amount_minor,
            currency,
            reference,
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(PaymentRow {
        id: p.id,
        direction: dir_label(p.direction).into(),
        party_id: p.party_id,
        amount_minor: p.amount_minor,
        currency: p.currency,
        reference: p.reference,
        status: p.status,
    })
}

pub async fn update_payment(
    conn: &mut Connection,
    id: String,
    reference: String,
) -> Result<PaymentRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PaymentServiceClient::new(channel);
    use crate::proto::payment::v1::UpdatePaymentRequest;
    let p = client
        .update_payment(UpdatePaymentRequest {
            id,
            reference: Some(reference),
            bank_account_id: None,
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(PaymentRow {
        id: p.id,
        direction: dir_label(p.direction).into(),
        party_id: p.party_id,
        amount_minor: p.amount_minor,
        currency: p.currency,
        reference: p.reference,
        status: p.status,
    })
}

pub async fn list_allocations(
    conn: &mut Connection,
    payment_id: String,
) -> Result<Vec<AllocationRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PaymentServiceClient::new(channel);
    let resp = client
        .list_allocations(ListAllocationsRequest { payment_id })
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .allocations
        .into_iter()
        .map(|a| AllocationRow {
            id: a.id,
            payment_id: a.payment_id,
            document_id: a.document_id,
            amount_minor: a.amount_minor,
        })
        .collect())
}

pub async fn create_allocation(
    conn: &mut Connection,
    payment_id: String,
    document_id: String,
    amount_minor: i64,
) -> Result<AllocationRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PaymentServiceClient::new(channel);
    let a = client
        .create_allocation(CreateAllocationRequest {
            payment_id,
            document_id,
            amount_minor,
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(AllocationRow {
        id: a.id,
        payment_id: a.payment_id,
        document_id: a.document_id,
        amount_minor: a.amount_minor,
    })
}
