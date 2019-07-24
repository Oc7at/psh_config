use crate::env_getter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Route {
    primary: bool,
    original_url: String,
    restrict_robots: bool,
    http_access: HttpAccess,
    #[serde(rename = "type")]
    type_field: String,
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

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Tls {
    client_certificate_authorities: Vec<String>,
    #[serde(default)]
    client_authentication: Option<String>,
    #[serde(default)]
    min_version: Option<String>,
    #[serde(default)]
    strict_transport_authorities: Option<StrictTransportSecurity>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct StrictTransportSecurity {
    #[serde(default)]
    include_subdomains: Option<bool>,
    #[serde(default)]
    enabled: Option<bool>,
    #[serde(default)]
    preload: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Cache {
    enabled: bool,
    default_ttl: i64,
    headers: Vec<String>,
    cookies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct HttpAccess {
    addresses: Vec<String>,
    basic_auth: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Ssi {
    enabled: bool,
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.original_url, self.type_field)
    }
}

/// Returns a single route definition.
///
/// Note: If no route ID was specified in routes.yaml then it will not be possible
/// to look up a route by ID.
pub fn get_route<'a>(
    routes: &'a HashMap<String, Route>,
    id: &str,
) -> Option<(&'a String, &'a Route)> {
    for (name, route) in routes {
        if route.id == Some(id.to_owned()) {
            return Some((name, route));
        }
    }
    None
}

pub fn get_routes() -> Option<HashMap<String, Route>> {
    let value = env_getter::get_json_from_var("PLATFORM_ROUTES")?;
    let value_map = value.as_object().unwrap();

    let mut routes: HashMap<String, Route> = HashMap::new();
    for (rel_name, route) in value_map {
        let route_elem: Route = serde_json::from_value(route.clone()).unwrap();
        routes.insert(rel_name.to_owned(), route_elem);
    }
    Some(routes)
}
