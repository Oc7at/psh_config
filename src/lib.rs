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
pub fn credentials(relationship: &str) -> Value {
    let _ = relationship;
    json!(null)
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
    true
}
pub fn on_production() -> bool {
    true
}
//pub fn registerFormatter(name: &str, callable $formatter) : self
//pub fn formattedCredentials(string $relationship, string $formatter)
pub fn has_relationship(relationship: &str) -> bool {
    let _ = relationship;
    true
}
