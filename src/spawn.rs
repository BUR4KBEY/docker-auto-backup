use std::process::Command;

use tracing::error;

pub fn spawn_child_process(command: &mut Command) {
    let mut child = {
        match command.spawn() {
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
}
