use std::fmt;
use std::fmt::Formatter;
use serde::{Serialize, Deserialize};
use triadic_logic::degree::Degree;
use crate::column::Column;

#[derive(Serialize,Deserialize)]
pub struct Table{
     table_name:String,
     table_column:Vec<Column>,
 }

impl Table{
    pub fn new(name:&str)->Table{
        Self{
            table_name: name.to_string(),
            table_column: vec![],
        }
    }

}
impl Table{
    pub fn get_table_name(&self)->&String{
        &self.table_name
    }
    pub fn get_full_column(&self,name:String)->Option<&Column>{
        for col in &self.table_column{
            if name == *col.get_column_name(){
                return Some(col.get_column())
            }
        }
        None
    }

}
impl Table{
    pub fn table_name(self)->String{
        self.table_name
    }
    pub fn add_column(&mut self,col:Column){
        self.table_column.push(col);
    }
    pub fn add_col_data(&mut self, n:&str ,col:&str,d:Degree)->&mut Table{
        for i in &mut self.table_column{
            if *i.clone().get_column_name() == n.to_string(){
                match string_to_integer(col) {
                    Ok(value) => {i.set_int_cell(value,d);}
                    _ =>{}
                }
                match string_to_float(col) {
                    Ok(value) => {i.set_float_cell(value,d);}
                    _ =>{}
                }
                match string_to_char(col) {
                    Ok(value) => {i.set_char_cell(value,d);}
                    _ =>{}
                }
                match string_to_bool(col) {
                    Ok(value) => {i.set_bool_cell(value,d);}
                    _ =>{}
                }
                i.set_string_cell(col.to_string(),d);
            }
        }
        return self;
    }
}

fn string_to_integer(s:&str)->Result<i32,std::num::ParseIntError>{
    s.parse()
}
fn string_to_float(s:&str)->Result<f64,std::num::ParseFloatError>{
    s.parse()
}
fn string_to_char(s:&str)->Result<char,&'static str>{
    match s.chars().next() {
        Some(c)=>Ok(c),
        None=>Err("Empty String"),
    }
}
fn string_to_bool(s:&str)->Result<bool,&'static str>{
    match s.to_lowercase().as_str() {
        "true"=>Ok(true),
        "false"=>Ok(false),
        _ =>Err("Invalid boolean representation"),
    }
}



impl fmt::Display for Table{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f,"TableName: {}",self.table_name)?;
        for col in self.table_column.clone(){
            write!(f,"{}     ",col.get_column_name())?;
        }
        println!();

        //here find max size of column data
        let mut size:usize=0;
        for col in self.table_column.clone(){
            if size < col.clone().get_size(){
                size=col.clone().get_size();
            }
        }
        let mut i:usize=0;
        loop{
            if i>=size{
                break;
            }
            for col in &self.table_column.clone(){
                match col.get_column_data(i) {
                    None => {
                        write!(f,"None     ")?;
                    }
                    Some(_) => {
                        write!(f,"{}      ",col.get_column_data(i).unwrap().clone())?;
                    }
                }
            }
            i+=1;
            println!();
        }
        Ok(())
    }
}