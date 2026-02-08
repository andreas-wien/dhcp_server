use std::fs::File;
use std::io::BufWriter;
use std::net::UdpSocket;

use crate::messages::parse_dhcpv4message;
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

    pub fn receive_packet(&mut self) {
        let mut buf = [0u8; 576];
        if let Some(ref socket) = self.socket {
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            let message = parse_dhcpv4message(&buf).unwrap();
            println!("{:?}", message);
            if message.mcookie() != [99, 130, 83, 99] {
                panic!("Not a DHCP message");
            }
        }
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
