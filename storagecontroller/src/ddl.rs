use crate::BaseControl;
use std::fs;
use std::path::Path;
use storge::column::{Column, Constraints};
use storge::table::Table;
use triadic_error::engine_error::EngineError;
use triadic_logic::datatype::AttributeType;

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
            let temp_path = &(self.system_path.clone() + path);
            let new_file_path = Path::new(temp_path);
            if new_file_path.exists() {
                //println!("already exist");
                EngineError::AlreadyExist
            } else {
                self.database_name = path.to_string();
                self.db_select = true;
                self.all_table.clear();
                self.save_to_file();
                EngineError::IsCreated
            }
        } else {
            //println!("\n\n\nError:First  initiate the database\n\n");
            EngineError::PathNotSelected
        };
    }
    pub fn remove_the_database(&mut self, name: &str) -> EngineError {
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
    pub fn rename_the_database(&mut self, path_old: &str, path_new: &str) -> String {
        let old = self.system_path.to_string() + path_old + ".json";
        let new = self.system_path.to_string() + path_new + ".json";

        let new_file_path = Path::new(&old);
        return if new_file_path.exists() {
            match fs::rename(old, new) {
                Ok(_) => {
                    println!("File renamed successfully!");
                    self.database_name = path_new.to_string();
                    format!("Change database name {} to {}.", path_old, path_new)
                }
                Err(_e) => {
                    format!(
                        "Failed to change database name {} to {}.",
                        path_old, path_new
                    )
                }
            }
        } else {
            format!("{} database is found!", path_old)
        };
    }
    pub fn truncate_database(&mut self) {
        self.all_table.clear();
        self.save_to_file();
    }
    pub fn truncate_table(&mut self, name: String) -> bool {
        for mut _tb in &mut self.all_table {
            if _tb.get_table_name() == name.as_str() {
                _tb.truncate_table();
                return true;
            }
        }
        false
    }
    pub fn add_table(
        &mut self,
        t_name: &str,
        col_name: Vec<String>,
        col_type: Vec<(AttributeType, Constraints)>,
    ) -> String {
        //println!("Yes! Come in engine side.");
        return match self.db_select {
            true => {
                if self.search_table(t_name) {
                    return format!("{} this table is already in database!", t_name);
                }
                let mut tb: Table = Table::new(t_name);

                if col_name.len() == col_type.len() {
                    for (i, (j, c)) in col_name.iter().zip(col_type.iter()) {
                        tb.add_column(Column::new(i, j, c.clone()));
                    }
                    self.all_table.push(tb);
                    return format!("{} table is created in database!", t_name);
                }
                format!(
                    "Something wrong with this {}  \
                or his column or datatype",
                    t_name
                )
            }
            false => "Error: Please first select database!".to_string(),
        };
    }
    pub fn add_column_into_table(
        &mut self,
        t_name: &str,
        col_name: String,
        col_type: AttributeType,
    ) ->bool
    {
        for mut _tb in &mut self.all_table {
            if _tb.get_table_name() == t_name {
                _tb.add_column(Column::new(col_name.as_str(), &col_type, Default::default()));
                return true;
            }
        }
        false
    }
}
