use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum AttributeType {
    TBool,
    TInt,
    TFloat,
    TChar,
    TString,
}
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum AttributeTypeValue {
    BoolIng(bool),
    IntIng(i32),
    FloatIng(f64),
    CharacterIng(char),
    Stringing(String),
}

impl AttributeTypeValue {
    pub fn get_bool(&self) -> Option<bool> {
        if let AttributeTypeValue::BoolIng(val) = self {
            Some(*val)
        } else {
            None
        }
    }
    pub fn get_int(&self) -> Option<i32> {
        if let AttributeTypeValue::IntIng(val) = self {
            Some(*val)
        } else {
            None
        }
    }
    pub fn get_float(&self) -> Option<f64> {
        if let AttributeTypeValue::FloatIng(val) = self {
            Some(*val)
        } else {
            None
        }
    }
    pub fn get_char(&self) -> Option<char> {
        if let AttributeTypeValue::CharacterIng(val) = self {
            Some(*val)
        } else {
            None
        }
    }
    pub fn get_string(&self) -> Option<String> {
        if let AttributeTypeValue::Stringing(val) = self {
            Some(val.to_string())
        } else {
            None
        }
    }
}
