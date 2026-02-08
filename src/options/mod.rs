use std::{ collections::HashMap };

use serde::{ Deserialize, Serialize };

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug)]
pub enum DhcpOptionType {
    SubnetMask = 1,
    TimeOffset = 2,
    Router = 3,
    TimeServer = 4,
    NameServer = 5,
    DomainNameServer = 6,
    LogServer = 7,
    CookieServer = 8,
    LinePrinterServer = 9,
    ImpressServer = 10,
    ResourceLocationServer = 11,
    HostName = 12,
    BootFileSize = 13,
    MeritDumpFile = 14,
    DomainName = 15,
    InterfaceMtuOption = 26,
    BroadcastAddress = 28,
    VendorSpecificInformation = 43,
    IpAddressLeaseTime = 51,
    DhcpMessageType(DhcpMessageType) = 53,
    ServerIdentifier = 54,
    ParameterRequestList = 55,
    RenewalTimeValue = 58,
    RebindingTimeValue = 59,
    ClientIdentifier = 61,
    // TODO: Implement all options listed in rfc2132
}

impl DhcpOptionType {
    fn parse_dhcp_option_data(code: u8, data: &[u8]) -> Option<Self> {
        Some(match code {
            53 => DhcpOptionType::DhcpMessageType(DhcpMessageType::from_code(data[0]).unwrap()),
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
        for j in 0..len {
            data.push(buf[i + 1 + j]);
        }

        let data = DhcpOptionType::parse_dhcp_option_data(code, &data);

        options.insert(code, DhcpV4Option { data });

        i += len + 2;
    }

    options
}
