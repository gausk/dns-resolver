# DNS Resolver in Rust

A comprehensive recursive DNS resolver written in Rust that provides both a library interface and an HTTP REST API server for DNS resolution.

## Live Demo

Try the DNS resolver online:
- **Primary**: [https://dns.gauravkumarsingh.live/](https://dns.gauravkumarsingh.live/)
- **Mirror**: [https://dns-resolver.onrender.com/](https://dns-resolver.onrender.com/)

## Features

- **Recursive DNS Resolution**: Starts from root DNS servers and follows the delegation chain to resolve domain names
- **Forward DNS Lookup**: Resolve domain names to IPv4 addresses
- **Reverse DNS Lookup**: Resolve IPv4 addresses back to domain names (PTR records)
- **Web Interface**: Modern, responsive UI for easy DNS resolution
- **HTTP REST API**: Web server with endpoints for DNS resolution
- **Caching**: Built-in memory cache with 1-hour TTL to improve performance
- **Multiple Record Types**: Supports A, NS, CNAME, PTR, and other DNS record types
- **Timeout Handling**: 5-second timeout for DNS queries to prevent hanging

## Installation

Clone the repository and build:

```bash
git clone https://github.com/gausk/dns-resolver.git
cd dns-resolver
cargo build --release
```

## Usage

### HTTP Server

Start the DNS resolver HTTP server locally:

```bash
cargo run --release
```

The server runs on `http://localhost:3000` and provides:
- **Web Interface**: UI at `http://localhost:3000/`
- **REST API**: Programmatic access via endpoints below

#### API Endpoints

#### Forward DNS Resolution
```bash
curl "http://localhost:3000/resolve?domain=google.com"
# Response: {"ip":"172.217.14.110"}
```

#### Reverse DNS Resolution
```bash
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

## Testing

Run the test suite:

```bash
cargo test
```