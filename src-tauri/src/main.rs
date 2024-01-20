// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use rom_ops::{load_rom, get_rom_data, get_img_data};

mod error;
mod rom_ops;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_rom, get_rom_data, get_img_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
