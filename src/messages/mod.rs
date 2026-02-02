use crate::options::{ DhcpV4Option, parse_dhcp_options };

#[derive(Debug, Clone)]
pub struct DhcpMessageParseError(String);

enum DhcpMessageType {
    DISCOVER,
    REQUEST,
    DECLINE,
    RELEASE,
    OFFER,
    ACK,
    RENEW,
    REBIND,
    INFORM,
}

#[derive(Debug)]
pub struct DhcpV4Message {
    op: u8,
    htype: u8,
    hlen: u8,
    hops: u8,
    xid: u32,
    secs: u16,
    flags: u16,
    ciaddr: u32,
    yiaddr: u32,
    siaddr: u32,
    giaddr: u32,
    chaddr: [u8; 16],
    sname: String,
    bname: String,
    mcookie: [u8; 4],
    options: Vec<DhcpV4Option>,
}

impl DhcpV4Message {
    pub fn mcookie(&self) -> &[u8] {
        &self.mcookie
    }
}

pub fn parse_dhcpv4message(buf: &[u8]) -> Result<DhcpV4Message, DhcpMessageParseError> {
    if buf.len() < 240 {
        return Err(DhcpMessageParseError("Too short for dhcp header".to_string()));
    }
    let hlen = buf[2] as usize;
    if hlen > 16 || 28 + hlen > buf.len() {
        return Err(DhcpMessageParseError("Invalid hlen".to_string()));
    }
    Ok(DhcpV4Message {
        op: buf[0],
        htype: buf[1],
        hlen: buf[2],
        hops: buf[3],
        xid: u32::from_be_bytes(buf[4..8].try_into().unwrap()),
        secs: u16::from_be_bytes(buf[8..10].try_into().unwrap()),
        flags: u16::from_be_bytes(buf[10..12].try_into().unwrap()),
        ciaddr: u32::from_be_bytes(buf[12..16].try_into().unwrap()),
        yiaddr: u32::from_be_bytes(buf[16..20].try_into().unwrap()),
        siaddr: u32::from_be_bytes(buf[20..24].try_into().unwrap()),
        giaddr: u32::from_be_bytes(buf[24..28].try_into().unwrap()),
        chaddr: {
            let mut ch = [0u8; 16];
            ch[..hlen].copy_from_slice(&buf[28..28 + hlen]);
            ch
        },
        sname: String::from_utf8_lossy(&buf[47..111])
            .trim_end_matches('\0')
            .to_string(),
        bname: String::from_utf8_lossy(&buf[112..236])
            .trim_end_matches('\0')
            .to_string(),
        mcookie: buf[236..240].try_into().unwrap(),
        options: parse_dhcp_options(&buf[240..]),
    })
}
