//! This **Degree** module provides Triadic truth values.
//! 
//! - True
//! - False
//! - Limit



use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Debug, Clone, Copy,PartialEq)]
pub enum Degree {
    ///Here T is represented status of value is true.
    T,
    ///Here F is represented status of value is false. 
    F,
    ///Here L is represented status of value is limit
    L,
}

impl Default for Degree{
    ///This default function assign to created variable Limit(L).
    fn default() -> Self {
        Self::new()
    }
}
#[allow(dead_code)]
impl Degree {
    ///This new function assign to created variable Limit(L).
    pub fn new() -> Self {
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


