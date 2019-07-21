use crate::env_getter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    scheme: String,
    service: String,
    ip: String,
    cluster: String,
    host: String,
    port: i64,
    #[serde(default)]
    rel: Option<String>, // It is not always set on Enterprise
    #[serde(default)]
    fragment: Option<String>,
    #[serde(default)]
    hostname: Option<String>,
    #[serde(default)]
    public: Option<bool>,
    #[serde(default)]
    #[serde(rename = "type")]
    type_field: Option<String>,
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    password: Option<String>,
    #[serde(default)]
    query: Option<Query>,
    #[serde(default)]
    path: Option<String>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Query {
    #[serde(default)]
    compression: Option<bool>,
    #[serde(default)]
    is_master: Option<bool>,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://{}:{}/", self.scheme, self.ip, self.port)
    }
}

/// Retrieves the credentials for accessing a relationship.
///
/// The relationship must be defined in the .platform.app.yaml file.
pub fn get_services() -> Option<HashMap<String, Service>> {
    let value = env_getter::get_json_from_var("PLATFORM_RELATIONSHIPS")?;
    let value_map = value.as_object().unwrap();

    let mut services: HashMap<String, Service> = HashMap::new();
    for (rel_name, relation) in value_map {
        let service: Service = serde_json::from_value(relation[0].clone()).unwrap();
        services.insert(rel_name.to_owned(), service);
    }
    Some(services)
}
