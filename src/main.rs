pub mod server;

fn main() {
    let mut server = server::server::TcpServer::new();
    server.listen("127.0.0.1", 5555);
}
