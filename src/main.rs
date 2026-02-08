use dhcp_server_lib::{scopes::DhcpV4Scope, server::DhcpV4Server};

fn main() {
    let mut server = DhcpV4Server::new();
    server.load_scopes();
    let test_scope = DhcpV4Scope::new("test", std::net::Ipv4Addr::new(10, 0, 0, 0), 24);
    server.add_scope(test_scope);
    server.save_scopes();
    server.start_listening();
    loop {
        server.receive_packet();
    }
}
