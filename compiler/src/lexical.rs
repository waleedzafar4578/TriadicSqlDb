use std::fs::{File};
use std::io::Read;
use std::string::String;
use serde::Deserialize;
use std::fs::read_to_string;



#[derive(PartialEq, Debug)]
pub enum Literal {
    Numeric(String),
    String(String),
    Boolean(String),
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Literal(Literal),
    Operator(String),
    Punctuation(char),
}
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}
#[allow(dead_code)]
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
    }
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }
    fn advance(&mut self) {
        if let Some(ch) = self.peek() {
            self.position += ch.len_utf8()
        }
    }
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        identifier
    }
    fn read_numeric_literal(&mut self) -> String {
        let mut numeric_literal = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                numeric_literal.push(ch);
                self.advance()
            } else {
                break;
            }
        }

        numeric_literal
    }
    fn read_string_literal(&mut self) -> String {
        let mut string_literal = String::new();
        self.advance();
        while let Some(ch) = self.peek() {
            if ch == '\'' {
                self.advance();
                break;
            } else {
                string_literal.push(ch);
                self.advance();
            }
        }
        string_literal
    }
    fn read_boolean_literal(&mut self) -> String {
        let mut boolean_literal = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() {
                boolean_literal.push(ch);
                self.advance()
            } else {
                break;
            }
        }
        boolean_literal
    }
    fn read_operator(&mut self) -> String {
        let mut operator = String::new();
        while let Some(ch) = self.peek() {
            if "+_*/=&|".contains(ch) {
                operator.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        operator
    }
    fn read_punctuation(&mut self) -> char {
        if let Some(ch) = self.peek() {
            self.advance();
            ch
        } else {
            '\0'
        }
    }
    pub fn tokenize(&mut self) -> Vec<Token> {

        //let m: KeywordHolder =converter();
        //let ky=m.keyword.Clone();
       	
       
        //Define SQL keywords inside the tokenize function
        //let sql_keywords: Vec<&str>=ky.iter().map(|s| s.as_str()).collect();
        let sql_keywords:Vec<&str>=vec![
    "CREATE",
    "DROP",
    "SHOW",
    "TABLE",
    "DATABASE",
    "ALTER",
    "RENAME",
    "UPDATE",
    "SEARCH",
    "USE",
    "TRUNCATE",
    "INSERT",
    "DELETE",
    "SELECT",
    "FROM",
    "GRANT",
    "REVOKE",
    "ADDUSER",
    "CHECKUSER",
    "CONNECT"
  ];
        
        
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.skip_whitespace();
            } else if ch.is_alphabetic() || ch == '_' {
                let identifier = self.read_identifier();
                let token = if sql_keywords.contains(&identifier.to_uppercase().as_str()) {
                    Token::Keyword(identifier)
                } else {
                    match identifier.to_lowercase().as_str() {
                        "true" | "false" => {
                            Token::Literal(Literal::Boolean(identifier.parse().unwrap()))
                        }
                        _ => Token::Identifier(identifier),
                    }
                };
                tokens.push(token);
            } else if ch.is_ascii_digit() || ch == '.' {
                let numerical_literal = self.read_numeric_literal();
                tokens.push(Token::Literal(Literal::Numeric(numerical_literal)));
            } else if ch == '\'' {
                let string_literal = self.read_string_literal();
                tokens.push(Token::Literal(Literal::String(string_literal)));
            } else if "+-*/=<>&|".contains(ch) {
                let operator = self.read_operator();
                tokens.push(Token::Operator(operator));
            } else if ",;()".contains(ch) {
                let punctuation = self.read_punctuation();
                tokens.push(Token::Punctuation(punctuation));
            } else {
                self.advance();
            }
        }

        tokens
    }
}



#[derive(Deserialize, Debug)]
pub struct KeywordHolder {
    keyword: Vec<String>,
}
pub fn converter()-> KeywordHolder {
    //let  file = File::open("Syskeywords.json");
    let mut contents = r#"
    {
  "keyword": [
    "CREATE",
    "DROP",
    "SHOW",
    "TABLE",
    "DATABASE",
    "ALTER",
    "RENAME",
    "UPDATE",
    "SEARCH",
    "USE",
    "TRUNCATE",
    "INSERT",
    "DELETE",
    "SELECT",
    "FROM",
    "GRANT",
    "REVOKE",
    "ADDUSER",
    "CHECKUSER",
    "CONNECT"
  ]
}
    "#;
    read_to_string(&mut contents).expect("Failed to read data from file!!");
    serde_json::from_str(&contents).unwrap()
}
