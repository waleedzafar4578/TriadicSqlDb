use crate::BaseControl;
use std::fs;
use triadic_error::engine_error::{EngineError};
use std::path::Path;
impl BaseControl {
    pub fn create_the_database(&mut self, path: &str) -> EngineError {
        return if self.initiate_lock {
            /*
            Here cloning the value to temp variable which use for fs::create_dir_all
            Here question is why we use create_parse dir all instead of only create_parse dir?
            Create_parse dir=> /database(/database/subhuman school)
            (/database) => /database/school/subhuman for handling multiple folder hierarchy.
            Fs::create_parse function return ok if the folder is created vice versa.
            And e.kind help to identify the actual Error.
             */
            let temp = &(self.system_path.clone() + path );
            let new_file_path = Path::new(temp);
            if new_file_path.exists() {
                //println!("already exist");
                EngineError::AlreadyExist
            } else {
                self.database_name = path.to_string();
                self.save_to_file();
                EngineError::IsCreated
            }
        } else {
            //println!("\n\n\nError:First  initiate the database\n\n");
            EngineError::PathNotSelected
        }
       
    }
    pub fn remove_the_database(&mut self,name:&str) -> EngineError {
        //Check if initiate lock is false then need to break the function
        if !self.initiate_lock {
            return EngineError::PathNotSelected;
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
        let temp = &(self.system_path.clone() + name + ".json");
        let new_file_path = Path::new(temp);
        if new_file_path.exists() {
            //println!("already exist");
            match fs::remove_file(temp) {
                Ok(_) => println!("File removed successfully."),
                Err(err) => println!("Error removing file: {}", err),
            }
            EngineError::IsRemove
        } else {
            EngineError::NotFind
        }

    }
    pub fn rename_the_database(&mut self, path_hold: &str, path_new: &str) -> bool {
        let old = self.system_path.to_string() + path_hold+".json";
        let new = self.system_path.to_string() + path_new+".json";

        
        let new_file_path = Path::new(&old);
        if new_file_path.exists() {
            match fs::rename(old, new) {
                Ok(_) => {
                    println!("File renamed successfully!");
                    self.database_name = path_new.to_string();
                    true
                }
                Err(_e) => {
                    println!("Failed to rename");
                    false
                }
            }
           
        } else {
            false
        }
         
    }
}
