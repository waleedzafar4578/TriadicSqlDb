use crate::lexical::Token;
use crate::syntax::{AstNode, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_drop_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        self.advance();
        //checking the next token is DATABASE
        if let Some(Token::Keyword(ref next_keyword)) = self.tokens.get(self.current_token) {
            if next_keyword == "DATABASE" {
                self.advance();
                return self.parse_drop_database_statement();
            }
            else if next_keyword == "TABLE" {
                self.advance();
                return self.parse_drop_table_statement();
            }
        }
         (AstNode::Nothing, Some(triadic_error::Compiler::MissKeyword))
    }
    fn parse_drop_database_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        //here checking that next token is identifier
        if let Some(Token::Identifier(ref db_name)) = self.tokens.get(self.current_token) {
            self.advance();
            return if let Some(Token::Punctuation(';')) = self.tokens.get(self.current_token) {
                //Successfully parsed DROP DATABASE statement
                (AstNode::DropDatabaseStatement(db_name.clone()), None)
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
    fn parse_drop_table_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        //here checking that next token is identifier
        if let Some(Token::Identifier(ref db_name)) = self.tokens.get(self.current_token) {
            self.advance();
            return if let Some(Token::Punctuation(';')) = self.tokens.get(self.current_token) {
                //Successfully parsed DROP Table statement
                (AstNode::DropTableStatement(db_name.clone()), None)
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
