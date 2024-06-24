use storge::column::{PRIMARYKEY, Unique};
use triadic_logic::degree::Degree;
use crate::lexical::{Literal, Token};
use crate::syntax::Parser;

impl<'a> Parser<'a> {
    pub fn primary_key(&mut self, start_constraint: &str) -> Option<PRIMARYKEY> {
        println!("coming:{}", start_constraint);
        let mut temp_primary_object=PRIMARYKEY::default();
        
        if start_constraint != "PRIMARY" {
            return None;
        }
        self.advance();
        if let Some(Token::Keyword(ref inner_constrain)) =
            self.tokens.get(self.current_token)
        {
            if inner_constrain.as_str() == "KEY" {
                //println!("System: primary key found");
                temp_primary_object.primary_key=true;
            }
            else {
                return None;
            }

        }
        else {
            return None;
        }
        if Some(&Token::Punctuation('(')) != self.tokens.get(self.current_token+1) {
            return Some(temp_primary_object);
        }
        else { 
            self.advance()
        }
        self.advance();
        if let Some(Token::Keyword(ref inner_constrain)) =
            self.tokens.get(self.current_token)
        {
           
            match inner_constrain.as_str() {
                "T"=>temp_primary_object.degree=Some(Degree::T),
                "F"=>temp_primary_object.degree=Some(Degree::F),
                "L"=>temp_primary_object.degree=Some(Degree::L),
                &_ => {
                    
                }
            }

        }
        else { 
            return None;
        }
        self.advance();
        if !self.close_bracket_check() {
            return None;
        }
        
        Some(temp_primary_object)
    }
    pub fn not_null(&mut self, start_constraint: &str) -> bool {
        //println!("coming:{}", start_constraint);
        if start_constraint == "NOT" {
            if let Some(Token::Keyword(ref inner_constrain)) =
                self.tokens.get(self.current_token + 1)
            {
                if inner_constrain.as_str() == "NULL" {
                    self.advance();
                    //println!("System: Not null found");
                    return true;
                }
            }
        }
        false
    }
    pub fn unique(&mut self, start_constraint: &str) -> Option<Unique> {
        println!("coming:{}", start_constraint);
        let mut temp_primary_object=Unique::default();

        if start_constraint != "UNIQUE" {
            return None;
        }
        temp_primary_object.unique=true;
        //self.advance();
        println!("{:?}",self.tokens.get(self.current_token));
        if Some(&Token::Punctuation('(')) != self.tokens.get(self.current_token+1) {
            return Some(temp_primary_object);
        }
        else {
            self.advance()
        }
        self.advance();

        if let Some(Token::Keyword(ref inner_constrain)) =
            self.tokens.get(self.current_token)
        {

            match inner_constrain.as_str() {
                "T"=>temp_primary_object.degree=Some(Degree::T),
                "F"=>temp_primary_object.degree=Some(Degree::F),
                "L"=>temp_primary_object.degree=Some(Degree::L),
                &_ => {

                }
            }

        }
        else {
            return None;
        }
        self.advance();
        if !self.close_bracket_check() {
            return None;
        }

        Some(temp_primary_object)
    }
    pub fn check(&mut self,start_constraint: &str)->(bool,String,String){
        if start_constraint == "CHECK" {
            let t1:bool=true;
            let mut t2:String=String::new();
            let mut t3:String=String::new();
            
            self.advance();
            self.open_bracket_check();

            self.advance();
            if let Some(Token::Identifier(ref colname)) =self.tokens.get(self.current_token){

                format!("Sys:column name:{}",colname);
            }
            else {
                panic!("column name missed!");
            }
            self.advance();
            if let Some(Token::Operator(ref operater)) =self.tokens.get(self.current_token){
                t2.clone_from(operater);
                //println!("Sys:operator :{}",operater);
            }
            else{
                panic!("operator missed!");
            }
            self.advance();
            if let Some(Token::Literal(ref value)) =self.tokens.get(self.current_token){
                match value {
                    Literal::Numeric(_d) => {
                        t3.clone_from(_d);
                    }
                    Literal::String(_d) => {
                        t3.clone_from(_d);
                    }
                    Literal::Boolean(_d) => {
                        t3.clone_from(_d);
                    }
                }
                //println!("Sys:value{:?}",value);
            }
            else{
                panic!("Value missed!");
            }

            self.advance();
            self.close_bracket_check();
            return (t1,t2,t3);
        }
        (false,"".to_string(),"".to_string())
    }
    pub fn default(&mut self,start_constraint: &str)->(bool,String){
        let t1:bool;
        let mut t2:String=String::new();
        if start_constraint =="DEFAULT" {
            if let Some(Token::Identifier(ref inner_value)) =
                self.tokens.get(self.current_token + 1)
            {
                t1 = true;
                t2.clone_from(inner_value);
                self.advance();
                //println!("System: NOT NULL found");
                return (t1,t2);
            }
        }
        (false,"".to_string())
    }
   
}
