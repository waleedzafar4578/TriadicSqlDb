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
    CharacterIng(String),
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
    pub fn get_char(&self) -> Option<String> {
        if let AttributeTypeValue::CharacterIng(val) = self {
            Some(val.clone())
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

#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub struct Date;
#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub struct Time;
#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub struct TimeStamp;
#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub struct Interval;
#[derive(Serialize, Deserialize, Clone, PartialEq,Debug)]
pub struct Money;