use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::process::Command;
use std::collections::HashMap;
use std::string::String;
use serde::{Deserialize, Serialize};
use crate::error::Error;


// while appending the values to hashmaps in the [get_imgs] and [card_data]
// we nned the name of the path and the name of the file to be the same type
// this enum helps with that
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Value {
    Path(PathBuf),
    Name(String),
    NameExtn(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)] 
enum RomType {
    Rom,
}

#[derive(Debug, Clone, Serialize)] // Derive Debug and Clone traits for RomData
pub struct RomData {
    file_location: Value,
    file_extension: Value,
    file_name: Value,
}

struct RomStore {
    roms: HashMap<RomType, Vec<RomData>>,
}

impl RomStore {
    // Function to add ROM data to the store
    fn add_rom(&mut self, rom_type: RomType, rom_data: RomData) {
        let rom_type_entry = self.roms.entry(rom_type).or_insert(Vec::new());

        // Retain only entries with file extensions other than "sav" or "SAV"
        rom_type_entry.retain(|existing_rom_data| {
            match (&existing_rom_data.file_extension, &rom_data.file_extension) {
                (Value::NameExtn(extn1), Value::NameExtn(extn2)) => {
                    extn1.to_lowercase() != "sav" || extn2.to_lowercase() != "sav"
                }
                _ => true,
            }
        });

        // Check if the new entry has a file extension "sav" or "SAV"
        if let Value::NameExtn(extn) = &rom_data.file_extension {
            if extn.to_lowercase() != "sav" {
                rom_type_entry.push(rom_data);
            }
        } else {
            rom_type_entry.push(rom_data);
        }
    }

    // Function to retrieve ROM data for a specific type
    fn get_roms(&self, rom_type: RomType) -> Option<&Vec<RomData>> {
        self.roms.get(&rom_type)
    }
}

// a function runs the open command to run the rom in its emulator
#[tauri::command]
pub fn load_rom(rom_name: &str) {
    Command::new("open") // this is macs version of xdg-open for linux
    .arg(rom_name)
    .output()
    .expect("failed to execute process");
}

// a funtion that makes a vector wth a hashmap inside of it 
// loops over the ROM-imgs/GBA directory and appends every image and its path to the hashmap
#[tauri::command]
pub fn get_img_data() -> Result<Vec<HashMap<&'static str, Value>>, Error> {
    let mut cool_vec = Vec::new();
    let paths = fs::read_dir("../ROM-imgs/box_arts")?;

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
pub fn get_rom_data(rom_type: String) -> Result<Vec<RomData>, Error> {
    let paths = fs::read_dir(format!("../ROMS/{}", rom_type)).unwrap();

    let mut rom_store = RomStore {
        roms: HashMap::new(),
    };

    for roms in paths {
        // let mut card_data_payload = HashMap::new();
        
        let rom_location = roms.unwrap();
        let rom_path = rom_location.path();
        let rom_name = rom_path.file_name().unwrap().to_owned();

        let rom_file_extension = Path::new(rom_path.as_os_str())
            .extension()
            .and_then(OsStr::to_str);
        
        let file_extention = rom_file_extension.unwrap().to_string();
        let remove_dot = file_extention.replace(".", "");
        let to_upper = remove_dot.to_uppercase();
        
        let roms = RomData {
            file_location:  Value::Path(PathBuf::from(rom_path.clone())),
            file_extension: Value::NameExtn(to_upper),
            file_name: Value::Name(rom_name.into_string().unwrap()),
        };
        
        rom_store.add_rom(RomType::Rom, roms.clone());

    }
    
    return Ok(rom_store.get_roms(RomType::Rom).unwrap().to_owned());
}