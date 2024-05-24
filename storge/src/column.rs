use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use triadic_logic::datatype::{AttributeType, Date, Interval, Money, Time, TimeStamp};
use triadic_logic::degree::Degree;
use triadic_logic::tri_var::TriData;

#[derive(Default,Serialize, Deserialize, Clone,Debug)]
pub struct Constraints {
    pub not_null: bool,
    pub unique: bool,
    pub primary_key: bool,
    pub foreign_key: bool,
    pub check: bool,
    pub check_operator: String,
    pub check_value: String,
    pub default: bool,
    pub default_value: String,
   
}

impl Constraints{
    pub fn new()->Self{
        Self{
            not_null: false,
            unique: false,
            primary_key: false,
            foreign_key: false,
            check: false,
            check_operator: "".to_string(),
            check_value: "".to_string(),
            default: false,
            default_value: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Column {
    name: String,
    type_status: AttributeType,
    size_status: usize,
    value: Vec<TriData>,
    constraints: Constraints,
    column_order:usize,
    
}

impl Column {
    //noinspection ALL
    pub fn new(n: &str, t: &AttributeType, constraints: Constraints) -> Self {
        Self {
            name: n.to_string(),
            type_status: t.clone(), //here some issue
            size_status: 0,
            value: vec![],
            constraints,
            column_order:0,
        }
    }
}
impl Column {
    pub fn set_bool_cell(&mut self, value: bool, d: Degree) {
        if self.type_status == AttributeType::TBool {
            self.size_status += 1;
            self.value.push(TriData::t_bool(value, d))
        }
    }
    pub fn set_int_cell(&mut self, value: i32, d: Degree) {
        if self.type_status == AttributeType::TInt {
            self.size_status += 1;
            self.value.push(TriData::t_int(value, d))
        }
    }
    pub fn set_small_int_cell(&mut self, value: i16, d: Degree) {
        if self.type_status == AttributeType::TSmallInt {
            self.size_status += 1;
            self.value.push(TriData::t_small_int(value, d))
        }
    }
    pub fn set_big_int_cell(&mut self, value: i64, d: Degree) {
        if self.type_status == AttributeType::TBigInt {
            self.size_status += 1;
            self.value.push(TriData::t_big_int(value, d))
        }
    }
    pub fn set_float_cell(&mut self, value: f64, d: Degree) {
        if self.type_status == AttributeType::TFloat {
            self.size_status += 1;
            self.value.push(TriData::t_float(value, d))
        }
    }
    pub fn set_char_cell(&mut self, value: String, d: Degree) {
        if self.type_status == AttributeType::TChar {
            self.size_status += 1;
            self.value.push(TriData::t_char(value, d))
        }
    }
   
    pub fn set_string_cell(&mut self, value: String, d: Degree) {
        if self.type_status == AttributeType::TString {
            self.size_status += 1;
            self.value.push(TriData::t_string(value, d))
        }
    }
    pub fn set_varchar_cell(&mut self, value: String,size:usize, d: Degree) {
        if self.type_status == AttributeType::TVarChar {
            self.size_status += 1;
            self.value.push(TriData::t_varchar(value,size, d))
        }
    }
    pub fn set_text_cell(&mut self, value: String, d: Degree) {
        if self.type_status == AttributeType::TText {
            self.size_status += 1;
            self.value.push(TriData::t_text(value, d))
        }
    }
    pub fn set_date_cell(&mut self, value: Date, d: Degree) {
        if self.type_status == AttributeType::TDate {
            self.size_status += 1;
            self.value.push(TriData::t_date(value, d))
        }
    }
    pub fn set_time_cell(&mut self, value: Time, d: Degree) {
        if self.type_status == AttributeType::TTime {
            self.size_status += 1;
            self.value.push(TriData::t_time(value, d))
        }
    }
    pub fn set_timestamp_cell(&mut self, value: TimeStamp, d: Degree) {
        if self.type_status == AttributeType::TTimestamp {
            self.size_status += 1;
            self.value.push(TriData::t_timestamp(value, d))
        }
    }
    pub fn set_interval_cell(&mut self, value: Interval, d: Degree) {
        if self.type_status == AttributeType::TInterval {
            self.size_status += 1;
            self.value.push(TriData::t_interval(value, d))
        }
    }
    pub fn set_money_cell(&mut self, value: Money, d: Degree) {
        if self.type_status == AttributeType::TMoney {
            self.size_status += 1;
            self.value.push(TriData::t_money(value, d))
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
        self
    }
}
/*
impl fmt::Display for Column {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.name)?;
        for data in &self.value {
            writeln!(f, "{}", data)?;
        }
        Ok(())
    }
}

 */
impl Column {
   
    
}

fn preprocess_for_value(data:Vec<TriData>,point:TriData)->bool{
    
    let _ = data.contains(&point);
    false
}