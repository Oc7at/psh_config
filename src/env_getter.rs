use serde_json::Value;
use std::env;
use std::str;
extern crate base64;

/// Wrapper over env::var()
///
/// Its only use is that I wanted to get an Option rather than a Result
pub fn get_env(var: &str) -> Option<String> {
    match env::var(var) {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}

/// Decodes a Platform variable and passes it through serde_json::from_str()
pub fn get_json_from_var(var: &str) -> Option<Value> {
    let b64_rel = get_env(var)?;
    let rel = base64::decode(&b64_rel).unwrap();
    let rel_str = str::from_utf8(&rel).unwrap();
    Some(serde_json::from_str(rel_str).unwrap())
}
