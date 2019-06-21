use serde_json::Value;
use std::env;
use std::str;
extern crate base64;

pub fn get_json_from_platform_var(var: &str) -> Value {
    let b64_rel = env::var(var).unwrap();
    let rel = base64::decode(&b64_rel).unwrap();
    serde_json::from_str(str::from_utf8(&rel).unwrap()).unwrap()
}
