use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client, config::Builder};
use indexmap::IndexMap;
use regex::Regex;

async fn build_client() -> Client {
    let gen_config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let config = Builder::from(&gen_config).force_path_style(true).build();

    return Client::from_conf(config);
}

async fn get_filepaths(client: Client, bucket_name: &str) -> Result<Vec<String>> {
    let mut file_paths: Vec<String> = Vec::new();

    // pagination cause it only returns 1000 objects (filepaths) per page
    let mut continuation_token: Option<String> = None;

    loop {
        let response = client
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

fn build_folder_map(file_paths: Vec<String>) -> IndexMap<String, Vec<String>> {
    let mut folder_map: IndexMap<String, Vec<String>> = IndexMap::new();
    for path in file_paths {
        if let Some((folder, filename)) = path.rsplit_once('/') {
            folder_map
                .entry(folder.to_string())
                .or_default()
                .push(filename.to_string());
        }
    }
    return folder_map;
}

fn filter_out_first_txt(folder_map: &mut IndexMap<String, Vec<String>>) {
    for (_folder, filenames) in folder_map.iter_mut() {
        let mut removed = false;

        filenames.retain(|filename| {
            if !removed && filename.ends_with(".txt") {
                removed = true;
                return false;
            }

            return true;
        });
    }

    return;
}

fn get_report(folder_map: IndexMap<String, Vec<String>>) -> IndexMap<String, Vec<String>> {
    let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}\.json$").unwrap();

    let mut folder_report: IndexMap<String, Vec<String>> = IndexMap::new();

    folder_report.insert(String::from("valid"), Vec::new());
    folder_report.insert(String::from("invalid"), Vec::new());

    for (folder, filenames) in folder_map {
        let all_valid = filenames
            .iter()
            .all(|filename| date_regex.is_match(filename));

        if all_valid {
            folder_report.get_mut("valid").unwrap().push(folder);
        } else {
            folder_report.get_mut("invalid").unwrap().push(folder);
        }
    }

    return folder_report;
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let client = build_client().await;

    let file_paths = get_filepaths(client, "reports").await?;

    let mut folder_map = build_folder_map(file_paths);
    filter_out_first_txt(&mut folder_map);

    let report = get_report(folder_map);

    let json_report = serde_json::to_string_pretty(&report).unwrap();
    println!("Report:\n{}", json_report);

    return Ok(());
}
