use std::env;

pub static DATA_DIR: &str = "DATA_DIR";
pub static RES_DIR: &str = "RES_DIR";
pub static PHOTOGRAMMETRY_SCRIPT: &str = "PHOTOGRAMMETRY_SCRIPT";

pub static VARS: [&str; 3] = [DATA_DIR, RES_DIR, PHOTOGRAMMETRY_SCRIPT];

/// Check that all necessary environment variable have been defined
pub fn check_env() {
    for var in VARS.iter() {
        match env::var(var) {
            Ok(val) => {
                println!("{} configured: {}", var, val)
            }
            Err(_) => {
                panic!("You must define a {} environment variable", var);
            }
        }
    }
}

/// Access to an env variable and panic if it does not exist.
pub fn get_var(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            panic!("{} must be defined", key);
        }
    }
}
