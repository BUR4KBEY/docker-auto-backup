use s3::error::S3Error;

use tokio::fs::File;
use tracing::{error, info};

use super::super::utils;
use super::storage::BackBlazeB2Storage;

pub struct BackBlazeB2Uploader {}

impl BackBlazeB2Uploader {
    pub async fn upload_file_as_stream(
        file_name: &str,
        path_to_write: &str,
    ) -> Result<(), S3Error> {
        let key_id = utils::get_env("BACKBLAZE_KEY_ID");
        let application_key = utils::get_env("BACKBLAZE_APPLICATION_KEY");
        let bucket_region = utils::get_env("BACKBLAZE_BUCKET_REGION");
        let bucket_name = utils::get_env("BACKBLAZE_BUCKET_NAME");

        let mut file = {
            match File::open(file_name).await {
                Ok(file) => file,
                Err(e) => {
                    error!("failed to open file \"{}\"\n\n{}", file_name, e);
                    std::process::exit(1);
                }
            }
        };

        let backblaze_bucket =
            BackBlazeB2Storage::new(key_id, application_key, bucket_region, bucket_name);

        match backblaze_bucket
            .upload_stream(path_to_write, &mut file)
            .await
        {
            Ok(_) => {
                info!("uploaded file \"{}\" to \"{}\"", file_name, path_to_write);

                Ok(())
            }

            Err(e) => {
                error!("failed to upload file \"{}\"\n\n{}", file_name, e);
                std::process::exit(1);
            }
        }
    }
}
