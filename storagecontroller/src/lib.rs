extern crate core;

use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::string::String;
use storge::table::Table;
pub mod ddl;
pub mod dml;
pub mod filing;
pub mod helping;
pub fn hello() {
    println!("Hello from storage controller")
}

#[derive(Serialize, Deserialize)]
pub struct BaseControl {
    system_path: String,
    database_name: String,
    db_select: bool,
    initiate_lock: bool,
    all_table: Vec<Table>,
}

impl fmt::Display for BaseControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "-----------------------------------------------------------"
        )
        .expect("Failed to show on console");
        writeln!(f, "Initiated Path: {}", self.system_path).expect("Failed to show on console");
        writeln!(f, "Select Database Name: {}", self.database_name)
            .expect("Failed to show on console");
        writeln!(f, "Lock value of Selected Database: {}", self.db_select)
            .expect("Failed to show on console");
        writeln!(f, "Lock value of initiate Path: {}", self.initiate_lock)
            .expect("Failed to show on console");
        writeln!(f).expect("Failed to show on console");
        if !self.all_table.is_empty() {
            for tb in &self.all_table {
                write!(f, "\n\n{}\n\n", tb).expect("Failed to show on console");
            }
        }
        Ok(())
    }
}
