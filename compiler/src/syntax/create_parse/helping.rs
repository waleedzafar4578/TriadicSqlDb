use triadic_logic::datatype::AttributeType;
use crate::lexical::Token;
use crate::syntax::Parser;

//helping function
impl<'a> Parser<'a> {
    pub(crate) fn terminate_with_close_bracket_and_semicolun(&mut self) -> bool {
        if self.tokens.get(self.current_token) == Some(&Token::Punctuation(')'))
            && self.tokens.get(self.current_token + 1) == Some(&Token::Punctuation(';'))
        {
            return true;
        }
        false
    }
    fn check_constrain(&mut self) {
        let (column, operator, value): (String, String, String);
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
    pub fn open_bracket_check(& self){
        if Some(&Token::Punctuation('(')) !=self.tokens.get(self.current_token){
            panic!("Opening Bracket missed!");
        }
    }
    pub fn close_bracket_check(& self){
        if Some(&Token::Punctuation(')')) !=self.tokens.get(self.current_token){
            panic!("Opening Bracket missed!");
        }
    }
}
