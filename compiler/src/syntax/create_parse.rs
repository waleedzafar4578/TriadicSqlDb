use crate::lexical::{Token};
use crate::syntax::{AstNode, CompilerTableParseEntry, Parser};

use triadic_logic::datatype::AttributeType;
pub mod helping;
pub mod constraints;

impl<'a> Parser<'a> {
    pub(crate) fn parse_create_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        self.advance();
        //checking the next token is DATABASE
        if let Some(Token::Keyword(ref next_keyword)) = self.tokens.get(self.current_token) {
            if next_keyword == "DATABASE" {
                self.advance(); 
                return self.parse_create_database_statement();
            } else if next_keyword == "TABLE" {
                self.advance();
                return self.parse_create_table_statement();
            }
        }
        (AstNode::Nothing, Some(triadic_error::Compiler::NotAKeyword))
    }
    fn parse_create_database_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        //here checking that next token is identifier
        if let Some(Token::Identifier(ref db_name)) = self.tokens.get(self.current_token) {
            self.advance();
            return if let Some(Token::Punctuation(';')) = self.tokens.get(self.current_token) {
                //Successfully parsed CREATE a DATABASE statement
                (AstNode::CreateDatabaseStatement(db_name.clone()), None)
            } else {
                (
                    AstNode::Nothing,
                    Some(triadic_error::Compiler::MissSemicolon),
                )
            };
        }
        (
            AstNode::Nothing,
            Some(triadic_error::Compiler::MissIdentifier),
        )
    }
}






impl<'a> Parser<'a> {
    fn parse_create_table_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        //This mutable variable help to store things when walkthrough in token vector
        let mut table_creation_attributes_result: CompilerTableParseEntry =
            CompilerTableParseEntry {
                name: "".to_string(),
                column_name: vec![],
                type_plus_constraint: vec![],
            };
        //This if condition check that next token is identifier.
        //Then it means that is table name.
        //If it is not an identifier, then it panics and closes the app.
        if let Some(Token::Identifier(ref table_name)) = self.tokens.get(self.current_token) {
            table_creation_attributes_result.name.clone_from(table_name);
            self.advance();
        } else {
           return  (
                AstNode::Nothing,
                Some(triadic_error::Compiler::MissIdentifier),
            )
        }
        //
        //
        //This if condition checks the round open bracket, this means now columns and datatype.
        //println!("{:?}",self.tokens.get(self.current_token));
        self.open_bracket_check();
        self.advance();
        //
        //
        //
        if Some(&Token::Punctuation(')')) == self.tokens.get(self.current_token) {
            return  (
                AstNode::Nothing,
                Some(triadic_error::Compiler::MissColumn),
            )
        }

        //
        //
        //This while the loop is iterated in every column that break when found semicolon
        while self.tokens.get(self.current_token) != Some(&Token::Punctuation(';')) {
            //
            //
            //This condition is explained already.
            if self.tokens.get(self.current_token).is_none() {
                return  (
                    AstNode::Nothing,
                    Some(triadic_error::Compiler::MissSemicolon),
                )
            }
            if self.terminate_with_close_bracket_and_semicolon() {
                break;
            }
            //
            //
            //
            match self.extract_column_name() {
                Some(_name) => {
                    table_creation_attributes_result.column_name.push(_name);
                    //println!("Sys:Column Name:{}", _name);
                    self.advance();
                }
                None => {
                    return  (
                        AstNode::Nothing,
                        Some(triadic_error::Compiler::MissColumnName),
                    )
                }
            }
            
            
            let col_type:AttributeType;
            match self.datatype_checker() {
                None => {
                    return  (
                        AstNode::Nothing,
                        Some(triadic_error::Compiler::MissColumnDatatype),
                    )
                }
                Some(_type) => {
                    col_type=_type.clone();
                    //println!("Sys:Column Datatype::{:?}", _type);
                    self.advance();
                }
            }
            //
            //
            //
            match self.inline_column_constraints() {
                None => {}
                Some(_constrains) => {
                    table_creation_attributes_result.type_plus_constraint.push((col_type,_constrains));
                    //println!("System:Column Constraints:{:#?}", _constrains);
                }
            }
            //
            //
            //println!("End of first loop iteration:{:?}",self.tokens.get(self.current_token));
            //
            //
            //This advance function skips comma.
            self.advance();
        }

        //println!("{:#?}", table_creation_attributes_result);
        (
            AstNode::CreateTableStatement(table_creation_attributes_result),
            Some(triadic_error::Compiler::Nothing),
        )
    }

    
}
