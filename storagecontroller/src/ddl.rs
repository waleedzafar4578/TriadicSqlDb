use crate::BaseControl;
use std::fs;
use triadic_error::engine_error::{EngineErrorCreate, EngineErrorDrop};

impl BaseControl {
    pub fn create_the_database(&mut self, path: &str) -> EngineErrorCreate {
        if self.initiate_lock {
            /*
            Here cloning the value to temp variable which use for fs::create_dir_all
            Here question is why we use create_parse dir all instead of only create_parse dir?
            Create_parse dir=> /database(/database/subhuman school)
            (/database) => /database/school/subhuman for handling multiple folder hierarchy.
            Fs::create_parse function return ok if the folder is created vice versa.
            And e.kind help to identify the actual Error.
             */
            let temp = self.system_path.clone() + path;
            if let Err(e) = fs::create_dir_all(temp) {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    //println!("AlreadyExist");
                    return EngineErrorCreate::AlreadyExist;
                }
            } else if !BaseControl::find_this_database(self, path) {
                println!("Database is created!");
                return EngineErrorCreate::DoneYes;
            } else {
                //println!("AlreadyExist");
                return EngineErrorCreate::AlreadyExist;
            }
        } else {
            println!("\n\n\nError:First  initiate the database\n\n");
            return EngineErrorCreate::PathNotSelected;
        }
        EngineErrorCreate::PathNotSelected
    }
    pub fn remove_the_database(&mut self,name:&str) -> EngineErrorDrop {
        //Check if initiate lock is false then need to break the function
        if !self.initiate_lock {
            return EngineErrorDrop::PathNotSelected;
        }
        //Check if select lock is false then need
        // to break the function because if you want to remove,
        // a database
        //must be select first
        /*
        if !self.db_select {
            return false;
        }

         */
        let temp = self.system_path.clone() + name;
        match fs::remove_dir(temp.clone()) {
            Ok(()) => {
                println!("Folder '{}' removed successfully!", temp);
                self.db_select = false;
                self.database_name = "".to_string();
                 EngineErrorDrop::DoneYes
            }
            Err(e) => {
                println!("{e}");
                 EngineErrorDrop::NotFind
            },
        }

    }
    pub fn rename_the_database(&mut self, path_hold: &str, path_new: &str) -> bool {
        let old = self.system_path.to_string() + path_hold;
        let new = self.system_path.to_string() + path_new;

         match fs::rename(old, new) {
            Ok(_) => {
                println!("Directory renamed successfully!");
                self.database_name = path_new.to_string();
                true
            }
            Err(_e) => {
                println!("Failed to rename");
                false
            }
        }
    }
}
