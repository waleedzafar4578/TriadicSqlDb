use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::{Read, Write};
//use serde_json::Value::String;
use crate::column::Column;
use std::string::String;
use triadic_logic::datatype::AttributeTypeValue;
use triadic_logic::degree::Degree;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Table {
    table_name: String,
    table_column: Vec<Column>,
    global_index: usize,
}

impl Table {
    pub fn new(name: &str) -> Table {
        Self {
            table_name: name.to_string(),
            table_column: vec![],
            global_index: 0,
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
    pub fn add_col_data(&mut self, n: &str, col: &str, d: Degree) -> bool {
        for i in &mut self.table_column {
            if i.clone().get_column_name() == &n.to_string() {
                if let Ok(value) = string_to_integer(col) {
                    if i.set_int_cell(value, d) {
                       self.global_index+=1;
                        return true;
                    }

                }
                if let Ok(value) = string_to_float(col) {
                    if i.set_float_cell(value, d){
                       self.global_index+=1;
                        return true;
                    }
                }
                if let Ok(value) = string_to_char(col) {
                    if i.set_char_cell(value, d){
                        self.global_index+=1;
                        return true;
                    }
                }
                if let Ok(value) = string_to_bool(col) {
                    if i.set_bool_cell(value, d){
                        self.global_index+=1;
                        return true;
                    }
                }
                return if i.set_string_cell(col.to_string(), d) {
                    self.global_index += 1;
                    true
                } else {
                    false
                }
            }
        }
        false
    }
    pub fn add_recheck_data(&mut self)  {
        for i in &mut self.table_column {
            loop {
                if self.global_index > i.clone().get_size(){
                    i.set_string_cell("NULL".to_string(),Degree::L);
                }
                else {
                    break;
                }
            }
        }

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
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ShowTable {
    pub table_name: String,
    pub column_name: Vec<String>,
    pub row: Vec<Vec<String>>,
}
impl Table {
    pub fn show_table(&self, column_display: Vec<String>) -> ShowTable {
        let mut display_constainer = ShowTable::default();
        let mut row: Vec<String>;
        let mut size = 0;
        for i in &self.table_column {
            if column_display.is_empty() || column_display.contains(i.get_column_name()) {
                if size < i.clone().get_size() {
                    size = i.clone().get_size();
                }
                display_constainer
                    .column_name
                    .push(i.get_column_name().to_string());
            }
        }
        row = vec![];
        let mut j = 0;
        while j < size {
            for i in &self.table_column {
                if column_display.is_empty() || column_display.contains(i.get_column_name()) {
                    row.push(show_column(i, j));
                }
            }
            display_constainer.row.push(row);
            row = vec![];
            j += 1;
        }

        display_constainer
    }
}
pub fn show_column(i: &Column, j: usize) -> String {
    match i.get_column_data(j) {
        None => "None".to_string(),
        Some(_f) => match _f.get_attribute() {
            None => {
                format!("None  :{}", _f.get_degree())
            }
            Some(_n) => {
                format!("{}   :{}", display_value(_n), _f.get_degree())
            }
        },
    }
}
pub fn display_value(_n: &AttributeTypeValue) -> String {
    match _n {
        AttributeTypeValue::BoolIng(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::IntIng(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::SmallINT(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::BigInt(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::FloatIng(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::CharacterIng(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::Stringing(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::Texting(_b) => {
            format!("{}", _b)
        }

        _ => "None".to_string(),
    }
}
