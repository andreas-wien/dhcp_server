use std::net::Ipv4Addr;

use crate::{ clients::DhcpV4Client, options::DhcpV4Option };

pub struct DhcpV4Scope {
    name: String,
    subnet_ip: Ipv4Addr,
    subnet_prefix: u8,
    gateway_ip: Ipv4Addr,
    dns_ip: Ipv4Addr,
    start_ip: Ipv4Addr,
    end_ip: Ipv4Addr,
    lease_time: u32,
    options: Vec<DhcpV4Option>,
    clients: Vec<DhcpV4Client>,
    reserved_ips: Vec<Ipv4Addr>,
}
