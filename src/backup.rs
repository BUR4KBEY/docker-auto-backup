use super::utils;
use std::process::Command;

use chrono::{DateTime, Local};
use tracing::{error, info};

pub fn create_backup() -> String {
    let date = {
        let now: DateTime<Local> = Local::now();
        let formatted = now.format("%Y-%m-%d_%H-%M-%p");

        info!(
            "creating a new backup with the formatted date \"{}\"",
            &formatted
        );

        formatted.to_string()
    };

    let recipient = utils::get_env("GPG_RECIPIENT");

    let mut child = {
        match Command::new("bash")
            .arg("generate_backup.sh")
            .env("GPG_RECIPIENT", recipient)
            .env("DATE", &date)
            .spawn()
        {
            Ok(child) => child,
            Err(e) => {
                error!("failed to spawn child process\n\n{}", e);
                std::process::exit(1);
            }
        }
    };

    match child.wait() {
        Ok(_) => (),
        Err(e) => {
            error!("failed to wait for child process\n\n{}", e);
            std::process::exit(1);
        }
    }

    String::from(format!("{}.tar.zst.gpg", &date))
}