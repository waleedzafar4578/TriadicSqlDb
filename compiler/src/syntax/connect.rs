use crate::lexical::Token;
use crate::syntax::{AstNode, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_adduser_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        self.advance();
        //checking the next token is DATABASE
        if let Some(Token::Identifier(ref db_name)) = self.tokens.get(self.current_token) {
            return (AstNode::AddUser(db_name.clone()), None);
        }
         (AstNode::Nothing, Some(triadic_error::Compiler::AddUser))
    }
    pub(crate) fn parse_checkuser_statement(
        &mut self,
    ) -> (AstNode, Option<triadic_error::Compiler>) {
        self.advance();
        //checking the next token is DATABASE
        if let Some(Token::Identifier(ref db_name)) = self.tokens.get(self.current_token) {
            return (AstNode::AddUser(db_name.clone()), None);
        }
         (AstNode::Nothing, Some(triadic_error::Compiler::CheckUser))
    }
}
