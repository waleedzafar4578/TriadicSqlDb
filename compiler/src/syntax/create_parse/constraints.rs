use storge::column::{Constraints, PRIMARYKEY};
use crate::lexical::{Token};
use crate::syntax::Parser;

pub mod parse_constraint;


impl<'a> Parser<'a>{
    pub(crate) fn inline_column_constraints(&mut self) -> Option<Constraints> {
        //
        //
        //
        let mut stock_of_constraints: Constraints = Constraints::new();
        //
        //
        //println!("Inline Constrains:{:?}", self.tokens.get(self.current_token));
        while self.tokens.get(self.current_token) != Some(&Token::Punctuation(',')) {
            //println!("Inline Constrains:{:?}", self.tokens.get(self.current_token));
            //this block of code breaks the loop when the current token is NONE
            //OR it found ");" bracket with semicolon
            if self.tokens.get(self.current_token).is_none()
                || self.terminate_with_close_bracket_and_semicolon()
            {
                break;
            }

            if let Some(Token::Keyword(ref _constrain)) = self.tokens.get(self.current_token) {

                match _constrain.as_str() {
                    "PRIMARY"=>{

                        match self.primary_key(_constrain.as_str()) {
                            None => {
                                println!("primary None");
                                return None;
                            }
                            Some(_data) => {
                                println!("primary in data");
                                stock_of_constraints.primary_key=_data;
                            }
                        }

                    }
                    "NOT"=>{
                        stock_of_constraints.not_null=self.not_null(_constrain.as_str());
                    }
                    "UNIQUE"=>{
                        stock_of_constraints.unique=self.unique(_constrain.as_str());
                    }
                    "CHECK" => {
                        (stock_of_constraints.check,stock_of_constraints.check_operator,stock_of_constraints.check_value)=self.check(_constrain.as_str());
                    }
                    "DEFAULT" => {
                        (stock_of_constraints.default,stock_of_constraints.default_value)=self.default(_constrain.as_str());
                    }
                    _ => {
                        //println!("----------:{:?}", self.tokens.get(self.current_token));
                    }
                }
            }
            self.advance();
        }
        Some(stock_of_constraints)
    }
}
