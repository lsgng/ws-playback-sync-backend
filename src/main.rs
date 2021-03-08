use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod client;
mod client_pool;
mod incoming_message;
mod outgoing_message;
mod payload;
mod server;

use server::Server;

const DEFAULT_PORT: u16 = 8000;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let port = match args.get(1) {
        Some(port) => port.parse::<u16>().unwrap_or(DEFAULT_PORT),
        None => DEFAULT_PORT,
    };

    env_logger::init();

    let server_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    let server = Server::new(server_address);
    server.run().await;
}
