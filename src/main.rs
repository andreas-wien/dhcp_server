use dhcp_server_lib::{ options::{ DhcpMessageType, DhcpOptionType }, server::DhcpV4Server };

fn main() {
    let mut server = DhcpV4Server::new();
    server.load_scopes();
    server.start_listening();
    loop {
        let response = server.receive_packet();
        if let Some(packet) = response {
            println!("{:?}", packet.message());
            if packet.message().op() == 1 {
                if
                    let DhcpOptionType::DhcpMessageType(dhcp_message) = packet
                        .message()
                        .options()
                        .get(&53)
                        .unwrap()
                        .data()
                        .as_ref()
                        .unwrap()
                {
                    match dhcp_message {
                        DhcpMessageType::DHCPDISCOVER => println!("DHCPDISCOVER received"),
                        DhcpMessageType::DHCPREQUEST => todo!(),
                        DhcpMessageType::DHCPDECLINE => todo!(),
                        DhcpMessageType::DHCPRELEASE => todo!(),
                        DhcpMessageType::DHCPINFORM => todo!(),
                        | DhcpMessageType::DHCPOFFER
                        | DhcpMessageType::DHCPACK
                        | DhcpMessageType::DHCPNAK =>
                            unreachable!("BOOTP request packets cannot be of these types"),
                    }
                }
            }
        }
    }
}
