//! Honest connection status for the reference UI shell.

/// Connection state exposed to the shell.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConnectionStatus {
    /// No attempt yet (or live transport unavailable on this target).
    #[default]
    NotConnected,
    /// RPC in flight.
    Connecting,
    /// Health (or equivalent) succeeded; list may still be empty.
    Connected,
    /// Last attempt failed; see [`ConnectionStatus::error_message`].
    Error { message: String },
}

impl ConnectionStatus {
    /// Short label for status chrome.
    pub fn as_str(&self) -> &str {
        match self {
            Self::NotConnected => "not connected",
            Self::Connecting => "connecting",
            Self::Connected => "connected",
            Self::Error { .. } => "error",
        }
    }

    /// Error detail when [`ConnectionStatus::Error`]; otherwise `None`.
    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Error { message } => Some(message.as_str()),
            _ => None,
        }
    }

    /// Build an error status with a truncated message (UI-safe).
    pub fn error(message: impl Into<String>) -> Self {
        let mut message = message.into();
        const MAX: usize = 240;
        let ellipsis = "…";
        let budget = MAX.saturating_sub(ellipsis.len());
        if message.len() > budget {
            // Stay on a char boundary so we never split a multi-byte scalar.
            let mut end = budget.min(message.len());
            while end > 0 && !message.is_char_boundary(end) {
                end -= 1;
            }
            message.truncate(end);
            message.push_str(ellipsis);
        }
        Self::Error { message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_cover_all_variants() {
        assert_eq!(ConnectionStatus::NotConnected.as_str(), "not connected");
        assert_eq!(ConnectionStatus::Connecting.as_str(), "connecting");
        assert_eq!(ConnectionStatus::Connected.as_str(), "connected");
        assert_eq!(
            ConnectionStatus::error("boom").as_str(),
            "error"
        );
        assert_eq!(
            ConnectionStatus::error("boom").error_message(),
            Some("boom")
        );
    }

    #[test]
    fn error_message_is_truncated() {
        let long = "x".repeat(400);
        let status = ConnectionStatus::error(long);
        let msg = status.error_message().unwrap();
        assert!(msg.len() <= 240);
        assert!(msg.ends_with('…'));
        assert!(msg.len() < 400);
    }
}
