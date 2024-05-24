use crate::lexical::{Literal, Token};
use crate::syntax::Parser;

impl<'a> Parser<'a> {
    pub fn primary_key(&mut self, start_constraint: &str) -> bool {
        //println!("coming:{}", start_constraint);
        if start_constraint == "PRIMARY" {
            if let Some(Token::Keyword(ref inner_constrain)) =
                self.tokens.get(self.current_token + 1)
            {
                if inner_constrain.as_str() == "KEY" {
                    self.advance();
                    //println!("System: primary key found");
                    return true;
                }
            }
        }
        false
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
    pub fn unique(&mut self, start_constraint: &str) -> bool {
        if start_constraint == "UNIQUE" {
           // println!("System:Unique constrain found.");
            return true;
        }
        false
    }
    pub fn check(&mut self,start_constraint: &str)->(bool,String,String){
        if start_constraint == "CHECK" {
            let mut t1:bool;
            let mut t2:String=String::new();
            let mut t3:String=String::new();
            t1=true;
            self.advance();
            self.open_bracket_check();

            self.advance();
            if let Some(Token::Identifier(ref colname)) =self.tokens.get(self.current_token){

                //println!("Sys:column name:{}",colname);
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
