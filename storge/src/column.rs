use avl::{AvlTreeSet};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use triadic_logic::datatype::{
    AttributeType, AttributeTypeValue, Date, Interval, Money, Time, TimeStamp,
};
use triadic_logic::degree::Degree;
use triadic_logic::tri_var::TriVar;
#[derive(Default, Clone, Debug)]
pub struct TraidicTree {
    pub avl_tree_set: AvlTreeSet<ForIndex>,
}
impl TraidicTree {
    pub fn check_avl_insert(&mut self, value: ForIndex) -> bool {
        if self.avl_tree_set.insert(value) {
            return true;
        }

        false
    }
}
// Implement Serialize for TraidicTree
impl Serialize for TraidicTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Convert AvlTreeSet<ForIndex> to a vector of ForIndex structs
        let index_vector: Vec<_> = self.avl_tree_set.iter().cloned().collect();
        index_vector.serialize(serializer)
    }
}

// Implement Deserialize for TraidicTree
impl<'de> Deserialize<'de> for TraidicTree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize the vector of ForIndex structs and convert it to AvlTreeSet<ForIndex>
        let index_vector: Vec<ForIndex> = Deserialize::deserialize(deserializer)?;
        let avl_tree_set: AvlTreeSet<ForIndex> = index_vector.into_iter().collect();
        Ok(TraidicTree { avl_tree_set })
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct ForIndex {
    pub value: Option<AttributeTypeValue>,
    pub index: usize,
}

impl Ord for ForIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for ForIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ForIndex {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for ForIndex {}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct PRIMARYKEY {
    pub primary_key: bool,
    pub degree: Option<Degree>,
}
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Unique {
    pub unique: bool,
    pub degree: Option<Degree>,
}
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Constraints {
    pub not_null: bool,
    pub unique: Unique,
    pub primary_key: PRIMARYKEY,
    pub foreign_key: bool,
    pub check: bool,
    pub check_operator: String,
    pub check_value: String,
    pub default: bool,
    pub default_value: String,
}

impl Constraints {
    pub fn new() -> Self {
        Self {
            not_null: false,
            unique: Unique::default(),
            primary_key: PRIMARYKEY::default(),
            foreign_key: false,
            check: false,
            check_operator: "".to_string(),
            check_value: "".to_string(),
            default: false,
            default_value: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column {
    name: String,
    pub type_status: AttributeType,
    size_status: usize,
    pub value: Vec<TriVar>,
    constraints: Constraints,
    column_order: usize,
    index_tree: Option<TraidicTree>,
}

impl Column {
    //noinspection ALL
    pub fn new(n: &str, t: &AttributeType, constraints: Constraints) -> Self {
        let temp = if constraints.primary_key.primary_key || constraints.unique.unique {
            Some(TraidicTree::default())
        } else {
            None
        };
        Self {
            name: n.to_string(),
            type_status: t.clone(), 
            size_status: 0,
            value: vec![],
            constraints,
            column_order: 0,
            index_tree: temp,
        }
    }
}
impl Column {
    pub fn set_bool_cell(&mut self, value: bool, d: Degree)->bool {
        if self.type_status == AttributeType::TBool {
            self.size_status += 1;
            self.value.push(TriVar::t_bool(value, d));
            return true
        }
        false
    }
    pub fn set_int_cell(&mut self, value: i32, degree: Degree)->bool {
        //datatype compare
        if self.type_status == AttributeType::TInt {
            //Here a primary key is need to check or not.
             if self.constraints.primary_key.primary_key || self.constraints.unique.unique{
                 //Now find primary key has degree or not.
                 //If not then
                 if let Some(primary_degree) = self.constraints.primary_key.degree {
                     if primary_degree != degree {
                         //println!("Degree different, so push value");
                         self.size_status += 1;
                         self.value.push(TriVar::t_int(value, degree));
                         return true;
                     }
                 }
                 if let Some(unique_degree) = self.constraints.unique.degree {
                     if unique_degree != degree {
                         //println!("Degree different, so push value");
                         self.size_status += 1;
                         self.value.push(TriVar::t_int(value, degree));
                         return true;
                     }
                 }

                 if let Some(tree) = self.index_tree.as_mut() {
                     let for_index = ForIndex {
                         value: Some(AttributeTypeValue::IntIng(value)),
                         index: self.size_status,
                     };

                     return if !tree.check_avl_insert(for_index) {
                         false
                     } else {
                         self.size_status += 1;
                         self.value.push(TriVar::t_int(value, degree));
                         true
                     }
                 }
             }
            else {
                self.size_status += 1;
                self.value.push(TriVar::t_int(value, degree));
                return true;
            }
            
            
        }
        false
    }

    pub fn set_small_int_cell(&mut self, value: i16, d: Degree) {
        if self.type_status == AttributeType::TSmallInt {
            self.size_status += 1;
            self.value.push(TriVar::t_small_int(value, d))
        }
    }
    pub fn set_big_int_cell(&mut self, value: i64, d: Degree) {
        if self.type_status == AttributeType::TBigInt {
            self.size_status += 1;
            self.value.push(TriVar::t_big_int(value, d))
        }
    }
    pub fn set_float_cell(&mut self, value: f64, d: Degree)->bool {
        if self.type_status == AttributeType::TFloat {
            if self.constraints.primary_key.primary_key || self.constraints.unique.unique{
                if let Some(primary_degree) = self.constraints.primary_key.degree {
                    if primary_degree != d {
                        //println!("Degree different, so push value");
                        self.size_status += 1;
                        self.value.push(TriVar::t_float(value, d));
                        return true;
                    }
                }
                if let Some(unique_degree) = self.constraints.unique.degree {
                    if unique_degree != d {
                        //println!("Degree different, so push value");
                        self.size_status += 1;
                        self.value.push(TriVar::t_float(value, d));
                        return true;
                    }
                }
                if let Some(tree) = self.index_tree.as_mut() {
                    let for_index = ForIndex {
                        value: Some(AttributeTypeValue::FloatIng(value)),
                        index: self.size_status,
                    };

                    return if !tree.check_avl_insert(for_index) {
                        false
                    } else {
                        self.size_status += 1;
                        self.value.push(TriVar::t_float(value, d));
                        true
                    }
                }
            }
            else {
                self.size_status += 1;
                self.value.push(TriVar::t_float(value, d));
                return true;
            }

        }
        false
    }
    pub fn set_char_cell(&mut self, value: char, d: Degree)->bool {
        if self.type_status == AttributeType::TChar {
            //println!("Inside of TChar");
            //println!("{}",self.constraints.primary_key.primary_key);
            if self.constraints.primary_key.primary_key || self.constraints.unique.unique{
                //println!("before the tree");
                if let Some(primary_degree) = self.constraints.primary_key.degree {
                    if primary_degree != d {
                        //println!("Degree different, so push value IN CHAR");
                        self.size_status += 1;
                        self.value.push(TriVar::t_char(value, d));
                        return true;
                    }
                }
                if let Some(unique_degree) = self.constraints.unique.degree {
                    if unique_degree != d {
                        //println!("Degree different, so push value");
                        self.size_status += 1;
                        self.value.push(TriVar::t_char(value, d));
                        return true;
                    }
                }
                if let Some(tree) = self.index_tree.as_mut() {
                    let for_index = ForIndex {
                        value: Some(AttributeTypeValue::CharacterIng(value)),
                        index: self.size_status,
                    };
                    //println!("Inside tree");
                    return if !tree.check_avl_insert(for_index) {
                        //println!("value not inserted in tree");
                        false
                    } else {
                        self.size_status += 1;
                        self.value.push(TriVar::t_char(value, d));
                        return true
                    }
                }
            }
            else {
                self.size_status += 1;
                self.value.push(TriVar::t_char(value, d));
                return true
            }

        }
        false
    }

    pub fn set_string_cell(&mut self, value: String, d: Degree)->bool {
        if self.constraints.not_null && (value=="NULL" || value=="null")  {
                return false;
        }
        return if self.type_status == AttributeType::TString {
            if self.constraints.primary_key.primary_key || self.constraints.unique.unique{
                if let Some(primary_degree) = self.constraints.primary_key.degree {
                    if primary_degree != d {
                        //println!("Degree different, so push value");
                        self.size_status += 1;
                        self.value.push(TriVar::t_string(value.clone(), d));
                        return true;
                    }
                }
                if let Some(unique_degree) = self.constraints.unique.degree {
                    if unique_degree != d {
                        //println!("Degree different, so push value");
                        self.size_status += 1;
                        self.value.push(TriVar::t_string(value, d));
                        return true;
                    }
                }
                if let Some(tree) = self.index_tree.as_mut() {
                    let for_index = ForIndex {
                        value: Some(AttributeTypeValue::Stringing(value.clone())),
                        index: self.size_status,
                    };

                    return if !tree.check_avl_insert(for_index) {
                        false
                    } else {
                        self.size_status += 1;
                        self.value.push(TriVar::t_string(value.clone(), d));
                        return true
                    }
                }
            }
            else {
                self.size_status += 1;
                self.value.push(TriVar::t_string(value.clone(), d));
                return true
            }
            true
        } else if self.type_status == AttributeType::TText {
            if self.constraints.primary_key.primary_key || self.constraints.unique.unique{
                if let Some(primary_degree) = self.constraints.primary_key.degree {
                    if primary_degree != d {
                        //println!("Degree different, so push value");
                        self.size_status += 1;
                        self.value.push(TriVar::t_text(value.clone(), d));
                        return true;
                    }
                }
                if let Some(unique_degree) = self.constraints.unique.degree {
                    if unique_degree != d {
                        //println!("Degree different, so push value");
                        self.size_status += 1;
                        self.value.push(TriVar::t_text(value, d));
                        return true;
                    }
                }
                if let Some(tree) = self.index_tree.as_mut() {
                    let for_index = ForIndex {
                        value: Some(AttributeTypeValue::Texting(value.clone())),
                        index: self.size_status,
                    };

                    return if !tree.check_avl_insert(for_index) {
                        false
                    } else {
                        self.size_status += 1;
                        self.value.push(TriVar::t_text(value.clone(), d));
                        return true
                    }
                }
            }
            else {
                self.size_status += 1;
                self.value.push(TriVar::t_text(value.clone(), d));
                return true
            }
            self.size_status += 1;
            self.value.push(TriVar::t_text(value.clone(), d));
            true
        }
        else {
            return false
        }

    }
    pub fn set_varchar_cell(&mut self, value: String, size: usize, d: Degree) {
        if self.type_status == AttributeType::TVarChar {
            self.size_status += 1;
            self.value.push(TriVar::t_varchar(value, size, d))
        }
    }
    pub fn set_date_cell(&mut self, value: Date, d: Degree) {
        if self.type_status == AttributeType::TDate {
            self.size_status += 1;
            self.value.push(TriVar::t_date(value, d))
        }
    }
    pub fn set_time_cell(&mut self, value: Time, d: Degree) {
        if self.type_status == AttributeType::TTime {
            self.size_status += 1;
            self.value.push(TriVar::t_time(value, d))
        }
    }
    pub fn set_timestamp_cell(&mut self, value: TimeStamp, d: Degree) {
        if self.type_status == AttributeType::TTimestamp {
            self.size_status += 1;
            self.value.push(TriVar::t_timestamp(value, d))
        }
    }
    pub fn set_interval_cell(&mut self, value: Interval, d: Degree) {
        if self.type_status == AttributeType::TInterval {
            self.size_status += 1;
            self.value.push(TriVar::t_interval(value, d))
        }
    }
    pub fn set_money_cell(&mut self, value: Money, d: Degree) {
        if self.type_status == AttributeType::TMoney {
            self.size_status += 1;
            self.value.push(TriVar::t_money(value, d))
        }
    }
}

impl Column {
    pub fn get_column_name(&self) -> &String {
        &self.name
    }

    pub fn get_column_data(&self, index: usize) -> Option<&TriVar> {
        self.value.get(index)
    }
    pub fn get_size(self) -> usize {
        self.size_status
    }
    pub fn get_column(&self) -> &Column {
        self
    }
}
