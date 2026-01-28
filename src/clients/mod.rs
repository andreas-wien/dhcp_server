use std::net::Ipv4Addr;

use crate::options::DhcpV4Option;

pub struct DhcpV4Client {
    mac_address: [u8; 6],
    ip_address: Ipv4Addr,
    options: Vec<DhcpV4Option>,
}
