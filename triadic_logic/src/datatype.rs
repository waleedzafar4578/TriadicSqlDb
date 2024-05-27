//! This **DataType** module provides two things.
//! 
//! - AttributeType
//! 
//! 
//! This AttributeType
//! used for transfer column datatype information from compiler side to engine side.
//! 
//! - AttributeTypeValue
//! 
//! 
//! This AttributeTypeValue is used for actual store value that comes from the compiler.
//! 
//! ## Supported Datatypes
//! 
//! - Bool
//! - Int
//! - Float
//! - Char
//! - String
//! - Text

//! ## Upcoming Datatype
//! - SmallInt
//! - BigInt
//! - VarChar
//! - Date
//! - Time
//! - Timestamp
//! - Interval
//! - Money
//! Custom design Datatype
//! 
//! - Date
//! 
//! - Time
//! 
//! - Timestamp
//! 
//! - Interval
//! 
//!  




use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub enum AttributeType {
    TBool,
    TInt,
    TSmallInt,
    TBigInt,
    TFloat,
    TChar,
    TString,
    TVarChar,
    TText,
    TDate,
    TTime,
    TTimestamp,
    TInterval,
    TMoney,
}
#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub enum AttributeTypeValue {
    BoolIng(bool),
    IntIng(i32),
    SmallINT(i16),
    BigInt(i64),
    FloatIng(f64),
    CharacterIng(char),
    Stringing(String),
    VarCharacterIng(String,usize),
    Texting(String),
    Dating(Date),
    Timing(Time),
    Timestamping(TimeStamp),
    Intervaling(Interval),
    Moneying(Money)
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
    pub fn get_small_int(&self) -> Option<i16> {
        if let AttributeTypeValue::SmallINT(val) = self {
            Some(*val)
        } else {
            None
        }
    }
    pub fn get_big_int(&self) -> Option<i64> {
        if let AttributeTypeValue::BigInt(val) = self {
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
    pub fn get_varchar(&self) -> Option<(String,usize)> {
        if let AttributeTypeValue::VarCharacterIng(val,size) = self {
            Some((val.to_string(), *size))
        } else {
            None
        }
    }
    pub fn get_text(&self) -> Option<String> {
        if let AttributeTypeValue::Texting(val) = self {
            Some(val.to_string())
        } else {
            None
        }
    }
    pub fn get_date(&self) -> Option<Date> {
        if let AttributeTypeValue::Dating(val) = self {
            Some(val.clone())
        } else {
            None
        }
    }
    pub fn get_time(&self) -> Option<Time> {
        if let AttributeTypeValue::Timing(val) = self {
            Some(val.clone())
        } else {
            None
        }
    }
    pub fn get_timestamp(&self) -> Option<TimeStamp> {
        if let AttributeTypeValue::Timestamping(val) = self {
            Some(val.clone())
        } else {
            None
        }
    }
    pub fn get_interval(&self) -> Option<Interval> {
        if let AttributeTypeValue::Intervaling(val) = self {
            Some(val.clone())
        } else {
            None
        }
    }
    pub fn get_money(&self) -> Option<Money> {
        if let AttributeTypeValue::Moneying(val) = self {
            Some(val.clone())
        } else {
            None
        }
    }
}
///Custom design Date datatype
#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub struct Date;
///Custom design Time datatype
#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub struct Time;
///Custom design Timestamp datatype
#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub struct TimeStamp;
///Custom design interval datatype
#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub struct Interval;
///Custom design Money datatype
#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub struct Money;