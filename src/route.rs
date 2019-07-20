use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    primary: bool,
    original_url: String,
    restrict_robots: bool,
    http_access: HttpAccess,
    #[serde(default)]
    #[serde(rename = "type")]
    type_field: Option<String>,
    #[serde(default)]
    tls: Option<Tls>,
    #[serde(default)]
    upstream: Option<String>,
    #[serde(default)]
    attributes: Option<HashMap<String, String>>,
    #[serde(default)]
    cache: Option<Cache>,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    ssi: Option<Ssi>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Tls {
    client_certificate_authorities: Vec<String>,
    #[serde(default)]
    client_authentication: Option<String>,
    #[serde(default)]
    min_version: Option<String>,
    #[serde(default)]
    strict_transport_authorities: Option<StrictTransportSecurity>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StrictTransportSecurity {
    #[serde(default)]
    include_subdomains: Option<bool>,
    #[serde(default)]
    enabled: Option<bool>,
    #[serde(default)]
    preload: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cache {
    enabled: bool,
    default_ttl: i64,
    headers: Vec<String>,
    cookies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HttpAccess {
    addresses: Vec<String>,
    basic_auth: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Ssi {
    enabled: bool,
}
