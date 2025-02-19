# Network Scanner in Rust

A simple async network scanner built using Rust, Tokio, and Reqwest.

## Features
- Async TCP port scanning
- Parallel execution with Tokio
- Fetch HTTP responses from open web ports
- Command-line arguments for flexibility

## Usage
```sh
cargo run -- --ip 192.168.1.1 --start-port 20 --end-port 100
