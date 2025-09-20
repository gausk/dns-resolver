# DNS Resolver in Rust

A comprehensive recursive DNS resolver written in Rust that provides both a library interface and an HTTP REST API server for DNS resolution.

## Features

- **Recursive DNS Resolution**: Starts from root DNS servers and follows the delegation chain to resolve domain names
- **Forward DNS Lookup**: Resolve domain names to IPv4 addresses
- **Reverse DNS Lookup**: Resolve IPv4 addresses back to domain names (PTR records)
- **HTTP REST API**: Web server with endpoints for DNS resolution
- **Caching**: Built-in memory cache with 1-hour TTL to improve performance
- **Multiple Record Types**: Supports A, NS, CNAME, PTR, and other DNS record types
- **Async/Await**: Fully asynchronous implementation using Tokio
- **Timeout Handling**: 5-second timeout for DNS queries to prevent hanging
- **Comprehensive Testing**: Unit tests for core DNS functionality

## Architecture

The project consists of several modules:

- `lib.rs` - Core DNS resolver logic with packet parsing
- `server.rs` - HTTP REST API endpoints
- `cache.rs` - In-memory caching using Moka
- `examples/resolve.rs` - Command-line example usage

## Installation

Clone the repository and build:

```bash
git clone <repository-url>
cd dns-resolver
cargo build --release
```

## Usage

### HTTP Server

Start the DNS resolver HTTP server:

```bash
cargo run --release
```

The server runs on `http://localhost:3000` and provides the following endpoints:

#### Forward DNS Resolution
```bash
# Resolve domain to IP
curl "http://localhost:3000/resolve?domain=google.com"
# Response: {"ip":"172.217.14.110"}
```

#### Reverse DNS Resolution
```bash
# Resolve IP to domain
curl "http://localhost:3000/reverse_resolve?ip=8.8.8.8"
# Response: {"domain":"dns.google"}
```

### Command Line Example

Run the example with default domains:

```bash
cargo run --example resolve
```

Or specify custom domains:

```bash
cargo run --example resolve -- google.com github.com rust-lang.org
```

### Library Usage

Use the DNS resolver as a library in your Rust code:

```rust
use dns_resolver::DNSResolver;
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create resolver with default root server (198.41.0.4)
    let resolver = DNSResolver::default();
    
    // Forward DNS lookup
    let ip = resolver.resolve("google.com").await?;
    println!("google.com resolves to: {}", ip);
    
    // Reverse DNS lookup
    let domain = resolver.reverse_resolve(&Ipv4Addr::new(8, 8, 8, 8)).await?;
    println!("8.8.8.8 resolves to: {}", domain);
    
    Ok(())
}
```

## Configuration

### Custom Root Server

You can specify a custom root DNS server:

```rust
let resolver = DNSResolver::new("198.41.0.4"); // a.root-servers.net
```

### Cache Settings

The cache is configured with:
- **Capacity**: 1000 entries
- **TTL**: 1 hour
- **Separate caches**: One for domain→IP, one for IP→domain

## Dependencies

- `tokio` - Async runtime and networking
- `axum` - HTTP web framework
- `moka` - High-performance caching library
- `anyhow` - Error handling
- `serde` - JSON serialization
- `tracing` - Structured logging
- `num_enum` - Enum conversions
- `rand` - Random number generation

## Testing

Run the test suite:

```bash
cargo test
```

## How It Works

1. **Query Construction**: Builds DNS queries according to RFC 1035
2. **Recursive Resolution**: Starts from root servers and follows NS records
3. **Packet Parsing**: Parses DNS responses including compressed names
4. **Caching**: Stores results to avoid repeated queries
5. **Error Handling**: Graceful handling of timeouts and malformed responses

The resolver supports DNS name compression and handles various record types including A, NS, CNAME, and PTR records for comprehensive DNS functionality.
