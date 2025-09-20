use dns_resolver::DNSResolver;
use std::env;
use std::net::Ipv4Addr;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let resolver = DNSResolver::new("198.41.0.4");

    let args: Vec<String> = env::args().skip(1).collect();
    let domains = if args.is_empty() {
        vec![
            "example.com".to_string(),
            "twitter.com".to_string(),
            "facebook.com".to_string(),
            "google.com".to_string(),
            "recurse.com".to_string(),
            "metafilter.com".to_string(),
        ]
    } else {
        args
    };

    for domain in domains.iter() {
        match resolver.resolve(domain).await {
            Ok(ip) => println!("\nIp for {domain} is {ip}\n"),
            Err(e) => eprintln!("\nFailed to resolve {domain}: {e}\n"),
        }
    }
    let reverse_domain = resolver.reverse_resolve(&Ipv4Addr::new(8, 8, 8, 8)).await;
    println!("\nDomain for ip 8.8.8.8 is {:?}\n", reverse_domain);
}
