use std::fs::File;
use std::io::BufWriter;
use std::net::UdpSocket;

use crate::messages::{ DhcpV4Message, parse_dhcpv4message };
use crate::scopes::DhcpV4Scope;

const DHCP_SERVER_PORT: u16 = 67;

pub struct DhcpV4Server {
    scopes: Vec<DhcpV4Scope>,
    socket: Option<UdpSocket>,
}

impl DhcpV4Server {
    pub fn new() -> Self {
        Self {
            scopes: vec![],
            socket: None,
        }
    }

    pub fn start_listening(&mut self) {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", DHCP_SERVER_PORT)).unwrap();
        socket.set_broadcast(true).unwrap();
        self.socket = Some(socket);
    }

    pub fn receive_packet(&mut self) -> Option<DhcpPacket> {
        let mut buf = [0u8; 576];
        if let Some(ref socket) = self.socket {
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            let message = parse_dhcpv4message(&buf).unwrap();

            if message.mcookie() != [99, 130, 83, 99] {
                panic!("Not a DHCP message");
            }

            return Some(DhcpPacket { message, bytes_received: amt, from: src });
        }

        return None;
    }

    pub fn scopes(&self) -> &Vec<DhcpV4Scope> {
        &self.scopes
    }

    pub fn add_scope(&mut self, scope: DhcpV4Scope) -> &DhcpV4Scope {
        self.scopes.push(scope);
        self.scopes.last().unwrap()
    }

    pub fn save_scopes(&self) {
        let file = File::create("scopes.json").unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &self.scopes).unwrap();
    }

    pub fn load_scopes(&mut self) {
        let file = File::open("scopes.json");
        match file {
            Ok(file) => {
                let scopes: Vec<DhcpV4Scope> = serde_json::from_reader(file).unwrap();
                self.scopes = scopes;
            }
            Err(_) => {
                self.scopes = vec![];
            }
        }
    }
}

pub struct DhcpPacket {
    message: DhcpV4Message,
    bytes_received: usize,
    from: std::net::SocketAddr,
}

impl DhcpPacket {
    pub fn message(&self) -> &DhcpV4Message {
        &self.message
    }

    pub fn bytes_received(&self) -> usize {
        self.bytes_received
    }

    pub fn from(&self) -> &std::net::SocketAddr {
        &self.from
    }
}
