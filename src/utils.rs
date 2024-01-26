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
