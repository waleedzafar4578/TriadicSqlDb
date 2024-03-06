use crate::lexical::Token;
use crate::syntax::{AstNode, Parser};

impl <'a> Parser<'a>{
    pub(crate) fn parse_create_statement(&mut self)->(AstNode,Option<String>){
        self.advance();
        //checking the next token is DATABASE
        if let Some(Token::Keyword(ref next_keyword))=
            self.tokens.get(self.current_token){
            if next_keyword.to_uppercase() == "DATABASE" {
                self.advance();//Move to next token
                return self.parse_create_database_statement();
            }
        }
        return (AstNode::Nothing,None)
    }
    fn parse_create_database_statement(&mut self)->(AstNode,Option<String>){
        //here checking that next token is identifier
        if let Some(Token::Identifier(ref db_name))=
            self.tokens.get(self.current_token)
        {
            self.advance();
            if let Some(Token::Punctuation(';'))=self.tokens.get(self.current_token){
                //Successfully parsed CREATE DATABASE statement
                return (AstNode::CreateDatabaseStatement(db_name.clone()),Some(db_name.clone()))
            }
        }
        return (AstNode::Nothing,None)
    }
}