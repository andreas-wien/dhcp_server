use dhcp_server_lib::server::DhcpV4Server;

fn main() {
    let mut server = DhcpV4Server::new();
    server.start_listening();
    loop {
        server.receive_packet();
    }
}
