//! Module toggles and auth user/role list helpers over slozhn.

use crate::conn::Connection;
use crate::proto::platform::v1::auth_service_client::AuthServiceClient;
use crate::proto::platform::v1::module_service_client::ModuleServiceClient;
use crate::proto::platform::v1::{
    CreateUserRequest, ListModulesRequest, ListPermissionsRequest, ListRolesRequest,
    ListUsersRequest, SetModuleEnabledRequest,
};

#[derive(Debug, Clone)]
pub struct ModuleRow {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub always_on: bool,
}

#[derive(Debug, Clone)]
pub struct UserRow {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct RoleRow {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct PermissionRow {
    pub id: String,
    pub resource: String,
    pub action: String,
}

pub async fn list_modules(conn: &mut Connection) -> Result<Vec<ModuleRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = ModuleServiceClient::new(channel);
    let resp = client
        .list_modules(ListModulesRequest {})
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .modules
        .into_iter()
        .map(|m| ModuleRow {
            id: m.id,
            name: m.name,
            enabled: m.enabled,
            always_on: m.always_on,
        })
        .collect())
}

pub async fn set_module_enabled(
    conn: &mut Connection,
    id: String,
    enabled: bool,
) -> Result<ModuleRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = ModuleServiceClient::new(channel);
    let m = client
        .set_module_enabled(SetModuleEnabledRequest { id, enabled })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(ModuleRow {
        id: m.id,
        name: m.name,
        enabled: m.enabled,
        always_on: m.always_on,
    })
}

pub async fn list_users(conn: &mut Connection) -> Result<Vec<UserRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = AuthServiceClient::new(channel);
    let resp = client
        .list_users(ListUsersRequest {})
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .users
        .into_iter()
        .map(|u| UserRow {
            id: u.id,
            login: u.login,
            display_name: u.display_name,
            active: u.active,
        })
        .collect())
}

pub async fn list_roles(conn: &mut Connection) -> Result<Vec<RoleRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = AuthServiceClient::new(channel);
    let resp = client
        .list_roles(ListRolesRequest {})
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .roles
        .into_iter()
        .map(|r| RoleRow {
            id: r.id,
            name: r.name,
            description: r.description,
        })
        .collect())
}

pub async fn list_permissions(conn: &mut Connection) -> Result<Vec<PermissionRow>, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = AuthServiceClient::new(channel);
    let resp = client
        .list_permissions(ListPermissionsRequest {})
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp
        .into_inner()
        .permissions
        .into_iter()
        .map(|p| PermissionRow {
            id: p.id,
            resource: p.resource,
            action: p.action,
        })
        .collect())
}

pub async fn create_user(
    conn: &mut Connection,
    login: String,
    display_name: String,
    password: String,
) -> Result<UserRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = AuthServiceClient::new(channel);
    let u = client
        .create_user(CreateUserRequest {
            login,
            display_name,
            password,
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(UserRow {
        id: u.id,
        login: u.login,
        display_name: u.display_name,
        active: u.active,
    })
}

pub async fn update_user(
    conn: &mut Connection,
    id: String,
    display_name: String,
    active: bool,
    password: Option<String>,
) -> Result<UserRow, String> {
    conn.connect();
    let channel = conn
        .channel()
        .ok_or_else(|| "failed to open WebSocket channel".to_string())?;
    let mut client = AuthServiceClient::new(channel);
    use crate::proto::platform::v1::UpdateUserRequest;
    let u = client
        .update_user(UpdateUserRequest {
            id,
            display_name: Some(display_name),
            active: Some(active),
            password,
        })
        .await
        .map_err(|e| e.to_string())?
        .into_inner();
    Ok(UserRow {
        id: u.id,
        login: u.login,
        display_name: u.display_name,
        active: u.active,
    })
}
