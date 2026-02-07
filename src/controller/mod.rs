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
    server: Option<Arc<Mutex<DhcpV4Server>>>,
    handle: Option<JoinHandle<()>>,
}

impl ServerController {
    pub fn new() -> Self {
        Self {
            server: None,
            handle: None,
        }
    }

    pub async fn start(&mut self) -> Result<(), ServerAlreadyStartedError> {
        if self.handle.is_some() {
            return Err(ServerAlreadyStartedError("Server already started".to_string()));
        }

        let server = DhcpV4Server::new();
        let (shared, handle) = server.start_server().await;

        self.server = Some(shared);
        self.handle = Some(handle);

        Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(handle) = &self.handle {
            handle.abort();
        }

        if let Some(server) = &self.server {
            server.lock().await.stop_server().await;
        }

        self.server = None;
        self.handle = None;
    }

    pub fn server(&self) -> Option<Arc<Mutex<DhcpV4Server>>> {
        self.server.clone()
    }
}
