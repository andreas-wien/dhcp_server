use std::{ collections::HashMap, net::Ipv4Addr };

use serde::{ Deserialize, Serialize };

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
pub enum DhcpOptionType {
    SubnetMask(Ipv4Addr) = 1,
    TimeOffset = 2,
    Router(Ipv4Addr) = 3,
    TimeServer = 4,
    NameServer = 5,
    DomainNameServer(Vec<Ipv4Addr>) = 6,
    LogServer = 7,
    CookieServer = 8,
    LinePrinterServer = 9,
    ImpressServer = 10,
    ResourceLocationServer = 11,
    HostName(String) = 12,
    BootFileSize = 13,
    MeritDumpFile = 14,
    DomainName(String) = 15,
    InterfaceMtuOption(u16) = 26,
    BroadcastAddress(Ipv4Addr) = 28,
    VendorSpecificInformation = 43,
    RequestedIpAddress(Ipv4Addr) = 50,
    IpAddressLeaseTime(u32) = 51,
    DhcpMessageType(DhcpMessageType) = 53,
    ServerIdentifier = 54,
    ParameterRequestList(Vec<u8>) = 55,
    MaximumDhcpMessageSize(u16) = 57,
    RenewalTimeValue(u32) = 58,
    RebindingTimeValue(u32) = 59,
    VendorClassIdentifier(String) = 60,
    ClientIdentifier(Vec<u8>) = 61,
    // TODO: Implement all options listed in rfc2132
}

impl DhcpOptionType {
    fn parse_dhcp_option_data(code: u8, data: &[u8]) -> Option<Self> {
        Some(match code {
            1 => DhcpOptionType::SubnetMask(Ipv4Addr::new(data[0], data[1], data[2], data[3])),
            3 => DhcpOptionType::Router(Ipv4Addr::new(data[0], data[1], data[2], data[3])),
            6 => {
                let mut ips = vec![];
                for i in (0..data.len()).step_by(4) {
                    ips.push(Ipv4Addr::new(data[i], data[i + 1], data[i + 2], data[i + 3]));
                }
                DhcpOptionType::DomainNameServer(ips)
            }
            12 => DhcpOptionType::HostName(String::from_utf8_lossy(data).to_string()),
            15 => DhcpOptionType::DomainName(String::from_utf8_lossy(data).to_string()),
            26 =>
                DhcpOptionType::InterfaceMtuOption(
                    u16::from_be_bytes(data[0..2].try_into().unwrap())
                ),
            28 =>
                DhcpOptionType::BroadcastAddress(Ipv4Addr::new(data[0], data[1], data[2], data[3])),
            50 =>
                DhcpOptionType::RequestedIpAddress(
                    Ipv4Addr::new(data[0], data[1], data[2], data[3])
                ),
            51 =>
                DhcpOptionType::IpAddressLeaseTime(
                    u32::from_be_bytes(data[0..4].try_into().unwrap())
                ),
            53 => DhcpOptionType::DhcpMessageType(DhcpMessageType::from_code(data[0]).unwrap()),
            55 => DhcpOptionType::ParameterRequestList(data.to_vec()),
            57 =>
                DhcpOptionType::MaximumDhcpMessageSize(
                    u16::from_be_bytes(data[0..2].try_into().unwrap())
                ),
            58 =>
                DhcpOptionType::RenewalTimeValue(
                    u32::from_be_bytes(data[0..4].try_into().unwrap())
                ),
            59 =>
                DhcpOptionType::RebindingTimeValue(
                    u32::from_be_bytes(data[0..4].try_into().unwrap())
                ),
            60 => DhcpOptionType::VendorClassIdentifier(String::from_utf8_lossy(data).to_string()),
            61 => DhcpOptionType::ClientIdentifier(data.to_vec()),
            _ => {
                return None;
            }
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DhcpMessageType {
    DHCPDISCOVER = 1,
    DHCPOFFER = 2,
    DHCPREQUEST = 3,
    DHCPDECLINE = 4,
    DHCPACK = 5,
    DHCPNAK = 6,
    DHCPRELEASE = 7,
    DHCPINFORM = 8,
}

impl DhcpMessageType {
    fn from_code(code: u8) -> Option<Self> {
        Some(match code {
            1 => DhcpMessageType::DHCPDISCOVER,
            2 => DhcpMessageType::DHCPOFFER,
            3 => DhcpMessageType::DHCPREQUEST,
            4 => DhcpMessageType::DHCPDECLINE,
            5 => DhcpMessageType::DHCPACK,
            6 => DhcpMessageType::DHCPNAK,
            7 => DhcpMessageType::DHCPRELEASE,
            8 => DhcpMessageType::DHCPINFORM,
            _ => {
                return None;
            }
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DhcpV4Option {
    data: Option<DhcpOptionType>,
}

impl DhcpV4Option {
    pub fn data(&self) -> &Option<DhcpOptionType> {
        &self.data
    }
}

pub fn parse_dhcp_options(buf: &[u8]) -> HashMap<u8, DhcpV4Option> {
    let mut options = HashMap::new();

    let mut i = 0;
    loop {
        let code = buf[i];

        if code == 0 {
            i += 1;
            continue;
        }
        if code == 255 {
            break;
        }

        let len = buf[i + 1] as usize;

        let mut data = vec![];
        for j in 1..=len {
            data.push(buf[i + 1 + j]);
        }

        let data = DhcpOptionType::parse_dhcp_option_data(code, &data);

        options.insert(code, DhcpV4Option { data });

        i += len + 2;
    }

    options
}
