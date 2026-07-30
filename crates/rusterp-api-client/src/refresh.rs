//! Unary refresh and party mutations over slozhn.

use crate::conn::Connection;
use crate::party::{party_row_from_parts_active, PartyRow};
use crate::proto::party::v1::party_service_client::PartyServiceClient;
use crate::proto::party::v1::{
    AddAddressRequest, AddContactRequest, AddressKind, CreatePartyRequest, ListAddressesRequest,
    ListContactsRequest, ListPartiesRequest, PartyRole,
};
use crate::proto::platform::v1::health_service_client::HealthServiceClient;
use crate::proto::platform::v1::HealthCheckRequest;
use crate::status::ConnectionStatus;

/// Result of a single refresh attempt (status + parties; never invented rows).
#[derive(Debug, Clone)]
pub struct RefreshSnapshot {
    pub status: ConnectionStatus,
    pub parties: Vec<PartyRow>,
    /// Health payload `status` field when Connected.
    pub health: Option<String>,
}

impl RefreshSnapshot {
    fn connected(parties: Vec<PartyRow>, health: String) -> Self {
        Self {
            status: ConnectionStatus::Connected,
            parties,
            health: Some(health),
        }
    }

    fn err(message: impl Into<String>) -> Self {
        Self {
            status: ConnectionStatus::error(message),
            parties: Vec::new(),
            health: None,
        }
    }
}

/// Connect via slozhn, `HealthService/Check`, then `PartyService/ListParties`.
pub async fn refresh(conn: &mut Connection, role_filter: Option<PartyRole>) -> RefreshSnapshot {
    conn.connect();
    let Some(channel) = conn.channel() else {
        return RefreshSnapshot::err("failed to open WebSocket channel");
    };

    let health = match check_health(channel.clone()).await {
        Ok(s) => s,
        Err(e) => return RefreshSnapshot::err(format!("Health check failed: {e}")),
    };

    match list_parties(channel, role_filter).await {
        Ok(parties) => RefreshSnapshot::connected(parties, health),
        Err(e) => RefreshSnapshot::err(format!("ListParties failed: {e}")),
    }
}

async fn check_health(channel: slozhn::client::Channel) -> Result<String, String> {
    let mut client = HealthServiceClient::new(channel);
    let resp = client
        .check(HealthCheckRequest {})
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp.into_inner().status)
}

async fn list_parties(
    channel: slozhn::client::Channel,
    role_filter: Option<PartyRole>,
) -> Result<Vec<PartyRow>, String> {
    let mut client = PartyServiceClient::new(channel);
    let resp = client
        .list_parties(ListPartiesRequest {
            role_filter: role_filter.unwrap_or(PartyRole::Unspecified) as i32,
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .parties
        .into_iter()
        .map(|p| party_row_from_parts_active(p.id, p.display_name, &p.roles, p.active))
        .collect())
}

/// Create a party on the core.
pub async fn create_party(
    conn: &mut Connection,
    display_name: String,
    roles: Vec<PartyRole>,
) -> Result<PartyRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PartyServiceClient::new(channel);
    let party = client
        .create_party(CreatePartyRequest {
            display_name,
            roles: roles.into_iter().map(|r| r as i32).collect(),
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(party_row_from_parts_active(
        party.id,
        party.display_name,
        &party.roles,
        party.active,
    ))
}

pub async fn update_party(
    conn: &mut Connection,
    id: String,
    display_name: String,
    roles: Vec<PartyRole>,
    active: bool,
) -> Result<PartyRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PartyServiceClient::new(channel);
    use crate::proto::party::v1::UpdatePartyRequest;
    let party = client
        .update_party(UpdatePartyRequest {
            id,
            display_name: Some(display_name),
            active: Some(active),
            update_roles: true,
            roles: roles.into_iter().map(|r| r as i32).collect(),
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(party_row_from_parts_active(
        party.id,
        party.display_name,
        &party.roles,
        party.active,
    ))
}

#[derive(Debug, Clone)]
pub struct ContactRow {
    pub id: String,
    pub party_id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub active: bool,
}

pub async fn list_contacts(conn: &mut Connection, party_id: String) -> Result<Vec<ContactRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PartyServiceClient::new(channel);
    let resp = client
        .list_contacts(ListContactsRequest { party_id })
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .contacts
        .into_iter()
        .map(|c| ContactRow {
            id: c.id,
            party_id: c.party_id,
            name: c.name,
            email: c.email.unwrap_or_default(),
            phone: c.phone.unwrap_or_default(),
            active: c.active,
        })
        .collect())
}

pub async fn add_contact(
    conn: &mut Connection,
    party_id: String,
    name: String,
    email: String,
    phone: String,
) -> Result<ContactRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PartyServiceClient::new(channel);
    let c = client
        .add_contact(AddContactRequest {
            party_id,
            name,
            email: if email.trim().is_empty() {
                None
            } else {
                Some(email)
            },
            phone: if phone.trim().is_empty() {
                None
            } else {
                Some(phone)
            },
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(ContactRow {
        id: c.id,
        party_id: c.party_id,
        name: c.name,
        email: c.email.unwrap_or_default(),
        phone: c.phone.unwrap_or_default(),
        active: c.active,
    })
}

pub async fn update_contact(
    conn: &mut Connection,
    id: String,
    name: String,
    email: String,
    phone: String,
    active: bool,
) -> Result<ContactRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PartyServiceClient::new(channel);
    use crate::proto::party::v1::UpdateContactRequest;
    let c = client
        .update_contact(UpdateContactRequest {
            id,
            name: Some(name),
            email: Some(email),
            phone: Some(phone),
            active: Some(active),
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(ContactRow {
        id: c.id,
        party_id: c.party_id,
        name: c.name,
        email: c.email.unwrap_or_default(),
        phone: c.phone.unwrap_or_default(),
        active: c.active,
    })
}

#[derive(Debug, Clone)]
pub struct AddressRow {
    pub id: String,
    pub party_id: String,
    pub kind: String,
    pub line1: String,
    pub city: String,
    pub country: String,
    pub active: bool,
}

pub async fn list_addresses(
    conn: &mut Connection,
    party_id: String,
) -> Result<Vec<AddressRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PartyServiceClient::new(channel);
    let resp = client
        .list_addresses(ListAddressesRequest { party_id })
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .addresses
        .into_iter()
        .map(|a| AddressRow {
            id: a.id,
            party_id: a.party_id,
            kind: match AddressKind::try_from(a.kind).unwrap_or(AddressKind::Other) {
                AddressKind::Billing => "billing".into(),
                AddressKind::Shipping => "shipping".into(),
                AddressKind::Other | AddressKind::Unspecified => "other".into(),
            },
            line1: a.line1,
            city: a.city,
            country: a.country,
            active: a.active,
        })
        .collect())
}

pub async fn add_address(
    conn: &mut Connection,
    party_id: String,
    line1: String,
    city: String,
    country: String,
) -> Result<AddressRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PartyServiceClient::new(channel);
    let a = client
        .add_address(AddAddressRequest {
            party_id,
            kind: AddressKind::Billing as i32,
            line1,
            line2: None,
            city,
            state_region: None,
            postal_code: None,
            country: if country.trim().is_empty() {
                "AU".into()
            } else {
                country
            },
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(AddressRow {
        id: a.id,
        party_id: a.party_id,
        kind: "billing".into(),
        line1: a.line1,
        city: a.city,
        country: a.country,
        active: a.active,
    })
}

pub async fn update_address(
    conn: &mut Connection,
    id: String,
    line1: String,
    city: String,
    country: String,
    active: bool,
) -> Result<AddressRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = PartyServiceClient::new(channel);
    use crate::proto::party::v1::UpdateAddressRequest;
    let a = client
        .update_address(UpdateAddressRequest {
            id,
            kind: None,
            line1: Some(line1),
            line2: None,
            city: Some(city),
            state_region: None,
            postal_code: None,
            country: Some(if country.trim().is_empty() {
                "AU".into()
            } else {
                country
            }),
            active: Some(active),
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(AddressRow {
        id: a.id,
        party_id: a.party_id,
        kind: match AddressKind::try_from(a.kind).unwrap_or(AddressKind::Other) {
            AddressKind::Billing => "billing".into(),
            AddressKind::Shipping => "shipping".into(),
            AddressKind::Other | AddressKind::Unspecified => "other".into(),
        },
        line1: a.line1,
        city: a.city,
        country: a.country,
        active: a.active,
    })
}
