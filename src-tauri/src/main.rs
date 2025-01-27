#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Utc};
use markdown::{to_html_with_options, Options};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu,
};
use tauri_plugin_log::LogTarget;

// 定义番茄钟数据结构
#[derive(Serialize, Deserialize, Debug, Clone)]
struct TomatoClock {
    start_time: DateTime<Utc>, // 番茄钟开始时间
    clock_type: String,        // 类型: "work" 或 "rest"
}

struct AppState {
    counter: u32,
}

// 获取文件路径：使用应用数据目录
fn get_file_path(app_handle: &tauri::AppHandle) -> PathBuf {
    let app_dir = app_handle.path_resolver().app_data_dir().unwrap();
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir).unwrap();
    }
    app_dir.join("tomato_clock.json")
}

// 创建默认的番茄钟数据
fn create_default_tomato_clock() -> TomatoClock {
    TomatoClock {
        start_time: Utc::now(),
        clock_type: "work".to_string(),
    }
}

// 保存番茄钟状态到文件
fn save_tomato_clock(
    app_handle: &tauri::AppHandle,
    clock: &TomatoClock,
) -> Result<(), std::io::Error> {
    let file_path = get_file_path(app_handle); // 使用文件路径
    let clock_json = serde_json::to_string(clock)?;
    fs::write(file_path, clock_json)?;
    Ok(())
}

// 从文件加载番茄钟状态
fn load_tomato_clock(app_handle: &tauri::AppHandle) -> Option<TomatoClock> {
    let file_path = get_file_path(app_handle);
    if file_path.exists() {
        if let Ok(clock_json) = fs::read_to_string(file_path) {
            if let Ok(clock) = serde_json::from_str::<TomatoClock>(&clock_json) {
                return Some(clock);
            }
        }
    }
    None // 返回 None 表示没有找到番茄钟状态
}

// 定义一个 Tauri 命令，返回番茄钟状态
#[tauri::command]
fn get_tomato_clock(app_handle: tauri::AppHandle) -> TomatoClock {
    load_tomato_clock(&app_handle).unwrap_or_else(create_default_tomato_clock) // 使用从文件中加载的番茄钟状态或创建默认
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn markdown2html(text: &str) -> String {
    let result = to_html_with_options(text, &Options::gfm());
    format!("{}", result.unwrap())
}

#[tauri::command]
fn update_counter(state: tauri::State<Mutex<AppState>>) -> String {
    let mut state = state.lock().unwrap();
    state.counter += 1;
    format!("点击次数: {}", state.counter)
}

fn main() {
    // 创建系统托盘菜单
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("update", "刷新状态"))
        .add_item(CustomMenuItem::new("quit", "退出"));

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .setup(move |app| {
            let app_handle = app.handle();
            let tomato_clock = Arc::new(Mutex::new(
                load_tomato_clock(&app_handle).unwrap_or_else(create_default_tomato_clock),
            ));

            // 定时线程，每秒更新番茄钟状态
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(1));
            });

            // 监听关闭事件，保存数据
            app.listen_global("tauri://close-requested", move |_| {
                let clock = tomato_clock.lock().unwrap();
                if let Err(e) = save_tomato_clock(&app_handle, &clock) {
                    println!("Failed to save tomato clock: {}", e);
                } else {
                    println!("Tomato clock saved.");
                }
                app_handle.exit(0);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            markdown2html,
            get_tomato_clock,
            update_counter
        ])
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                .build(),
        )
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "update" => {
                        let app_handle = app.clone();
                        app_handle.emit_all("tray-update", "正在刷新...").unwrap();
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                },
                // 处理其他事件（如左键点击）
                SystemTrayEvent::LeftClick { .. } => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                _ => {}
            }
        })
        .manage(Mutex::new(AppState { counter: 0 }))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
