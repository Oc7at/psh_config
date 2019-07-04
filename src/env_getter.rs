use serde_json::Value;
use std::env;
use std::str;
extern crate base64;

pub fn get_env(var: &str) -> Option<String> {
    match env::var(var) {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}

pub fn get_json_from_var(var: &str) -> Option<Value> {
    let content = get_env(var)?;

    let b64_rel = env::var(content).unwrap();
    let rel = base64::decode(&b64_rel).unwrap();
    Some(serde_json::from_str(str::from_utf8(&rel).unwrap()).unwrap())
}
