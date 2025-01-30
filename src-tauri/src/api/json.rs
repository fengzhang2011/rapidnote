use serde_json::Value;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tauri::{command, AppHandle, Manager};

#[command]
pub fn get_json_data() -> String {
    let data = r#"{"message": "Hello from JSON API!"}"#;
    data.to_string()
}

#[command]
pub fn save_json_to_file(
    app_handle: AppHandle,
    file_path: String,
    json_data: Value,
) -> Result<(), String> {
    let mut absolute_file_path: PathBuf = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    absolute_file_path.push(file_path.clone());
    println!("Saving JSON to directory: {}", absolute_file_path.display());
    let path = Path::new(&absolute_file_path);
    if let Some(parent) = path.parent() {
        create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory {}: {}", parent.display(), e))?;
    }
    if path.exists() {
        std::fs::remove_file(&absolute_file_path)
            .map_err(|e| format!("Failed to delete existing file {}: {}", file_path, e))?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&absolute_file_path)
        .map_err(|e| format!("Failed to create or open file {}: {}", file_path, e))?;
    let json_string = serde_json::to_string_pretty(&json_data)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?;
    file.write_all(json_string.as_bytes())
        .map_err(|e| format!("Failed to write to file {}: {}", file_path, e))?;
    Ok(())
}

#[command]
pub fn load_json_from_file(app_handle: AppHandle, file_path: &str) -> Result<Value, String> {
    let mut absolute_file_path: PathBuf = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;
    absolute_file_path.push(file_path);

    let mut file = File::open(absolute_file_path)
        .map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;

    let mut json_string = String::new();
    file.read_to_string(&mut json_string)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;

    let json_data: Value = serde_json::from_str(&json_string)
        .map_err(|e| format!("Failed to parse JSON from {}: {}", file_path, e))?;

    Ok(json_data)
}
