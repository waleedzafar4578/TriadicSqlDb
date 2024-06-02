//! This **Degree** module provides Triadic truth values.
//! 
//! - True
//! - False
//! - Limit



use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Degree {
    ///Here T is represented status of value is true.
    T,
    ///Here F is represented status of value is false. 
    F,
    ///Here L is represented status of value is limit
    L,
}

impl Ord for Degree {
    fn cmp(&self, other: &Self) -> Ordering {
        // We can simply map each variant to an integer value for comparison
        // In this case, we can define the order T < F < L
        match (self, other) {
            (Degree::T, Degree::T) => Ordering::Equal,
            (Degree::T, _) => Ordering::Less,
            (Degree::F, Degree::F) => Ordering::Equal,
            (Degree::F, Degree::T) => Ordering::Greater,
            (Degree::F, Degree::L) => Ordering::Less,
            (Degree::L, Degree::L) => Ordering::Equal,
            (Degree::L, _) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Degree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Degree {}

impl PartialEq for Degree {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
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


