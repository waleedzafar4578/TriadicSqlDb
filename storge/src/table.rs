use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::string::String;

use serde::{Deserialize, Serialize};

use common_structure::{EqualOperator, GreaterEqualOperator, GreaterOperator, LessEqualOperator, LessOperator, NotEqualOperator, SelectEntry};
use triadic_logic::datatype::{AttributeType, AttributeTypeValue};
use triadic_logic::degree::Degree;

use crate::column::Column;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Table {
    table_name: String,
    table_column: Vec<Column>,
    global_index: usize,
}

impl Table {
    pub fn new(name: &str) -> Table {
        Self {
            table_name: name.to_string(),
            table_column: vec![],
            global_index: 0,
        }
    }
}
impl Table {
    pub fn get_table_name(&self) -> &String {
        &self.table_name
    }
    pub fn set_table_name(&mut self,new_name: &String){
        self.table_name= new_name.to_string()
    }
    pub fn truncate_table(&mut self){
        self.table_column.clear()
    }
    pub fn get_full_column(&self, name: String) -> Option<&Column> {
        for col in &self.table_column {
            if name == *col.get_column_name() {
                return Some(col.get_column());
            }
        }
        None
    }
}
impl Table {
    pub fn table_name(self) -> String {
        self.table_name
    }
    pub fn add_column(&mut self, col: Column) {
        self.table_column.push(col);
    }
    pub fn drop_column(&mut self, col_name: String)->bool {
        let mut j:i32=-1;
        for i in &mut self.table_column{
            if col_name==i.name{
                break;
            }
            j+=1;
        }
        if j>=0{
            j+=1;
            self.table_column.remove(j as usize);
            return  true
        }
        false
    }
    pub fn add_col_data(&mut self, n: &str, col: &str, d: Degree) -> bool {
        for i in &mut self.table_column {
            if i.clone().get_column_name() == &n.to_string() {
                if let Ok(value) = string_to_integer(col) {
                    if i.set_int_cell(value, d) {
                        self.global_index += 1;
                        return true;
                    }
                }
                if let Ok(value) = string_to_float(col) {
                    if i.set_float_cell(value, d) {
                        self.global_index += 1;
                        return true;
                    }
                }
                if let Ok(value) = string_to_char(col) {
                    if i.set_char_cell(value, d) {
                        self.global_index += 1;
                        return true;
                    }
                }
                if let Ok(value) = string_to_bool(col) {
                    if i.set_bool_cell(value, d) {
                        self.global_index += 1;
                        return true;
                    }
                }
                return if i.set_string_cell(col.to_string(), d) {
                    self.global_index += 1;
                    true
                } else {
                    false
                };
            }
        }
        false
    }
    pub fn add_recheck_data(&mut self) {
        for i in &mut self.table_column {
            loop {
                if self.global_index > i.clone().get_size() {
                    i.set_string_cell("NULL".to_string(), Degree::L);
                } else {
                    break;
                }
            }
        }
    }
}

pub fn string_to_integer(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse()
}
pub fn string_to_float(s: &str) -> Result<f64, std::num::ParseFloatError> {
    s.parse()
}
pub fn string_to_char(s: &str) -> Result<char, &'static str> {
    match s.chars().next() {
        Some(c) => Ok(c),
        None => Err("Empty String"),
    }
}
pub fn string_to_bool(s: &str) -> Result<bool, &'static str> {
    match s.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("Invalid boolean representation"),
    }
}

impl Table {
    pub fn save_to_file(self, path: String) {
        let file_name = path + &*self.table_name + ".json";
        let mut file = OpenOptions::new()
            .write(true)
            .read(false)
            .create(true)
            .truncate(true)
            .open(file_name)
            .expect("failed to open file!");
        let stream = serde_json::to_string_pretty(&self).unwrap();
        file.write_all(stream.as_bytes()).unwrap();
    }
    pub fn load_to_file(&mut self, path: String, file_name: String) -> Table {
        let p_file_name = path + &*file_name + ".json";
        let mut file = OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .truncate(false)
            .open(p_file_name)
            .expect("Failed to open file!");
        let mut stream = String::new();
        file.read_to_string(&mut stream).unwrap();
        let object: Table = serde_json::from_str(&stream).unwrap();
        object
    }
}
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ShowTable {
    pub table_name: String,
    pub column_name: Vec<String>,
    pub row: Vec<Vec<String>>,
}
impl Table {
    pub fn show_table(&self, info: &SelectEntry) -> ShowTable {
        //variable to container which uses for send data o the front end.
        let mut display_container = ShowTable::default();
        let mut row: Vec<String>;
        let mut size = 0;
        for i in &self.table_column {
            if info.column_name.is_empty() || info.column_name.contains(i.get_column_name()) {
                if size < i.clone().get_size() {
                    size = i.clone().get_size();
                }
                display_container
                    .column_name
                    .push(i.get_column_name().to_string());
            }
        }
        row = vec![];
        let mut j = 0;
        let mut index_container: Vec<usize> = vec![];
        match &info.where_clause {
            None => {
                while j < size {
                    for i in &self.table_column {
                        if info.column_name.is_empty()
                            || info.column_name.contains(i.get_column_name())
                        {
                            row.push(show_column(i, j));
                        }
                    }
                    display_container.row.push(row.clone());
                    row = vec![];
                    j += 1;
                }
            }
            Some(_condition) => {
                if let Some(_va)=&_condition.equal_operator{
                    index_container= self.where_index_equal(_va);
                }
                if let Some(_va)=&_condition.not_equal_operator{
                    index_container= self.where_index_not_equal(_va);
                }
                if let Some(_va)=&_condition.less_operator{
                    index_container= self.where_index_less(_va);
                }
                if let Some(_va)=&_condition.less_equal_operator{
                    index_container= self.where_index_less_eq(_va);
                }
                if let Some(_va)=&_condition.greater_operator{
                    index_container= self.where_index_greater(_va);
                }
                if let Some(_va)=&_condition.greater_equal_operator{
                    index_container= self.where_index_greater_eq(_va);
                }
                //println!("{:?}",index_container);
                row = vec![];

                for _ggt in &index_container{
                    for i in &self.table_column {
                        if info.column_name.is_empty()
                            || info.column_name.contains(i.get_column_name())
                        {
                            //row.push(show_column(i, index_container[*_ggt]));
                            //println!("{}:{}",i.clone().get_size(),_ggt);
                            //println!("{:?}",i.get_column_data(*_ggt));
                            let  val;
                            match i.get_column_data(*_ggt) {
                                None => val="None".to_string(),
                                Some(_f) => match _f.get_attribute() {
                                    None => {
                                        val=format!("None  :{}", _f.get_degree())
                                    }
                                    Some(_n) => {
                                        val=format!("{}   :{}", display_value(_n), _f.get_degree())
                                    }
                                }
                            }
                            row.push(val);
                        }
                    }
                    display_container.row.push(row.clone());
                    row = vec![];
                }
            }
        }

        display_container
    }

    pub fn where_index_equal(&self, _va: &EqualOperator) ->Vec<usize>{
        let mut index_container: Vec<usize> = vec![];
        for i in &self.table_column {
            if _va.column_name.eq(i.get_column_name()) {
                let mut index=0;
                for values in &i.value {
                    match values.get_attribute() {
                        None => {

                        }
                        Some(_b) => {
                            let t = display_value(_b);
                            //println!("Compare{}:{}",t,&_va.column_value);

                            match &_va.column_value {
                                None => {

                                    match &_va.degree {
                                        None => {
                                            index_container.push(index);
                                        }
                                        Some(_degree) => {
                                            if values.get_degree_ori() == *_degree {
                                                println!("{}:{}",t,_degree);
                                                index_container.push(index);
                                            }
                                        }
                                    }

                                }
                                Some(_compare_value) => {
                                    if t.eq(_compare_value) {
                                        match &_va.degree {
                                            None => {
                                                index_container.push(index);
                                            }
                                            Some(_degree) => {
                                                if values.get_degree_ori() == *_degree {
                                                    println!("{}:{}",t,_degree);
                                                    index_container.push(index);
                                                }
                                            }
                                        }
                                    }
                                }
                            }



                        }
                    }
                    index+=1;
                }
            }
        }
        index_container
    }
    pub fn where_index_not_equal(&self, _va: &NotEqualOperator) ->Vec<usize>{
        let mut index_container: Vec<usize> = vec![];
        for i in &self.table_column {
            if _va.column_name.eq(i.get_column_name()) {
                let mut index=0;
                for values in &i.value {
                    match values.get_attribute() {
                        None => {

                        }
                        Some(_b) => {
                            let t = display_value(_b);
                            //println!("Compare{}:{}",t,&_va.column_value);

                            match &_va.column_value {
                                None => {

                                    match &_va.degree {
                                        None => {
                                            index_container.push(index);
                                        }
                                        Some(_degree) => {
                                            if values.get_degree_ori() != *_degree {
                                                println!("{}:{}",t,_degree);
                                                index_container.push(index);
                                            }
                                        }
                                    }

                                }
                                Some(_compare_value) => {
                                    if !t.eq(_compare_value) {
                                        match &_va.degree {
                                            None => {
                                                index_container.push(index);
                                            }
                                            Some(_degree) => {
                                                if values.get_degree_ori() != *_degree {
                                                    println!("{}:{}",t,_degree);
                                                    index_container.push(index);
                                                }
                                            }
                                        }
                                    }
                                }
                            }



                        }
                    }
                    index+=1;
                }
            }
        }
        index_container
    }
    pub fn where_index_less(&self, _va: &LessOperator) ->Vec<usize>{
        let mut index_container: Vec<usize> = vec![];
        for i in &self.table_column {
            if _va.column_name.eq(i.get_column_name()) {

                let mut index=0;
                for values in &i.value {
                    match values.get_attribute() {
                        None => {

                        }
                        Some(_b) => {
                            let t = display_value(_b);
                            //println!("Compare{}:{}",t,&_va.column_value);

                            match &_va.column_value {
                                None => {

                                    match &_va.degree {
                                        None => {
                                            index_container.push(index);
                                        }
                                        Some(_degree) => {
                                            if values.get_degree_ori() == *_degree {
                                                println!("{}:{}",t,_degree);
                                                index_container.push(index);
                                            }
                                        }
                                    }

                                }
                                Some(_compare_value) => {
                                    let mut chk=false;
                                    match i.type_status{
                                        AttributeType::TInt => {
                                            if let Ok(vl)=string_to_integer(_compare_value){
                                                if let Ok(vl2)=string_to_integer(t.as_str()){
                                                    chk=vl2<vl;
                                                }
                                            }

                                        }
                                        AttributeType::TFloat => {
                                            if let Ok(vl)=string_to_float(_compare_value){
                                                if let Ok(vl2)=string_to_float(t.as_str()){
                                                    chk=vl2<vl;
                                                }
                                            }
                                        }

                                        _ => {}
                                    }

                                    if chk {
                                        match &_va.degree {
                                            None => {
                                                index_container.push(index);
                                            }
                                            Some(_degree) => {
                                                if values.get_degree_ori() == *_degree {
                                                    println!("{}:{}",t,_degree);
                                                    index_container.push(index);
                                                }
                                            }
                                        }
                                    }
                                }
                            }



                        }
                    }
                    index+=1;
                }
            }
        }
        index_container
    }
    pub fn where_index_greater(&self, _va: &GreaterOperator) ->Vec<usize>{
        let mut index_container: Vec<usize> = vec![];
        for i in &self.table_column {
            if _va.column_name.eq(i.get_column_name()) {

                let mut index=0;
                for values in &i.value {
                    match values.get_attribute() {
                        None => {

                        }
                        Some(_b) => {
                            let t = display_value(_b);
                            //println!("Compare{}:{}",t,&_va.column_value);

                            match &_va.column_value {
                                None => {

                                    match &_va.degree {
                                        None => {
                                            index_container.push(index);
                                        }
                                        Some(_degree) => {
                                            if values.get_degree_ori() == *_degree {
                                                println!("{}:{}",t,_degree);
                                                index_container.push(index);
                                            }
                                        }
                                    }

                                }
                                Some(_compare_value) => {
                                    let mut chk=false;
                                    match i.type_status{
                                        AttributeType::TInt => {
                                            if let Ok(vl)=string_to_integer(_compare_value){
                                                if let Ok(vl2)=string_to_integer(t.as_str()){
                                                    chk=vl2>vl;
                                                }
                                            }

                                        }
                                        AttributeType::TFloat => {
                                            if let Ok(vl)=string_to_float(_compare_value){
                                                if let Ok(vl2)=string_to_float(t.as_str()){
                                                    chk=vl2>vl;
                                                }
                                            }
                                        }

                                        _ => {}
                                    }

                                    if chk {
                                        match &_va.degree {
                                            None => {
                                                index_container.push(index);
                                            }
                                            Some(_degree) => {
                                                if values.get_degree_ori() == *_degree {
                                                    println!("{}:{}",t,_degree);
                                                    index_container.push(index);
                                                }
                                            }
                                        }
                                    }
                                }
                            }



                        }
                    }
                    index+=1;
                }
            }
        }
        index_container
    }
    pub fn where_index_less_eq(&self, _va: &LessEqualOperator) ->Vec<usize>{
        let mut index_container: Vec<usize> = vec![];
        for i in &self.table_column {
            if _va.column_name.eq(i.get_column_name()) {

                let mut index=0;
                for values in &i.value {
                    match values.get_attribute() {
                        None => {

                        }
                        Some(_b) => {
                            let t = display_value(_b);
                            //println!("Compare{}:{}",t,&_va.column_value);

                            match &_va.column_value {
                                None => {

                                    match &_va.degree {
                                        None => {
                                            index_container.push(index);
                                        }
                                        Some(_degree) => {
                                            if values.get_degree_ori() == *_degree {
                                                println!("{}:{}",t,_degree);
                                                index_container.push(index);
                                            }
                                        }
                                    }

                                }
                                Some(_compare_value) => {
                                    let mut chk=false;
                                    match i.type_status{
                                        AttributeType::TInt => {
                                            if let Ok(vl)=string_to_integer(_compare_value){
                                                if let Ok(vl2)=string_to_integer(t.as_str()){
                                                    chk=vl2<=vl;
                                                }
                                            }

                                        }
                                        AttributeType::TFloat => {
                                            if let Ok(vl)=string_to_float(_compare_value){
                                                if let Ok(vl2)=string_to_float(t.as_str()){
                                                    chk=vl2<=vl;
                                                }
                                            }
                                        }

                                        _ => {}
                                    }

                                    if chk {
                                        match &_va.degree {
                                            None => {
                                                index_container.push(index);
                                            }
                                            Some(_degree) => {
                                                if values.get_degree_ori() == *_degree {
                                                    println!("{}:{}",t,_degree);
                                                    index_container.push(index);
                                                }
                                            }
                                        }
                                    }
                                }
                            }



                        }
                    }
                    index+=1;
                }
            }
        }
        index_container
    }
    pub fn where_index_greater_eq(&self, _va: &GreaterEqualOperator) ->Vec<usize>{
        let mut index_container: Vec<usize> = vec![];
        for i in &self.table_column {
            if _va.column_name.eq(i.get_column_name()) {

                let mut index=0;
                for values in &i.value {
                    match values.get_attribute() {
                        None => {

                        }
                        Some(_b) => {
                            let t = display_value(_b);
                            //println!("Compare{}:{}",t,&_va.column_value);

                            match &_va.column_value {
                                None => {

                                    match &_va.degree {
                                        None => {
                                            index_container.push(index);
                                        }
                                        Some(_degree) => {
                                            if values.get_degree_ori() == *_degree {
                                                println!("{}:{}",t,_degree);
                                                index_container.push(index);
                                            }
                                        }
                                    }

                                }
                                Some(_compare_value) => {
                                    let mut chk=false;
                                    match i.type_status{
                                        AttributeType::TInt => {
                                            if let Ok(vl)=string_to_integer(_compare_value){
                                                if let Ok(vl2)=string_to_integer(t.as_str()){
                                                    chk=vl2>=vl;
                                                }
                                            }

                                        }
                                        AttributeType::TFloat => {
                                            if let Ok(vl)=string_to_float(_compare_value){
                                                if let Ok(vl2)=string_to_float(t.as_str()){
                                                    chk=vl2>=vl;
                                                }
                                            }
                                        }

                                        _ => {}
                                    }

                                    if chk {
                                        match &_va.degree {
                                            None => {
                                                index_container.push(index);
                                            }
                                            Some(_degree) => {
                                                if values.get_degree_ori() == *_degree {
                                                    println!("{}:{}",t,_degree);
                                                    index_container.push(index);
                                                }
                                            }
                                        }
                                    }
                                }
                            }



                        }
                    }
                    index+=1;
                }
            }
        }
        index_container
    }
}
pub fn show_column(i: &Column, j: usize) -> String {
    match i.get_column_data(j) {
        None => "None".to_string(),
        Some(_f) => match _f.get_attribute() {
            None => {
                format!("None  :{}", _f.get_degree())
            }
            Some(_n) => {
                format!("{}   :{}", display_value(_n), _f.get_degree())
            }
        },
    }
}
pub fn display_value(_n: &AttributeTypeValue) -> String {
    match _n {
        AttributeTypeValue::BoolIng(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::IntIng(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::SmallINT(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::BigInt(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::FloatIng(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::CharacterIng(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::Stringing(_b) => {
            format!("{}", _b)
        }
        AttributeTypeValue::Texting(_b) => {
            format!("{}", _b)
        }

        _ => "None".to_string(),
    }
}
