use dns_resolver::DNSResolver;

fn main() {
    let packet = DNSResolver::new("8.8.8.8").lookup("example.com");
    println!("{:#?}", packet);
}
