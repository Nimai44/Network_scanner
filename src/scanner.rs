use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use futures::stream::{self, StreamExt};
use std::net::SocketAddr;

async fn scan_port(ip: &str, port: u16) -> bool {
    let addr = format!("{}:{}", ip, port).parse::<SocketAddr>().unwrap();
    match timeout(Duration::from_secs(1), TcpStream::connect(&addr)).await {
        Ok(Ok(_)) => {
            println!("Port {} is open!", port);
            true
        }
        _ => false,
    }
}

pub async fn scan_ports(ip: &str, start: u16, end: u16) {
    let ports = start..=end;
    let tasks = stream::iter(ports.map(|port| scan_port(ip, port)))
        .buffer_unordered(50) // Control concurrency
        .collect::<Vec<_>>()
        .await;
}
