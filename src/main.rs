mod backblaze;
mod backup;
mod ntfy;
mod utils;

use backblaze::BackBlazeB2Uploader;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let out_file_name = backup::create_backup();
    info!("created backup file \"{}\"", &out_file_name);

    BackBlazeB2Uploader::upload_file_as_stream(
        &out_file_name,
        &format!("backups/{}", &out_file_name),
    )
    .await;

    info!("done");
}
