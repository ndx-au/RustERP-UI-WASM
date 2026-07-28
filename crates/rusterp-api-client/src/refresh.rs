//! Unary refresh: Health + ListParties over slozhn.

use crate::conn::Connection;
use crate::party::{party_row_from_parts, PartyRow};
use crate::proto::party::v1::party_service_client::PartyServiceClient;
use crate::proto::party::v1::ListPartiesRequest;
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
pub async fn refresh(conn: &mut Connection) -> RefreshSnapshot {
    conn.connect();
    let Some(channel) = conn.channel() else {
        return RefreshSnapshot::err("slozhn channel unavailable");
    };

    let health = match check_health(channel.clone()).await {
        Ok(s) => s,
        Err(e) => return RefreshSnapshot::err(format!("health failed: {e}")),
    };

    match list_parties(channel).await {
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

async fn list_parties(channel: slozhn::client::Channel) -> Result<Vec<PartyRow>, String> {
    let mut client = PartyServiceClient::new(channel);
    let resp = client
        .list_parties(ListPartiesRequest {})
        .await
        .map_err(|e| e.to_string())?;
    let parties = resp
        .into_inner()
        .parties
        .into_iter()
        .map(|p| party_row_from_parts(p.id, p.display_name, &p.roles))
        .collect();
    Ok(parties)
}
