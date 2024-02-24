use reqwest::{Certificate, Client};
use std::fs;
use tracing::{info, error};

use crate::utils;

pub async fn send_notification(message: &str) {
    let ntfy_url = utils::get_env_without_exit("NTFY_URL");
    let ca_file_path = utils::get_env_without_exit("NTFY_CA_FILE_PATH");

    if let Some(url) = ntfy_url {
        let mut client = Client::builder().use_rustls_tls();

        if let Some(path) = ca_file_path {
            let pem = match fs::read_to_string(path) {
                Ok(data) => data,
                Err(e) => {
                    error!("failed to read the ca file: {}", e);
                    std::process::exit(1);
                }
            };

            let cert = match Certificate::from_pem(pem.as_bytes()) {
                Ok(cert) => cert,
                Err(e) => {
                    error!("failed to parse the ca file: {}", e);
                    std::process::exit(1);
                }
            };


            client = client.add_root_certificate(cert);
        }

        let client = match client.build() {
            Ok(client) => client,
            Err(e) => {
                error!("failed to build the client: {}", e);
                std::process::exit(1);
            }
        };

        let response = client.post(url).body(message.to_string()).send().await;

        match response {
            Ok(_) => info!("notification sent"),
            Err(e) => info!("failed to send notification: {}", e),
        }
    } else {
        info!("skipping notification");
    }
}
