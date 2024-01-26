use s3::creds::Credentials;
use s3::error::S3Error;
// use s3::request::ResponseData;
use s3::{Bucket, Region};
use tokio::fs::File;

pub struct BackBlazeB2Storage {
    key_id: String,
    application_key: String,
    bucket_region: String,
    bucket_name: String,
}

impl BackBlazeB2Storage {
    pub fn new(
        key_id: String,
        application_key: String,
        bucket_region: String,
        bucket_name: String,
    ) -> Self {
        Self {
            key_id,
            application_key,
            bucket_region,
            bucket_name,
        }
    }

    fn get_bucket(&self) -> Result<Bucket, S3Error> {
        let bucket = Bucket::new(
            &self.bucket_name,
            Region::Custom {
                region: self.bucket_region.to_owned(),
                endpoint: format!("s3.{}.backblazeb2.com", self.bucket_region),
            },
            Credentials::new(
                Some(&self.key_id),
                Some(&self.application_key),
                None,
                None,
                None,
            )?,
        )?;

        Ok(bucket)
    }

    // pub async fn upload(&self, path: &str, data: &[u8]) -> Result<ResponseData, S3Error> {
    //     let bucket = self.get_bucket()?;
    //     let response_data = bucket.put_object(path, data).await?;

    //     Ok(response_data)
    // }

    pub async fn upload_stream(&self, path: &str, file: &mut File) -> Result<(), S3Error> {
        let bucket = self.get_bucket()?;

        bucket.put_object_stream(file, path).await?;

        Ok(())
    }
}
