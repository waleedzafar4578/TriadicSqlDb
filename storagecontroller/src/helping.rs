use crate::BaseControl;
use std::fs;

impl BaseControl {
    pub fn new() -> BaseControl {
        Self {
            system_path: "".to_string(),
            database_name: "".to_string(),
            db_select: false,
            initiate_lock: false,
            all_table: vec![],
        }
    }
    pub fn initiate_database(&mut self, path: &str) -> bool {
        if self.initiate_lock == false {
            self.initiate_lock = true;
            self.system_path = path.to_string();
            return true;
        }
        false
    }
    pub fn list_down_the_name_database(&self) -> bool {
        /*
        fs::read dir function return all inside dir of given path
           then use For loop to iterate each dir and then compare the metadata which help to
           identify whether it is dir or file
           after identify dir display on console
         */
        if let Ok(entries) = fs::read_dir(self.system_path.clone()) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            format!("Folder: {}", name);
                        }
                    }
                }
            }
        } else {
            eprintln!("Error reading directory '{}'", &self.system_path);
        }
        return true;
    }
    pub fn list_of_tables(&self){
        for cl in &self.all_table{
            println!("Table Name: {}",cl.get_table_name());
        }

    }
    pub fn tables_name(self) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        for i in self.all_table {
            ret.push(i.table_name());
        }
        ret
    }
    pub fn search_table(self, name: &str) -> bool {
        for i in self.all_table {
            if i.table_name() == name {
                return true;
            }
        }
        false
    }
    pub fn find_this_database(&mut self, path: &str) -> bool {
        if let Ok(entries) = fs::read_dir(self.system_path.clone()) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            if path == name {
                                //println!("This {} database is find!", path);
                                return true;
                            }
                        }
                    }
                }
            }
        } else {
            eprintln!("Error reading directory '{}'", &mut self.system_path);
        }
        return false;
    }
    pub fn use_this_database(&mut self, path: &str) -> bool {
        if self.db_select == false && self.initiate_lock == true {
            if let Ok(entries) = fs::read_dir(self.system_path.clone()) {
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            if let Some(name) = entry.file_name().to_str() {
                                if path == name {
                                    self.database_name = name.parse().unwrap();
                                }
                            }
                        }
                    }
                }
            } else {
                eprintln!("Error reading directory '{}'", &mut self.system_path);
            }
            self.db_select = true;
            return true;
        }
        return false;
    }
}
