use std::fs;
use std::io::Error;
use serde_json::Value;
pub fn get_json_files(dir: &str) -> Result<Vec<String>, Error> {
    let entries = fs::read_dir(dir)?;
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "json") {
            if let Some(file_path) = path.to_str() {
                let content = fs::read_to_string(file_path)?;
                let json: Value = serde_json::from_str(&content)?;

                if json.is_array() {
                    files.push(file_path.to_string());
                }
            }
        }
    }

    Ok(files)
}