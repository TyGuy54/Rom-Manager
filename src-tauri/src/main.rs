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

// while appending the values to hashmaps in the [get_imgs] and [card_data]
// we nned the name of the path and the name of the file to be the same type
// this enum helps with that
#[derive(Debug, Serialize, Deserialize)]
enum Value {
    Path(PathBuf),
    Name(String),
    NameExtn(String)
}

// for error handling with tauri
#[derive(Debug, thiserror::Error)]
enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error)
}

// for error handling with tauri
// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

// a function runs the open command to run the rom in its emulator
#[tauri::command]
fn load_rom(rom_name: &str) {
    Command::new("open") // this is macs version of xdg-open for linux
    .arg(rom_name)
    .output()
    .expect("failed to execute process");
}

// a funtion that makes a vector wth a hashmap inside of it 
// loops over the ROM-imgs/GBA directory and appends every image and its path to the hashmap
#[tauri::command]
fn get_img_data() -> Result<Vec<HashMap<&'static str, Value>>, Error> {
    let mut cool_vec = Vec::new();
    let paths = fs::read_dir("../ROM-imgs/GBA")?;

    for imgs in paths {
        let mut card_data_payload = HashMap::new();

        let img_location = imgs?;
        let img_path = img_location.path();
        let img_name = img_path.file_name().unwrap().to_owned();

        card_data_payload.insert("img_path", Value::Path(img_path));
        card_data_payload.insert("img_name", Value::Name(img_name.into_string().unwrap()));

        cool_vec.push(card_data_payload);
    }

    return Ok(cool_vec);
}

// a funtion that makes a vector wth a hashmap inside of it 
// loops over the ROMS/GBA directory and grabs every roms and its path and appends it to the hashmap
#[tauri::command]
fn get_rom_data() -> Result<Vec<HashMap<&'static str, Value>>, Error>{
    let mut cool_vec = Vec::new();
    let paths = fs::read_dir("../ROMS/GBA")?;

    for roms in paths {
        let mut card_data_payload = HashMap::new();
        
        let rom_location = roms?;
        let rom_path = rom_location.path();
        let rom_name = rom_path.file_name().unwrap().to_owned();

        let rom_file_extension = Path::new(rom_path.as_os_str())
            .extension()
            .and_then(OsStr::to_str);
        
        match rom_file_extension.unwrap() {
            "gba" => { 
                card_data_payload.insert("rom_path", Value::Path(rom_path.clone()));
                card_data_payload.insert("rom_name", Value::Name(rom_name.into_string().unwrap()));
                
                let file_extention = rom_file_extension.unwrap().to_string();
                let remove_dot = file_extention.replace(".", "");
                let to_upper = remove_dot.to_uppercase();
                
                card_data_payload.insert("rom_extn", Value::NameExtn(to_upper));

                cool_vec.push(card_data_payload)
            },
            "nes" => {
                card_data_payload.insert("rom_path", Value::Path(rom_path.clone()));
                card_data_payload.insert("rom_name", Value::Name(rom_name.into_string().unwrap()));

                let file_extention = rom_file_extension.unwrap().to_string();
                let remove_dot = file_extention.replace(".", "");
                let to_upper = remove_dot.to_uppercase();
                
                card_data_payload.insert("rom_extn", Value::NameExtn(to_upper));

                cool_vec.push(card_data_payload)
            },
            "gb" => {
                card_data_payload.insert("rom_path", Value::Path(rom_path.clone()));
                card_data_payload.insert("rom_name", Value::Name(rom_name.into_string().unwrap()));

                let file_extention = rom_file_extension.unwrap().to_string();
                let remove_dot = file_extention.replace(".", "");
                let to_upper = remove_dot.to_uppercase();
                
                card_data_payload.insert("rom_extn", Value::NameExtn(to_upper));

                cool_vec.push(card_data_payload)
            },
            "gbc" => {
                card_data_payload.insert("rom_path", Value::Path(rom_path.clone()));
                card_data_payload.insert("rom_name", Value::Name(rom_name.into_string().unwrap()));

                let file_extention = rom_file_extension.unwrap().to_string();
                let remove_dot = file_extention.replace(".", "");
                let to_upper = remove_dot.to_uppercase();
                
                card_data_payload.insert("rom_extn", Value::NameExtn(to_upper));

                cool_vec.push(card_data_payload)
            },
            _ => {}
        }
    }
    
    return Ok(cool_vec);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_rom, get_rom_data, get_img_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
