use tokio::net::UdpSocket;
use tokio::task;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

use crate::clients::DhcpV4Client;
use crate::messages::parse_dhcpv4message;
use crate::options::DhcpV4Option;
use crate::scopes::DhcpV4Scope;

const DHCP_SERVER_PORT: u16 = 67;

pub struct DhcpV4Server {
    scopes: Vec<DhcpV4Scope>,
    clients: Vec<DhcpV4Client>,
    options: Vec<DhcpV4Option>,
    lease_time: u32,
    socket: Option<UdpSocket>,
    token: Arc<CancellationToken>,
}

impl DhcpV4Server {
    pub fn new() -> Self {
        Self {
            scopes: vec![],
            clients: vec![],
            options: vec![],
            lease_time: 0,
            socket: None,
            token: Arc::new(CancellationToken::new()),
        }
    }

    pub async fn start_server(mut self) -> task::JoinHandle<()> {
        let token = self.token.clone();

        task::spawn(async move {
            self.start_listening(token).await;
        })
    }

    async fn start_listening(&mut self, token: Arc<CancellationToken>) {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", DHCP_SERVER_PORT)).await.unwrap();
        socket.set_broadcast(true).unwrap();
        self.socket = Some(socket);

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    break;
                }
                _ = self.receive_packet() => {}
            }
        }

        self.stop_server().await;
    }

    pub async fn stop_server(&self) {
        self.token.cancel();
    }

    async fn receive_packet(&mut self) {
        let mut buf = [0u8; 576];
        if let Some(ref socket) = self.socket {
            let (amt, src) = socket.recv_from(&mut buf).await.unwrap();
            let message = parse_dhcpv4message(&buf).unwrap();
            println!("{:?}", message);
            if message.mcookie() != [99, 130, 83, 99] {
                panic!("Not a DHCP message");
            }
        }
    }
}
