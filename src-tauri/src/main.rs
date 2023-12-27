// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::env;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::process::Command;
use std::collections::HashMap;
use std::string::String;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
enum Value {
    Path(PathBuf),
    String(String),
}

#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error)
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn load_rom(rom_name: &str) {
    let output = Command::new("open") // this is macs version of xdg-open for linux
    .arg(rom_name)
    .output()
    .expect("failed to execute process");

    println!("status: {}", output.status);
}

#[tauri::command]
fn get_imgs() -> Result<Vec<HashMap<&'static str, Value>>, Error> {
    let mut cool_vec = Vec::new();
    let paths = fs::read_dir("../ROM-imgs/GBA").unwrap();

    for imgs in paths {
        let mut card_data_payload = HashMap::new();

        let img_location = imgs.unwrap();
        let img_path = img_location.path();
        let img_name = img_path.file_name().unwrap().to_owned();

        card_data_payload.insert("img_path", Value::Path(img_path));
        card_data_payload.insert("img_name", Value::String(img_name.into_string().unwrap()));
        cool_vec.push(card_data_payload);
    }

    return Ok(cool_vec);
}


#[tauri::command]
fn card_data() -> Result<Vec<HashMap<&'static str, Value>>, Error>{
    let mut cool_vec = Vec::new();
    let paths = fs::read_dir("../ROMS/GBA").unwrap();

    for roms in paths {
        let mut card_data_payload = HashMap::new();
        
        let rom_location = roms.unwrap();
        let rom_path = rom_location.path();
        let rom_name = rom_path.file_name().unwrap().to_owned();

        let rom_file_extension = Path::new(rom_path.as_os_str())
            .extension()
            .and_then(OsStr::to_str);
        
        match rom_file_extension.unwrap() {
            "gba" => { 
                card_data_payload.insert("rom_path", Value::Path(rom_path.clone()));
                card_data_payload.insert("rom_name", Value::String(rom_name.into_string().unwrap()));

                cool_vec.push(card_data_payload)
            },
            _ => {
                let err = "File Extention Does Not Exist".to_string();
                println!("{}", err);
            }
        }
    }
    
    return Ok(cool_vec);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_rom, card_data, get_imgs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
