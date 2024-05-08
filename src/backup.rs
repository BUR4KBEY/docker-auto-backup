use super::utils;
use crate::{
    config_parser::{BackupFile, Config},
    spawn::spawn_child_process,
};

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

    let mut command = Command::new("bash");
    command
        .arg("generate_backup.sh")
        .env("GPG_RECIPIENT", recipient)
        .env("DATE", &date);
    spawn_child_process(&mut command);

    String::from(format!("{}.tar.zst.gpg", &date))
}

pub fn cleanup(file_name: &str) {
    let cleanup = match utils::get_env_without_exit("DO_NOT_CLEANUP") {
        Some(data) => data != "true",
        None => true,
    };

    if !cleanup {
        info!("skipping cleanup");
        return;
    }

    match std::fs::remove_file(file_name) {
        Ok(_) => {
            info!("(cleanup) file \"{}\" deleted", file_name);
        }
        Err(e) => {
            error!("(cleanup) failed to delete file \"{}\"\n\n{}", file_name, e);
            std::process::exit(1);
        }
    }
}

pub async fn create_docker_containers_backup(config: &Config) {
    for container in &config.containers {
        if let Some(pre_build_script) = &container.pre_build_script {
            for command_str in pre_build_script {
                info!(
                    "[{}] running pre-build script: \"{}\"",
                    &container.name, command_str
                );

                let mut command = Command::new("docker");
                command
                    .arg("exec")
                    .arg("-it")
                    .arg(&container.name)
                    .arg("sh")
                    .arg("-c")
                    .arg(&command_str);
                spawn_child_process(&mut command);

                info!("[{}] done", &container.name);
            }
        }

        for BackupFile(target_path, backup_path) in &container.files {
            info!(
                "[{}] creating the local path \"{}\"",
                &container.name, backup_path
            );

            let mut mkdir_command = Command::new("mkdir");
            mkdir_command.arg("-p").arg(backup_path);
            spawn_child_process(&mut mkdir_command);

            info!("[{}] copying files to the local path", &container.name);

            let mut cp_command = Command::new("docker");
            cp_command
                .arg("cp")
                .arg(format!("{}:{}", &container.name, target_path))
                .arg(backup_path);
            spawn_child_process(&mut cp_command);

            info!("[{}] done", &container.name);
        }

        if let Some(post_build_script) = &container.post_build_script {
            for command_str in post_build_script {
                info!(
                    "[{}] running post-build script: \"{}\"",
                    &container.name, command_str
                );

                let mut command = Command::new("docker");
                command
                    .arg("exec")
                    .arg("-it")
                    .arg(&container.name)
                    .arg("sh")
                    .arg("-c")
                    .arg(&command_str);
                spawn_child_process(&mut command);

                info!("[{}] done", &container.name);
            }
        }
    }
}
