use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::OpenOptions;
use std::io::{Read, Write};
//use serde_json::Value::String;
use crate::column::Column;
use std::string::String;
use triadic_logic::degree::Degree;
#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Table {
    table_name: String,
    table_column: Vec<Column>,
}



impl Table {
    pub fn new(name: &str) -> Table {
        Self {
            table_name: name.to_string(),
            table_column: vec![],
        }
    }
}
impl Table {
    pub fn get_table_name(&self) -> &String {
        &self.table_name
    }
    pub fn get_full_column(&self, name: String) -> Option<&Column> {
        for col in &self.table_column {
            if name == *col.get_column_name() {
                return Some(col.get_column());
            }
        }
        None
    }
}
impl Table {
    pub fn table_name(self) -> String {
        self.table_name
    }
    pub fn add_column(&mut self, col: Column) {
        self.table_column.push(col);
    }
    pub fn add_col_data(&mut self, n: &str, col: &str, d: Degree) -> &mut Table {
        for i in &mut self.table_column {
            if i.clone().get_column_name() == &n.to_string() {
                if let Ok(value) = string_to_integer(col) {
                    i.set_int_cell(value, d);
                }
                if let Ok(value) = string_to_float(col) {
                    i.set_float_cell(value, d);
                }
                if let Ok(value) = string_to_char(col) {
                    //i.set_char_cell(value, d);
                }
                if let Ok(value) = string_to_bool(col) {
                    i.set_bool_cell(value, d);
                }
                i.set_string_cell(col.to_string(), d);
            }
        }
        self
    }
}

fn string_to_integer(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}
fn string_to_float(s: &str) -> Result<f64, std::num::ParseFloatError> {
    s.parse()
}
fn string_to_char(s: &str) -> Result<char, &'static str> {
    match s.chars().next() {
        Some(c) => Ok(c),
        None => Err("Empty String"),
    }
}
fn string_to_bool(s: &str) -> Result<bool, &'static str> {
    match s.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("Invalid boolean representation"),
    }
}
/*
impl fmt::Display for Table {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "TableName: {}", self.table_name)?;
        for col in self.table_column.clone() {
            write!(f, "{}     ", col.get_column_name())?;
        }
        let _ = writeln!(f, "{:?}", ..);

        //here find max size of column data
        let mut size: usize = 0;
        for col in self.table_column.clone() {
            if size < col.clone().get_size() {
                size = col.clone().get_size();
            }
        }
        let mut i: usize = 0;
        loop {
            if i >= size {
                break;
            }
            for col in &self.table_column.clone() {
                match col.get_column_data(i) {
                    None => {
                        write!(f, "None     ")?;
                    }
                    Some(_) => {
                        write!(f, "{}      ", col.get_column_data(i).unwrap().clone())?;
                    }
                }
            }
            i += 1;
            let _ = writeln!(f, "{:?}", ..);
        }
        Ok(())
    }
}

 */

impl Table {
    pub fn save_to_file(self, path: String) {
        let file_name = path + &*self.table_name + ".json";
        let mut file = OpenOptions::new()
            .write(true)
            .read(false)
            .create(true)
            .truncate(true)
            .open(file_name)
            .expect("failed to open file!");
        let stream = serde_json::to_string_pretty(&self).unwrap();
        file.write_all(stream.as_bytes()).unwrap();
    }
    pub fn load_to_file(&mut self, path: String, file_name: String) -> Table {
        let p_file_name = path + &*file_name + ".json";
        let mut file = OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .truncate(false)
            .open(p_file_name)
            .expect("Failed to open file!");
        let mut stream = String::new();
        file.read_to_string(&mut stream).unwrap();
        let object: Table = serde_json::from_str(&stream).unwrap();
         object
    }
}
