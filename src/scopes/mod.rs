use std::{ fmt::Display, net::Ipv4Addr };

use crate::{ clients::DhcpV4Client, options::DhcpV4Option };

#[derive(Clone)]
pub struct DhcpV4Scope {
    name: String,
    subnet_ip: Ipv4Addr,
    subnet_prefix: u8,
    start_ip: Ipv4Addr,
    end_ip: Ipv4Addr,
    lease_time: u32,
    options: Vec<DhcpV4Option>,
    clients: Vec<DhcpV4Client>,
    reserved_ips: Vec<Ipv4Addr>,
}

impl Display for DhcpV4Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}/{}", self.name, self.subnet_ip, self.subnet_prefix)
    }
}

impl DhcpV4Scope {}
