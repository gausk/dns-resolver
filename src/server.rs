use crate::DNSResolver;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Dns {
    domain: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct IpAddr {
    ip: Ipv4Addr,
}

pub async fn resolve_dns(
    Query(params): Query<Dns>,
) -> Result<Json<IpAddr>, (StatusCode, Json<String>)> {
    let ip = DNSResolver::default().resolve(params.domain.as_str()).await;
    match ip {
        Ok(ip) => Ok(Json(IpAddr { ip })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
    }
}

pub async fn resolve_ip(
    Query(params): Query<IpAddr>,
) -> Result<Json<Dns>, (StatusCode, Json<String>)> {
    let domain = DNSResolver::default().reverse_resolve(&params.ip).await;
    match domain {
        Ok(domain) => Ok(Json(Dns { domain })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string()))),
    }
}
