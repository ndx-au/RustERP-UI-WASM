//! Connection helpers: slozhn channel + platform-correct async spawn.

use std::sync::Arc;

use slozhn::client::Channel;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConnState {
    Idle,
    Ready,
}

pub struct Connection {
    url: String,
    channel: Option<Channel>,
    state: ConnState,
}

impl Connection {
    pub fn new(url: String) -> Self {
        Self {
            url,
            channel: None,
            state: ConnState::Idle,
        }
    }

    pub fn connect(&mut self) {
        let ch = slozhn::client::builder(self.url.clone()).resume().build();
        self.channel = Some(ch);
        self.state = ConnState::Ready;
    }

    pub fn channel(&self) -> Option<Channel> {
        self.channel.clone()
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
        self.channel = None;
        self.state = ConnState::Idle;
    }

    pub fn disconnect(&mut self) {
        self.channel = None;
        self.state = ConnState::Idle;
    }

    pub fn state(&self) -> ConnState {
        self.state
    }
}

/// Shared result mailbox for fire-and-forget RPCs into the UI thread.
pub type SharedResult<T> = Arc<std::sync::Mutex<Option<Result<T, String>>>>;

pub fn shared_result<T>() -> SharedResult<T> {
    Arc::new(std::sync::Mutex::new(None))
}

/// Spawn a future on the appropriate runtime (Macaron pattern).
#[cfg(target_arch = "wasm32")]
pub fn spawn_local_fut(fut: impl std::future::Future<Output = ()> + 'static) {
    wasm_bindgen_futures::spawn_local(fut);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_local_fut(fut: impl std::future::Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}
