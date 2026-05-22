use crate::client::ClientExt;
use anyhow::Result;
use aws_sdk_s3::Client;

mod client;
mod helpers;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let client: Client = Client::build_client().await;

    let file_paths = client.get_filepaths("reports").await?;

    let mut folder_map = helpers::build_folder_map(file_paths);
    helpers::filter_out_first_txt(&mut folder_map);

    let report = helpers::build_report(folder_map);

    let json_report = serde_json::to_string_pretty(&report).unwrap();
    println!("Report:\n{}", json_report);

    return Ok(());
}
