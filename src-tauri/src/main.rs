// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn getcards(name: &str) -> Result<usize, String> {
    let path = Path::new("/mnt/sda1/cards/");

    // Check if the directory exists
    if !path.exists() {
        return Err("Directory not found".into());
    }
    // Read the directory and count the files matching the pattern
    match fs::read_dir(path) {
        Ok(entries) => {
            let count: usize = entries
                .filter_map(Result::ok)
                .filter(|entry| {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();
                    file_name_str.ends_with(".cards") && file_name_str.contains(".cards")
                })
                .count();

            Ok(24)
        }
        Err(e) => Err(format!("Failed to read directory: {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, getcards])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
