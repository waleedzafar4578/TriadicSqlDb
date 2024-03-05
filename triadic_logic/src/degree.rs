use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Degree {
    //Here T is represented status of value is true
    //Here F is represented status of value is false
    //Here L is represented status of value is limit
    T,
    F,
    L,
}
#[allow(dead_code)]
impl Degree {
    pub fn default() -> Self {
        Degree::L
    }
}
impl fmt::Display for Degree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let display_str = match self {
            Degree::T => "T",
            Degree::F => "F",
            Degree::L => "L",
        };
        write!(f, "{}", display_str)
    }
}

pub fn hello() {
    println!("Hello from triadic_logic side");
}
