use crate::BaseControl;
use std::fs;

impl BaseControl {
    pub fn create_the_database(&mut self, path: &str) -> bool {
        if self.initiate_lock == true {
            /*
            Here cloning the value to temp variable which use for fs::create_dir_all
            Here question is why we use create dir all instead of only create dir?
            create dir=> /database(/database/subhuman school)
            (/database) => /database/school/subhuman for handling multiple folder hierarchy.
            fs::create function return ok if the folder is created vice versa .
            And e.kind help to identify the actual Error.
             */
            let temp = self.system_path.clone() + path;
            if let Err(e) = fs::create_dir_all(temp) {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    println!("AlreadyExist");
                    return false;
                }
            } else {
                if BaseControl::find_this_database(self, path) == false {
                    //println!("Database is created!");
                    return true;
                } else {
                    //println!("AlreadyExist");
                    return false;
                }
            }
        } else {
            println!("\n\n\nError:First  initiate the database\n\n");
            return false;
        }
        return false;
    }
    pub fn remove_the_database(&mut self) -> bool {
        //Check if initiate lock is false then need to break the function
        if self.initiate_lock == false {
            return false;
        }
        //Check if select lock is false then need to break the function because if you want to remove database
        //must be select first
        if self.db_select == false {
            return false;
        }
        let temp = self.system_path.clone() + &*self.database_name;
        match fs::remove_dir(temp.clone()) {
            Ok(()) => {
                println!("Folder '{}' removed successfully!", temp);
                self.db_select = false;
                self.database_name = "".to_string();
                return true;
            }
            Err(e) => eprintln!("Error removing folder '{}': {}", temp, e),
        }

        return false;
    }
    pub fn rename_the_database(&mut self, path: &str) -> bool {
        let old = self.system_path.to_string() + &*self.database_name.to_string();
        let new = self.system_path.to_string() + path;

        return match fs::rename(old, new) {
            Ok(_) => {
                println!("Directory renamed successfully!");
                self.database_name = path.to_string();
                true
            }
            Err(_e) => {
                println!("Failed to rename");
                false
            }
        };
    }
}