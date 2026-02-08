use std::{ fmt::Display, net::Ipv4Addr };

use serde::{ Deserialize, Serialize };

use crate::{ clients::DhcpV4Client, options::DhcpV4Option };

#[derive(Serialize, Deserialize, Debug)]
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

impl DhcpV4Scope {
    pub fn new(name: &str, subnet_ip: Ipv4Addr, subnet_prefix: u8) -> Self {
        let start_ip = {
            let mut ip = subnet_ip.octets();
            ip[3] += 1;
            Ipv4Addr::from(ip)
        };
        let end_ip = {
            let mut ip = subnet_ip.octets();
            let usable_adresses = (2u32).pow((32 - subnet_prefix) as u32) - 2;
            let bytes = usable_adresses.to_be_bytes();
            ip[0] += bytes[0];
            ip[1] += bytes[1];
            ip[2] += bytes[2];
            ip[3] += bytes[3];
            Ipv4Addr::from(ip)
        };
        Self {
            name: name.to_string(),
            subnet_ip,
            subnet_prefix,
            start_ip,
            end_ip,
            lease_time: 86400,
            options: vec![],
            clients: vec![],
            reserved_ips: vec![],
        }
    }
}
