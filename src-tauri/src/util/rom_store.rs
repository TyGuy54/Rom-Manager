use std::collections::HashMap;

use crate::rom_ops::Value;
use crate::rom_ops::RomType;
use crate::rom_ops::RomData;

pub struct RomStore {
    pub roms: HashMap<RomType, Vec<RomData>>,
}


// this implimintaion simplifies the process of adding roms specific data to a vector and hashmap
impl RomStore {
    // Function to add ROM data to the store
    pub fn add_rom(&mut self, rom_type: RomType, rom_data: RomData) {
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
    pub fn get_roms(&self, rom_type: RomType) -> Option<&Vec<RomData>> {
        self.roms.get(&rom_type)
    }
}