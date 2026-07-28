//! Endpoint / RPC URL configuration for the RustERP core.

/// Environment variable for slozhn RPC WebSocket URL.
pub const RPC_URL_ENV: &str = "RUSTERP_RPC_URL";

/// Legacy env for plain tonic TCP (documented for migration; UI default is slozhn).
pub const ENDPOINT_ENV: &str = "RUSTERP_GRPC_ENDPOINT";

/// Default slozhn RPC URL (core HTTP listener + `/rpc`).
pub const DEFAULT_RPC_URL: &str = "ws://127.0.0.1:8123/rpc";

/// Legacy default TCP gRPC URI (grpcurl / API tools on core).
pub const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:50051";

/// Resolve RPC URL: `override_value` → env → [`DEFAULT_RPC_URL`].
///
/// Accepts `ws://` / `wss://` URLs. Legacy `http://host:50051` values are mapped
/// to `ws://host:8123/rpc` for local dual-port setups.
pub fn resolve_rpc_url(override_value: Option<&str>) -> String {
    let raw = override_value
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .or_else(|| {
            std::env::var(RPC_URL_ENV)
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .or_else(|| {
            std::env::var(ENDPOINT_ENV)
                .ok()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| DEFAULT_RPC_URL.to_string());
    normalize_rpc_url(&raw)
}

/// Map legacy HTTP gRPC endpoints to slozhn WS where sensible.
pub fn normalize_rpc_url(raw: &str) -> String {
    let t = raw.trim();
    if t.starts_with("ws://") || t.starts_with("wss://") {
        return t.to_string();
    }
    if t.starts_with("http://") || t.starts_with("https://") {
        // Map http://host:50051 → ws://host:8123/rpc (dual-port convention).
        let without_scheme = t
            .trim_start_matches("https://")
            .trim_start_matches("http://");
        if let Some(host) = without_scheme.split(':').next() {
            let ws_scheme = if t.starts_with("https://") {
                "wss"
            } else {
                "ws"
            };
            return format!("{ws_scheme}://{host}:8123/rpc");
        }
    }
    if !t.contains("://") {
        return format!("ws://{t}/rpc");
    }
    t.to_string()
}

/// Backward-compatible alias used by the UI endpoint field.
pub fn resolve_endpoint(override_value: Option<&str>) -> String {
    resolve_rpc_url(override_value)
}

/// Normalize bare `host:port` to `http://host:port` (legacy tonic helper).
pub fn normalize_endpoint(raw: &str) -> String {
    let t = raw.trim();
    if t.starts_with("http://") || t.starts_with("https://") {
        t.to_string()
    } else {
        format!("http://{t}")
    }
}

/// Page-relative slozhn URL on WASM; env/default on native.
pub fn default_rpc_url() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let loc = window.location();
            let protocol = loc.protocol().unwrap_or_else(|_| "http:".into());
            let host = loc.host().unwrap_or_else(|_| "127.0.0.1:8123".into());
            let ws = if protocol == "https:" { "wss:" } else { "ws:" };
            return format!("{ws}//{host}/rpc");
        }
        DEFAULT_RPC_URL.to_string()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        resolve_rpc_url(None)
    }
}

/// Whether this build can open a live slozhn channel to core.
pub fn live_grpc_supported() -> bool {
    true
}

/// Message when live transport is unavailable (kept for API stability).
pub fn live_grpc_unavailable_reason() -> &'static str {
    "Live RPC unavailable (check core HTTP listener and RPC URL)."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_rpc_url_is_ws() {
        assert_eq!(normalize_rpc_url(DEFAULT_RPC_URL), DEFAULT_RPC_URL);
    }

    #[test]
    fn legacy_http_maps_to_ws_port() {
        assert_eq!(
            normalize_rpc_url("http://127.0.0.1:50051"),
            "ws://127.0.0.1:8123/rpc"
        );
    }

    #[test]
    fn ws_passthrough() {
        assert_eq!(
            normalize_rpc_url("ws://192.0.2.1:9090/rpc"),
            "ws://192.0.2.1:9090/rpc"
        );
    }

    #[test]
    fn override_wins() {
        let url = resolve_rpc_url(Some("ws://192.0.2.1:8080/rpc"));
        assert_eq!(url, "ws://192.0.2.1:8080/rpc");
    }
}
