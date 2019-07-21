use serde_json::Value;
use std::env;

pub mod env_getter;
pub mod route;
pub mod service;

/// Checks whether the code is running on a platform with valid environment variables.
///
/// True if configuration can be used, false otherwise.
pub fn is_valid_platform() -> bool {
    env_getter::get_env("PLATFORM_APPLICATION_NAME").is_some()
}

/// Checks whether the code is running in a build environment.
///
/// If false, it's running at deploy time.
pub fn in_build() -> bool {
    is_valid_platform() && env_getter::get_env("PLATFORM_ENVIRONMENT").is_none()
}

/// Checks whether the code is running in a runtime environment.
pub fn in_runtime() -> bool {
    is_valid_platform() && env_getter::get_env("PLATFORM_ENVIRONMENT").is_some()
}

/// Returns a variable from the VARIABLES array.
///
/// Note: variables prefixed with `env:` can be accessed as normal environment
/// variables.
///
/// This method will return such a variable by the name with the prefix still
/// included.
///
/// Generally it's better to access those variables directly.
pub fn variable(name: &str) -> Option<String> {
    let vars = env_getter::get_json_from_var("PLATFORM_VARIABLES")?;
    if vars[&name].is_string() {
        Some(vars[&name].to_string())
    } else {
        None
    }
}

/// Returns the full variables array.
///
/// If you're looking for a specific variable, the variable() method is a more robust option.
///
/// This method is for cases where you want to scan the whole variables list looking for a pattern.
pub fn variables() -> Option<Value> {
    env_getter::get_json_from_var("PLATFORM_VARIABLES")
}

/// Returns the application definition array.
///
/// This is, approximately, the .platform.app.yaml file as a nested array.  However, it also
/// has other information added by Platform.sh as part of the build and deploy process.
pub fn application() -> Option<Value> {
    env_getter::get_json_from_var("PLATFORM_APPLICATION")
}

/// Determines if the current environment is a Platform.sh Enterprise environment.
pub fn on_enterprise() -> bool {
    is_valid_platform() && env::var("PLATFORM_MODE").unwrap() == "enterprise"
}

/// Determines if the current environment is a production environment.
///
/// Note: There may be a few edge cases where this is not entirely correct on Enterprise,
/// if the production branch is not named `production`.  In that case you'll need to use
/// your own logic.
pub fn on_production() -> bool {
    let prod_branch = if on_enterprise() {
        "production"
    } else {
        "master"
    };
    env::var("PLATFORM_BRANCH").unwrap() == prod_branch
}
