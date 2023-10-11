use std::fs::File;
use std::path::Path;
use crate::data::Data;

pub struct SaveSystem {
    save_file: String,
}

impl SaveSystem {
    pub fn new(save_file: String) -> Self {
        SaveSystem::setup_save_file(&save_file);
        return Self { save_file };
    }

    fn setup_save_file(save_file: &String) {
        if (Path::new(&save_file)).exists() {
            println!("Save file exists");
        } else {
            println!("Save file does not exist. Creating save file...");
            File::create(&save_file).unwrap();
        }
    }

    pub fn save(&self, data: &Data) {
        println!("Saving...");
        let file = File::create(&self.save_file).unwrap();
        let result = bincode::serialize_into(file, &data);
        match result {
            Ok(_) => println!("Save successful"),
            Err(_) => println!("Save failed"),
        }
    }

    pub fn load(&self) -> Data {
        println!("Loading...");
        let file = File::open(&self.save_file).unwrap();
        let result = bincode::deserialize_from(file);
        return match result {
            Ok(data) => data,
            Err(_) => Data { tasks: Vec::new() },
        }
    }
}
