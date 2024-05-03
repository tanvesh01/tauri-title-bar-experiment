// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use windows::UI::{Core, WindowManagement};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // WindowManagement::AppWindowTitleBar::SetExtendsContentIntoTitleBar(, true);
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap(); // `main` is the first window from tauri.conf.json without an explicit label
            let hwnd = window.hwnd().unwrap();
            let windowId = Core::Preview::CoreAppWindowPreview::GetIdFromWindow(get_hwnd);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
