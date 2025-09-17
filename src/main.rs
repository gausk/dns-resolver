use dns_resolver::DNSResolver;
use std::env;

fn main() {
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
        match resolver.resolve(domain) {
            Ok(ip) => println!("\nIp for {domain} is {ip}\n"),
            Err(e) => eprintln!("\nFailed to resolve {domain}: {e}\n"),
        }
    }
}
