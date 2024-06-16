use crate::BaseControl;
use std::fs::OpenOptions;
use std::io::{Read, Write};

impl BaseControl {
    pub fn save_to_file(&self) {
        let file_name = (*self.system_path).to_owned() + &*self.database_name +".json";
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_name)
            .expect("Failed to open file!");
        let streem = serde_json::to_string_pretty(&self).unwrap();
        file.write_all(streem.as_bytes()).unwrap();
    }
    pub fn load_to_file(&mut self) -> BaseControl {
        
        let p_file_name = (*self.system_path).to_owned() + &*self.database_name +".json";
        let mut file = OpenOptions::new()
            .read(true)
            .open(p_file_name)
            .expect("Failed to open file!");
        let mut streem = String::new();
        file.read_to_string(&mut streem).unwrap();
        return match serde_json::from_str(&streem) {
            Ok(data) => {
                data
            }
            Err(_) => {
                println!("Something wrong when load file,now giving you fresh object!");
                BaseControl::new()
            }
        }
    }
}
