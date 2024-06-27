use crate::datatype::{AttributeTypeValue, Date, Interval, Money, Time, TimeStamp};
use crate::degree::Degree;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TriVar {
    value: Option<AttributeTypeValue>,
    status: Degree,
}
impl Ord for TriVar {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.status.cmp(&other.status) {
            Ordering::Equal => self.value.cmp(&other.value),
            other => other,
        }
    }
}

impl PartialOrd for TriVar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TriVar {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.status == other.status
    }
}

impl Eq for TriVar {}

impl TriVar {
    pub fn get_degree(&self) -> char {
        match self.status {
            Degree::T => {
                'T'
            }
            Degree::F => {
                'F'
            }
            Degree::L => {
                'L'
            }
        }
    }
    pub fn get_degree_ori(&self) -> Degree {
         self.status.clone()
    }
    pub fn t_bool(v: bool, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::BoolIng(v)),
            status: s,
        }
    }
    pub fn t_int(v: i32, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::IntIng(v)),
            status: s,
        }
    }
    pub fn t_small_int(v: i16, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::SmallINT(v)),
            status: s,
        }
    }
    pub fn t_big_int(v: i64, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::BigInt(v)),
            status: s,
        }
    }
    pub fn t_float(v: f64, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::FloatIng(v)),
            status: s,
        }
    }
    pub fn t_char(v: char, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::CharacterIng(v)),
            status: s,
        }
    }
    pub fn t_string(v: String, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::Stringing(v)),
            status: s,
        }
    }
    pub fn t_varchar(v: String, size: usize, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::VarCharacterIng(v, size)),
            status: s,
        }
    }
    pub fn t_text(v: String, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::Texting(v)),
            status: s,
        }
    }
    pub fn t_date(v: Date, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::Dating(v)),
            status: s,
        }
    }
    pub fn t_time(v: Time, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::Timing(v)),
            status: s,
        }
    }
    pub fn t_timestamp(v: TimeStamp, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::Timestamping(v)),
            status: s,
        }
    }
    pub fn t_interval(v: Interval, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::Intervaling(v)),
            status: s,
        }
    }
    pub fn t_money(v: Money, s: Degree) -> Self {
        Self {
            value: Option::from(AttributeTypeValue::Moneying(v)),
            status: s,
        }
    }
    pub fn get_attribute(&self) -> Option<&AttributeTypeValue> {
        self.value.as_ref()
    }

}
/*
impl fmt::Display for TriData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.value {
            None => {
                write!(f, "None:{}", self.status)
            }
            Some(value) => match value {
                AttributeTypeValue::BoolIng(_) => {
                    write!(f, "{}:{}",self.value , self.status)
                }
                AttributeTypeValue::IntIng(_) => {
                    write!(f, "{}:{}", value.get_int().unwrap(), self.status)
                }
                AttributeTypeValue::FloatIng(_) => {
                    write!(f, "{}:{}", value.get_float().unwrap(), self.status)
                }
                AttributeTypeValue::CharacterIng(_) => {
                    write!(f, "{}:{}", value.get_char().unwrap(), self.status)
                }
                AttributeTypeValue::Stringing(_) => {
                    write!(f, "{}:{}", value.get_string().unwrap(), self.status)
                }
                AttributeTypeValue::SmallINT(_) => {

                }
                AttributeTypeValue::BigInt(_) => {}
                AttributeTypeValue::VarCharacterIng(_, _) => {}
                AttributeTypeValue::Texting(_) => {}
                AttributeTypeValue::Dating(_) => {}
                AttributeTypeValue::Taming(_) => {}
                AttributeTypeValue::Timestamping(_) => {}
                AttributeTypeValue::Intervaling(_) => {}
                AttributeTypeValue::Moneying(_) => {}
            },
        }
    }
}


 */
