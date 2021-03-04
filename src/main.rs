use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod client;
mod client_pool;
mod incoming_message;
mod outgoing_message;
mod payload;
mod server;

use server::Server;

#[tokio::main]
async fn main() {
    env_logger::init();

    let server_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);
    let server = Server::new(server_address);
    server.run().await;
}
