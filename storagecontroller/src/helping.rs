use crate::BaseControl;
use std::fs;
use std::path::Path;
use storge::table::{ShowTable};

impl Default for BaseControl {
    fn default() -> Self {
        Self::new()
    }
}

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
        if !self.initiate_lock {
            self.initiate_lock = true;
            self.system_path = path.to_string();
            return true;
        }
        false
    }
    pub fn list_down_the_name_database(&self) -> Vec<String> {
        /*
        fs::read dir function return all inside dir of a given path
           then use For loop to iterate each dir and then compare the metadata which helps to
           identify whether it is a dir or file
           after identify dir display on console
         */
        let mut answer: Vec<String> = vec![];

        if let Ok(entries) = fs::read_dir(self.system_path.clone()) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Some(name) = entry.file_name().to_str() {
                            let t:Vec<&str>=name.split(".").collect();
                            answer.push(format!("{} ",t[0]));
                        }
                    }
                }
            }
        } else {
            eprintln!("Error reading directory '{}'", &self.system_path);
        }
        answer
    }
    pub fn list_of_tables(&self)->Vec<String> {
        let mut names:Vec<String>=vec![];
        for cl in &self.all_table {
            //println!("Table Name: {}", cl.get_table_name());
            names.push(cl.get_table_name().to_string());
        }
        return names
    }
    pub fn tables_name(self) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        for i in self.all_table {
            ret.push(i.table_name());
        }
        ret
    }
    pub fn search_table(&self, name: &str) -> bool {
        for i in &self.all_table {
            if i.get_table_name() == name {
                return true;
            }
        }
        false
    }
    pub fn find_this_database(&mut self, path: &str) -> (String,bool) {
        if let Ok(entries) = fs::read_dir(self.system_path.clone()) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Some(name) = entry.file_name().to_str() {
                            //println!("{}",name);
                            if (path.to_owned() +".json") == name {
                                return (format!("This {} database is find!", path),true);
                            }
                        }
                    }
                }
            }
        } else {
            println!("Error reading directory '{}'", &mut self.system_path);
        }
        return (format!("This {} database is not found!", path),false);
    }
    pub fn use_this_database(&mut self, path: &str) -> bool {
        if !self.db_select && self.initiate_lock {
            let temp = &(self.system_path.clone() + path + ".json");
            let new_file_path = Path::new(temp);
            if new_file_path.exists() {
                self.database_name = path.to_string();
                self.db_select = true;
                return true;
            } else {
                return false;
            }
        }
        false
    }
    pub fn show_table(&self, name: &str,column:Vec<String>) -> ShowTable {
        for i in self.all_table.clone() {
            if i.clone().table_name() == name {
                let mut t = i.show_table(column.clone());
                t.table_name=name.to_string();
                return t;
            }
        }
        ShowTable::default()
    }


}
//for developer
impl BaseControl {
    pub fn check_database_selected(&self)->bool{
        self.db_select
    }
    pub fn check_initiate(&self)->bool{
        self.initiate_lock
    }
    pub fn set_db_true(&mut self){
        self.db_select=true;
    }
    pub fn set_db_false(&mut self){
        self.db_select=false;
    }
}
