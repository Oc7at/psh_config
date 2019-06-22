use serde_json::{json, Value};
use std::env;

mod env_getter;
pub use env_getter::*;

pub fn is_valid_platform() -> bool {
    let key = "PLATFORM_APPLICATION_NAME";

    can_get_env(&key)
}
pub fn in_build() -> bool {
    is_valid_platform() && !can_get_env("PLATFORM_ENVIRONMENT")
}
pub fn in_runtime() -> bool {
    is_valid_platform() && can_get_env("PLATFORM_ENVIRONMENT")
}
pub fn credentials(relation: &str) -> Result<Value, &str> {
    let relationships = get_json_from_var("PLATFORM_RELATIONSHIPS");
    Ok(relationships[&relation][0].clone())
}
pub fn variable(name: &str) -> Result<&str, &str> {
    let _ = name;
    Ok("Ok")
}
pub fn variables() -> Value {
    json!(null)
}
pub fn routes() -> Value {
    json!(null)
}
pub fn get_route(id: &str) -> Value {
    let _ = id;
    json!(null)
}
pub fn application() -> Value {
    json!(null)
}
pub fn on_enterprise() -> bool {
    is_valid_platform() && env::var("PLATFORM_MODE").unwrap() == "enterprise"
}
pub fn on_production() -> bool {
    let prod_branch = if on_enterprise() {
        "production"
    } else {
        "master"
    };
    env::var("PLATFORM_BRANCH").unwrap() == prod_branch
}
//pub fn registerFormatter(name: &str, callable $formatter) : self
//pub fn formattedCredentials(string $relationship, string $formatter)
pub fn has_relationship(relationship: &str) -> bool {
    let _ = relationship;
    true
}
