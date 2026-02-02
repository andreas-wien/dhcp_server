enum DhcpOptionType {
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
    // TODO: Implement all options listed in rfc2132
}

#[derive(Debug, Clone)]
pub struct DhcpV4Option {
    code: u8,
    data: Vec<u8>, // TODO: Implement parsing function for option data by matching the DhcpOptionType enum
}

pub fn parse_dhcp_options(buf: &[u8]) -> Vec<DhcpV4Option> {
    let mut options = vec![];

    let mut i = 0;
    loop {
        let code = buf[i];

        if code == 0 {
            continue;
        }
        if code == 255 {
            break;
        }

        let len = buf[i + 1] as usize;

        let mut data = vec![];
        for j in 0..len {
            data.push(buf[i + j]);
        }

        options.push(DhcpV4Option { code, data });

        i += len + 2;
    }

    options
}
