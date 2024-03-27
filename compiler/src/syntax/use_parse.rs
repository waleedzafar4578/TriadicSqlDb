use crate::lexical::Token;
use crate::syntax::{AstNode, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_use_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        self.advance();
        //checking the next token is DATABASE
        if let Some(Token::Keyword(ref next_keyword)) = self.tokens.get(self.current_token) {
            if next_keyword == "DATABASE" {
                self.advance();
                return self.parse_use_database_statement();
            }
        }
         (AstNode::Nothing, Some(triadic_error::Compiler::Use))
    }
    fn parse_use_database_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        //here checking that next token is identifier
        if let Some(Token::Identifier(ref db_name)) = self.tokens.get(self.current_token) {
            self.advance();
            return if let Some(Token::Punctuation(';')) = self.tokens.get(self.current_token) {
                (AstNode::UseDatabaseStatement(db_name.clone()), None)
            } else {
                (
                    AstNode::Nothing,
                    Some(triadic_error::Compiler::UseDatabaseIdentifier),
                )
            }
        }
         (AstNode::Nothing, Some(triadic_error::Compiler::UseDatabase))
    }
}
