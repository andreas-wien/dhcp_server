use std::net::UdpSocket;

use crate::{
    clients::DhcpV4Client,
    messages::parse_dhcpv4message,
    options::DhcpV4Option,
    scopes::DhcpV4Scope,
};

const DHCP_SERVER_PORT: u16 = 67;

pub struct DhcpV4Server {
    scopes: Vec<DhcpV4Scope>,
    clients: Vec<DhcpV4Client>,
    options: Vec<DhcpV4Option>,
    lease_time: u32,
    socket: Option<UdpSocket>,
}

impl DhcpV4Server {
    pub fn new() -> Self {
        DhcpV4Server {
            scopes: vec![],
            clients: vec![],
            options: vec![],
            lease_time: 0,
            socket: None,
        }
    }

    fn get_scopes(&self) -> &Vec<DhcpV4Scope> {
        &self.scopes
    }

    pub fn start_server(&mut self) {
        self.socket = Some(UdpSocket::bind(format!("0.0.0.0:{}", DHCP_SERVER_PORT)).unwrap());
        self.socket.as_ref().unwrap().set_broadcast(true).unwrap();

        loop {
            self.receive_packet();
            // TODO: Implement dhcp workflow as described in rfc2131
        }
    }

    fn receive_packet(&mut self) {
        let mut buf = [0; 576];
        if let Some(ref socket) = self.socket {
            let (amt, src) = socket.recv_from(&mut buf).unwrap();
            //println!("Received {} bytes from {}: {:?}", amt, src, &buf[..amt]);
            let message = parse_dhcpv4message(&buf).unwrap();
            println!("{:?}", message);
            if message.get_mcookie() != [99, 130, 83, 99] {
                panic!("Not a dhcp message")
            }
        }
    }
}
