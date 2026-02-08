use std::net::Ipv4Addr;

use serde::{ Deserialize, Serialize };

use crate::options::DhcpV4Option;

#[derive(Serialize, Deserialize, Debug)]
pub struct DhcpV4Client {
    mac_address: [u8; 6],
    ip_address: Ipv4Addr,
    options: Vec<DhcpV4Option>,
}
