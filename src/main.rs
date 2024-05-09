mod backblaze;
mod backup;
mod config_parser;
mod ntfy;
mod spawn;
mod utils;

use backblaze::BackBlazeB2Uploader;
use config_parser::get_config;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    if let Some(config) = get_config() {
        info!("config file found");
        backup::create_docker_containers_backup(&config).await;
    }

    let out_file_name = backup::create_backup();
    info!("created backup file \"{}\"", &out_file_name);

    BackBlazeB2Uploader::upload_file_as_stream(
        &out_file_name,
        &format!("backups/{}", &out_file_name),
    )
    .await;

    info!("done");
}
