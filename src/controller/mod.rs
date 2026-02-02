use std::fmt::Display;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio::sync::Mutex;

use crate::server::DhcpV4Server;

pub struct ServerAlreadyStartedError(String);

impl Display for ServerAlreadyStartedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct ServerController {
    server: Option<DhcpV4Server>,
    running_flag: Option<Arc<Mutex<bool>>>,
    handle: Option<JoinHandle<()>>,
}

impl ServerController {
    pub fn new() -> Self {
        Self {
            server: Some(DhcpV4Server::new()),
            running_flag: None,
            handle: None,
        }
    }

    pub async fn start(&mut self) -> Result<(), ServerAlreadyStartedError> {
        let server = self.server
            .take()
            .ok_or(ServerAlreadyStartedError("Server already started".to_string()))?;

        let handle = server.start_server().await;
        self.handle = Some(handle);

        Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(handle) = &self.handle {
            handle.abort();
        }
        self.server = Some(DhcpV4Server::new());
    }

    pub fn is_running(&self) -> bool {
        self.running_flag
            .as_ref()
            .map(|r| tokio::runtime::Handle::current().block_on(async { *r.lock().await }))
            .unwrap_or(false)
    }
}
