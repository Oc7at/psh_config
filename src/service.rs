use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
struct Service {
    scheme: String,
    service: String,
    fragment: Option<String>,
    ip: String,
    hostname: String,
    public: Option<bool>,
    cluster: String,
    host: String,
    rel: Option<String>, // It is not always set on Enterprise
    port: i64,
    #[serde(rename = "type")]
    type_field: String,
    username: Option<String>,
    password: Option<String>,
    query: Option<Query>,
    path: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
struct Query {
    compression: bool,
    is_master: bool,
}
