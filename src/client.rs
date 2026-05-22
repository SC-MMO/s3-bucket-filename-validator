use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, config::Builder};

pub trait ClientExt {
    async fn build_client() -> Client;
    async fn get_filepaths(&self, bucket_name: &str) -> Result<Vec<String>>;
}

impl ClientExt for Client {
    async fn build_client() -> Client {
        let gen_config = aws_config::defaults(BehaviorVersion::latest()).load().await;

        let config = Builder::from(&gen_config).force_path_style(true).build();

        return Client::from_conf(config);
    }

    async fn get_filepaths(&self, bucket_name: &str) -> Result<Vec<String>> {
        let mut file_paths: Vec<String> = Vec::new();

        // pagination cause it only returns 1000 objects (filepaths) per page
        let mut continuation_token: Option<String> = None;

        loop {
            let response = self
                .list_objects_v2()
                .bucket(bucket_name)
                .set_continuation_token(continuation_token.clone())
                .send()
                .await?;

            for object in response.contents() {
                if let Some(key) = object.key() {
                    file_paths.push(key.to_string());
                }
            }

            if response.is_truncated() != Some(true) {
                break;
            }

            continuation_token = response.next_continuation_token().map(String::from);
        }

        return Ok(file_paths);
    }
}
