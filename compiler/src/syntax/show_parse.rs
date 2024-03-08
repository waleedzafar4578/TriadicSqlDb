use crate::lexical::Token;
use crate::syntax::{AstNode, Parser};
impl<'a> Parser<'a> {
    pub(crate) fn parse_show_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        self.advance(); // Move to the next token

        // Check if the next token is DATABASE
        if let Some(Token::Keyword(ref next_keyword)) = self.tokens.get(self.current_token) {
            return if next_keyword.to_uppercase() == "DATABASE" {
                self.advance(); // Move to the next token

                // Delegate to a separate function for parsing Search DATABASE
                self.parse_show_database_statement()
            } else {
                (AstNode::Nothing, Some(triadic_error::Compiler::Show))
            }
        }
        return (AstNode::Nothing, None);
    }

    fn parse_show_database_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        // Check if the next token is a semicolon
        if let Some(Token::Punctuation(';')) = self.tokens.get(self.current_token) {
            // Successfully parsed a show DATABASE statement
            return (
                AstNode::ShowDatabaseStatement,
                None,
            );
        }
        return (AstNode::Nothing, Some(triadic_error::Compiler::ShowDatabase));
    }
}
