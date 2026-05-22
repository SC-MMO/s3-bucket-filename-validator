use indexmap::IndexMap;
use regex::Regex;

pub fn build_folder_map(file_paths: Vec<String>) -> IndexMap<String, Vec<String>> {
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

pub fn filter_out_first_txt(folder_map: &mut IndexMap<String, Vec<String>>) {
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

pub fn build_report(folder_map: IndexMap<String, Vec<String>>) -> IndexMap<String, Vec<String>> {
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
