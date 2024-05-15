// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Clean {
    base: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(async)]
fn read(_path: &str) -> Clean {
    let path = tauri::api::dialog::blocking::FileDialogBuilder::new()
        .set_directory("/home/chiffa/Dev/Solutions/C++/Test/").pick_file();
    let base = fs::read_to_string(path.unwrap()).unwrap_or("Error".to_owned());
    Clean { base }
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, read])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
