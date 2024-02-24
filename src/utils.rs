use tracing::error;

pub fn get_env(key: &str) -> String {
    match std::env::var(key) {
        Ok(val) => val,
        Err(_) => {
            error!("failed to get environment variable \"{}\"", key);
            std::process::exit(1);
        }
    }
}

pub fn get_env_without_exit(key: &str) -> Option<String> {
    match std::env::var(key) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}
