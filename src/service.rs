use crate::env_getter;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
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

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_nonexistant_service() {
        let relationships = "eyJkYXRhYmFzZSI6IFt7InVzZXJuYW1lIjogInVzZXIiLCAic2NoZW1lIjogIm15c3FsIiwgInNlcnZpY2UiOiAibXlzcWxkYiIsICJmcmFnbWVudCI6IG51bGwsICJpcCI6ICIxNjkuMjU0LjEyMS4xODYiLCAiaG9zdG5hbWUiOiAiN29waWd5Z28yMm1xemtoYXk0cHRncXI0Y3kubXlzcWxkYi5zZXJ2aWNlLl8uZXUtMi5wbGF0Zm9ybXNoLnNpdGUiLCAicHVibGljIjogZmFsc2UsICJjbHVzdGVyIjogInNjZXZtamZyeHUzcGstbWFzdGVyLTdycXR3dGkiLCAiaG9zdCI6ICJkYXRhYmFzZS5pbnRlcm5hbCIsICJyZWwiOiAibXlzcWwiLCAicXVlcnkiOiB7ImlzX21hc3RlciI6IHRydWV9LCAicGF0aCI6ICJtYWluIiwgInBhc3N3b3JkIjogIiIsICJ0eXBlIjogIm15c3FsOjEwLjIiLCAicG9ydCI6IDMzMDZ9XX0=";
        env::set_var("PLATFORM_RELATIONSHIPS", relationships);

        let services = get_services().unwrap();
        assert_eq!(None, services.get("redis-cache"));
    }

    #[test]
    fn test_existant_service() {
        let relationships = "eyJkYXRhYmFzZSI6IFt7InVzZXJuYW1lIjogInVzZXIiLCAic2NoZW1lIjogIm15c3FsIiwgInNlcnZpY2UiOiAibXlzcWxkYiIsICJmcmFnbWVudCI6IG51bGwsICJpcCI6ICIxNjkuMjU0LjEyMS4xODYiLCAiaG9zdG5hbWUiOiAiN29waWd5Z28yMm1xemtoYXk0cHRncXI0Y3kubXlzcWxkYi5zZXJ2aWNlLl8uZXUtMi5wbGF0Zm9ybXNoLnNpdGUiLCAicHVibGljIjogZmFsc2UsICJjbHVzdGVyIjogInNjZXZtamZyeHUzcGstbWFzdGVyLTdycXR3dGkiLCAiaG9zdCI6ICJkYXRhYmFzZS5pbnRlcm5hbCIsICJyZWwiOiAibXlzcWwiLCAicXVlcnkiOiB7ImlzX21hc3RlciI6IHRydWV9LCAicGF0aCI6ICJtYWluIiwgInBhc3N3b3JkIjogIiIsICJ0eXBlIjogIm15c3FsOjEwLjIiLCAicG9ydCI6IDMzMDZ9XX0=";
        env::set_var("PLATFORM_RELATIONSHIPS", relationships);
        let database = Service {
            scheme: "mysql".to_string(),
            service: "mysqldb".to_string(),
            ip: "169.254.121.186".to_string(),
            cluster: "scevmjfrxu3pk-master-7rqtwti".to_string(),
            host: "database.internal".to_string(),
            port: 3306,
            rel: Some("mysql".to_string()),
            fragment: None,
            hostname: Some(
                "7opigygo22mqzkhay4ptgqr4cy.mysqldb.service._.eu-2.platformsh.site".to_string(),
            ),
            public: Some(false),
            type_field: Some("mysql:10.2".to_string()),
            username: Some("user".to_string()),
            password: Some("".to_string()),
            query: Some(Query {
                compression: None,
                is_master: Some(true),
            }),
            path: Some("main".to_string()),
        };

        let services = get_services().unwrap();
        assert_eq!(Some(&database), services.get("database"));
    }
}
