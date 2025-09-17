# DNS Resolver in Rust

This is a simple recursive DNS resolver written in Rust.

It starts from a root DNS server `a.root-servers.net` (`198.41.0.4` by default) and follows the DNS delegation chain until it resolves the requested domain name.

## Build

Clone the repository and build:

```bash
cargo build --release
```

## Run

```bash
cargo run --release
```

## Custom Run

```bash
cargo run google.com github.com rust-lang.org
```
