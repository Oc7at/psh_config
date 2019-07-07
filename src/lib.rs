use serde_json::Value;
use std::env;

mod env_getter;
pub use env_getter::*;

pub fn is_valid_platform() -> bool {
    match get_env("PLATFORM_APPLICATION_NAME") {
        Some(_) => true,
        None => false,
    }
}

pub fn in_build() -> bool {
    is_valid_platform()
        && match get_env("PLATFORM_ENVIRONMENT") {
            Some(_) => false,
            None => true,
        }
}

pub fn in_runtime() -> bool {
    is_valid_platform()
        && match get_env("PLATFORM_ENVIRONMENT") {
            Some(_) => true,
            None => false,
        }
}

pub fn credentials(relation: &str) -> Option<Value> {
    let relationships = get_json_from_var("PLATFORM_RELATIONSHIPS")?;
    Some(relationships[&relation][0].clone())
}

pub fn variable(name: &str) -> Option<String> {
    let vars = get_json_from_var("PLATFORM_VARIABLES")?;
    match vars[&name].is_string() {
        true => Some(vars[&name].to_string()),
        false => None,
    }
}

pub fn variables() -> Option<Value> {
    get_json_from_var("PLATFORM_VARIABLES")
}

pub fn routes() -> Option<Value> {
    get_json_from_var("PLATFORM_ROUTES")
}

pub fn get_route(id: &str) -> Option<(String, Value)> {
    let routes = get_json_from_var("PLATFORM_ROUTES")?;

    let routes_it = routes.as_object()?.iter();
    for (route, settings) in routes_it {
        if &settings["id"] == id {
            return Some((route.to_owned(), routes[&route].clone()));
        }
    }
    None
}

pub fn application() -> Option<Value> {
    get_json_from_var("PLATFORM_APPLICATION")
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

pub fn has_relationship(relationship: &str) -> bool {
    let relationships = get_json_from_var("PLATFORM_RELATIONSHIPS");

    let relationships = match relationships {
        None => return false,
        Some(rel) => rel,
    };

    if relationships[&relationship].is_array() {
        return true;
    }
    false
}
