use moka::future::{Cache, CacheBuilder};
use std::net::Ipv4Addr;
use std::sync::LazyLock;
use std::time::Duration;

pub static DOMAIN_TO_IP_CACHE: LazyLock<Cache<String, Ipv4Addr>> = LazyLock::new(|| {
    CacheBuilder::new(1000)
        .time_to_live(Duration::from_secs(60 * 60))
        .build()
});

pub static IP_TO_DOMAIN_CACHE: LazyLock<Cache<Ipv4Addr, String>> = LazyLock::new(|| {
    CacheBuilder::new(1000)
        .time_to_live(Duration::from_secs(60 * 60))
        .build()
});
