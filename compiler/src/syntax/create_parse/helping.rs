use crate::lexical::{Literal, Token};
use crate::syntax::Parser;
use triadic_logic::datatype::AttributeType;

//helping function

impl<'a> Parser<'a> {
    pub(crate) fn terminate_with_close_bracket_and_semicolon(&mut self) -> bool {
        if self.tokens.get(self.current_token) == Some(&Token::Punctuation(')'))
            && self.tokens.get(self.current_token + 1) == Some(&Token::Punctuation(';'))
        {
            return true;
        }
        false
    }
    pub(crate) fn terminate_with_semicolon(&self) -> bool {
        if self.tokens.get(self.current_token) == Some(&Token::Punctuation(';')) {
            return true;
        }
        false
    }
    
    pub fn check_constrain(&mut self) {
       // let (column, operator, value): (String, String, String);
        if Some(&Token::Punctuation('(')) != self.tokens.get(self.current_token + 1) {
            panic!("Some this wrong with CHECK ..");
        }
        let _column: String;
        if let Some(Token::Identifier(ref __column)) = self.tokens.get(self.current_token + 2) {
            _column = __column.clone();
        }
    }
    pub(crate) fn datatype_checker(&self) -> Option<AttributeType> {
        if let Some(Token::Keyword(ref _datatype)) = self.tokens.get(self.current_token) {
            match _datatype.as_str() {
                "INT" => Some(AttributeType::TInt),
                "STRING" => Some(AttributeType::TString),
                "FLOAT" => Some(AttributeType::TFloat),
                "CHAR" => Some(AttributeType::TChar),
                "TEXT" => Some(AttributeType::TText),
                "BOOLEAN" => Some(AttributeType::TBool),
                &_ => Some(AttributeType::TTime),
            }
        } else {
            None
        }
    }

    pub(crate) fn extract_column_name(&self) -> Option<String> {
        let mut _tempory_column_name: String = String::new();
        //
        //
        //checking the Column name if not find then not needs to checking further.
        if let Some(Token::Identifier(ref column_name)) = self.tokens.get(self.current_token) {
            _tempory_column_name.clone_from(column_name);
        } else {
            return None;
        }
        Some(_tempory_column_name)
    }
    pub(crate) fn extract_identifier(&self) -> Option<String> {
        let mut _tempory_id: String = String::new();
        //
        //
        //checking the Column name if not find then not needs to checking further.
        if let Some(Token::Identifier(ref column_name)) = self.tokens.get(self.current_token) {
            _tempory_id.clone_from(column_name);
        } else {
            return None;
        }
        Some(_tempory_id)
    }
    pub fn open_bracket_check(&self) -> bool {
        if Some(&Token::Punctuation('(')) == self.tokens.get(self.current_token) {
            return true;
        }
        false
    }
    pub fn close_bracket_check(&self) -> bool {
        if Some(&Token::Punctuation(')')) == self.tokens.get(self.current_token) {
            return true;
        }
        false
    }
    pub fn comma_check(&self) -> bool {
        if Some(&Token::Punctuation(',')) == self.tokens.get(self.current_token) {
            return true;
        }
        false
    }
    pub fn keyword_check(&self, word: &str) -> bool {
        if let Some(Token::Keyword(ref next_keyword)) = self.tokens.get(self.current_token) {
            if next_keyword == word {
                return true;
            }
        }
        false
    }
    pub fn get_literal(&self) -> Option<String> {
        if let Some(Token::Literal(ref next_keyword)) = self.tokens.get(self.current_token) {
            return match next_keyword {
                Literal::Numeric(_d) => Some(_d.clone()),
                Literal::String(_d) => Some(_d.clone()),
                Literal::Boolean(_d) => Some(_d.clone()),
            };
        }
        if let Some(Token::Identifier( next_keyword)) = self.tokens.get(self.current_token) {
            return Some(next_keyword.to_string());
        }
        None
    }
    pub fn get_value_degree(&self) -> Option<char> {
        if let Some(Token::Keyword(ref next_keyword)) = self.tokens.get(self.current_token) {
            return match next_keyword.as_str() {
                "T" => Some('T'),
                "L" => Some('L'),
                "F" => Some('F'),
                _ => Some('L'),
            };
        }

        None
    }
    pub fn check_operator(&self, operator: &str) -> bool {
        return Some(&Token::Operator(operator.to_string())) == self.tokens.get(self.current_token)
    }
    pub fn get_list_of_values(&mut self) -> Option<Vec<(String,char)>> {
        //println!("Inside of list of column name");
        let mut column_data: Vec<(String,char)> = vec![];
        if !self.open_bracket_check() {
            return None;
        }
        self.advance();
        //println!("Open bracket find");
        while !self.close_bracket_check() {
            match self.get_literal() {
                None => {
                    //println!("");
                    return None;
                }
                Some(_column_value) => {
                    //println!("column name :{}",_column_name.clone());
                    self.advance();
                    match self.get_value_degree() {
                        None => {}
                        Some(_d) => {
                            //println!("{}:{}",_column_value,_d);
                            column_data.push((_column_value,_d));
                        }
                    }

                }
            }
            self.advance();
            match self.comma_check() {
                true => {
                    self.advance();
                }
                false => {
                    if self.close_bracket_check() {
                        self.advance();
                        return Some(column_data);
                    }
                    return None;
                }
            }
        }
        Some(column_data)
    }
    pub fn get_list_of_column(&mut self) -> Option<Vec<String>> {
        //println!("Inside of list of column name");

        let mut column_name: Vec<String> = vec![];
        if !self.open_bracket_check() {
            return None;
        }
        self.advance();

        //println!("Open bracket find");
        while !self.close_bracket_check() {
            match self.extract_identifier() {
                None => {
                    //println!("column name not finds");
                    return None;
                }
                Some(_column_name) => {
                   // println!("column name :{}",_column_name.clone());
                    column_name.push(_column_name);


                }
            }
            self.advance();
            match self.comma_check() {
                true => {
                    self.advance();
                }
                false => {
                    if self.close_bracket_check() {
                        self.advance();
                        return Some(column_name);
                    }
                    return None;
                }
            }
        }
        Some(column_name)
    }
    
}
