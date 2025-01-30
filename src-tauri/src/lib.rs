mod api;

pub fn run() {
    tauri::Builder::default()
        .setup(|_app| Ok(()))
        .invoke_handler(tauri::generate_handler![
            api::json::get_json_data,
            api::json::save_json_to_file,
            api::json::load_json_from_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
