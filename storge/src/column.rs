use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Formatter};
use triadic_logic::datatype::AttributeType;
use triadic_logic::degree::Degree;
use triadic_logic::tri_var::TriData;

#[derive(Serialize, Deserialize,Clone)]
pub struct Column {
    name: String,
    type_status: AttributeType,
    size_status: usize,
    value: Vec<TriData>,
}

impl Column {
    //noinspection ALL
    pub fn new(n: &str, t: AttributeType) -> Self {
        Self {
            name: n.to_string(),
            type_status: t, //here some issue
            size_status: 0,
            value: vec![],
        }
    }
}
impl Column {
    pub fn set_int_cell(&mut self, value: i32, d: Degree) {
        match self.type_status {
            AttributeType::TBool => {}
            AttributeType::TInt => {
                self.size_status+=1;
                self.value.push(TriData::t_int(value,d))
            }
            AttributeType::TFloat => {}
            AttributeType::TChar => {}
            AttributeType::TString => {}
        }
    }
    pub fn set_float_cell(&mut self, value: f64, d: Degree) {
        match self.type_status {
            AttributeType::TBool => {}
            AttributeType::TInt => {}
            AttributeType::TFloat => {
                self.size_status+=1;
                self.value.push(TriData::t_float(value,d))
            }
            AttributeType::TChar => {}
            AttributeType::TString => {}
        }
    }
    pub fn set_char_cell(&mut self, value: char, d: Degree) {
        match self.type_status {
            AttributeType::TBool => {}
            AttributeType::TInt => {}
            AttributeType::TFloat => {}
            AttributeType::TChar => {
                self.size_status+=1;
                self.value.push(TriData::t_char(value,d))
            }
            AttributeType::TString => {}
        }
    }
    pub fn set_bool_cell(&mut self, value: bool, d: Degree) {
        match self.type_status {
            AttributeType::TBool => {
                self.size_status+=1;
                self.value.push(TriData::t_bool(value,d))
            }
            AttributeType::TInt => {}
            AttributeType::TFloat => {}
            AttributeType::TChar => {}
            AttributeType::TString => {}
        }
    }
    pub fn set_string_cell(&mut self, value: String, d: Degree) {
        match self.type_status {
            AttributeType::TBool => {}
            AttributeType::TInt => {}
            AttributeType::TFloat => {}
            AttributeType::TChar => {}
            AttributeType::TString => {
                self.size_status+=1;
                self.value.push(TriData::t_string(value,d))
            }
        }
    }

}

impl Column {
    pub fn get_column_name(&self) -> &String {
        &self.name
    }
    pub fn get_column_data(&self, index: usize) -> Option<&TriData> {
        self.value.get(index)
    }
    pub fn get_size(self) -> usize {
        self.size_status
    }
    pub fn get_column(&self) -> &Column {
        &self
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        for data in &self.value {
            writeln!(f, "{}", data)?;
        }
        Ok(())
    }
}
