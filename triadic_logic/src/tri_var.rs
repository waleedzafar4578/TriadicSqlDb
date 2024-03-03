use std::fmt;
use std::fmt::Formatter;
use serde::{Serialize, Deserialize};
use crate::datatype::AttributeTypeValue;
use crate::degree::Degree;
#[derive(Serialize,Deserialize,Clone)]
pub struct TriData{
    value:Option<AttributeTypeValue>,
    status:Degree,
}
impl TriData{
    pub fn t_string(v:String,s:Degree)->Self{
        Self{
            value: Option::from(AttributeTypeValue::Stringing(v)),
            status: s,
        }
    }
    pub fn t_int(v:i32,s:Degree)->Self{
        Self{
            value: Option::from(AttributeTypeValue::IntIng(v)),
            status: s,
        }
    }
    pub fn t_float(v:f64,s:Degree)->Self{
        Self{
            value: Option::from(AttributeTypeValue::FloatIng(v)),
            status: s,
        }
    }
    pub fn t_char(v:char,s:Degree)->Self{
        Self{
            value: Option::from(AttributeTypeValue::CharacterIng(v)),
            status: s,
        }
    }
    pub fn t_bool(v:bool,s:Degree)->Self{
        Self{
            value: Option::from(AttributeTypeValue::BoolIng(v)),
            status: s,
        }
    }
    pub fn get_attribute(&self)->Option<&AttributeTypeValue>{
        self.value.as_ref()
    }
}

impl fmt::Display for TriData{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.value {
            None => {
                write!(f,"None:{}",self.status)
            }
            Some(value) => match value {
                AttributeTypeValue::BoolIng(_) => {
                    write!(f,"{}:{}",value.get_bool().unwrap(),self.status)
                }
                AttributeTypeValue::IntIng(_) => {
                    write!(f,"{}:{}",value.get_int().unwrap(),self.status)
                }
                AttributeTypeValue::FloatIng(_) => {
                    write!(f,"{}:{}",value.get_float().unwrap(),self.status)
                }
                AttributeTypeValue::CharacterIng(_) => {
                    write!(f,"{}:{}",value.get_char().unwrap(),self.status)
                }
                AttributeTypeValue::Stringing(_) => {
                    write!(f,"{}:{}",value.get_string().unwrap(),self.status)
                }
            }
        }
    }
}