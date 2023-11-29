// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use markdown::{to_html_with_options, Options};
use tauri_plugin_log::{LogTarget};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn markdown2html(text: &str) -> String {
    let result = to_html_with_options(text, &Options::gfm());
    format!("{}", result.unwrap())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![markdown2html])
        .plugin(tauri_plugin_log::Builder::default().targets([
            LogTarget::LogDir,
            LogTarget::Stdout,
            LogTarget::Webview,
        ]).build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
