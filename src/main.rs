mod scanner;
mod http_client;

use clap::Parser;

/// Simple Network Scanner
#[derive(Parser)]
struct Args {
    /// Target IP address
    #[arg(short, long)]
    ip: String,

    /// Start port
    #[arg(short, long, default_value = "1")]
    start_port: u16,

    /// End port
    #[arg(short, long, default_value = "65535")]
    end_port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    println!("Scanning {} from port {} to {}", args.ip, args.start_port, args.end_port);
    scanner::scan_ports(&args.ip, args.start_port, args.end_port).await;

    println!("\nChecking HTTP response from {}", args.ip);
    http_client::fetch_http(&format!("http://{}", args.ip)).await;
}
