use crate::lexical::Token;
use crate::syntax::{AstNode, Parser};
impl<'a> Parser<'a> {
    pub(crate) fn parse_search_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        self.advance(); // Move to the next token

        // Check if the next token is DATABASE
        if let Some(Token::Keyword(ref next_keyword)) = self.tokens.get(self.current_token) {
            return if next_keyword == "DATABASE" {
                self.advance(); // Move to the next token

                // Delegate to a separate function for parsing Search DATABASE
                self.parse_search_database_statement()
            } else {
                (AstNode::Nothing, Some(triadic_error::Compiler::MissKeyword))
            }
        }
         (AstNode::Nothing, None)
    }

    fn parse_search_database_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        // Check if the next token is an identifier (database name)
        if let Some(Token::Identifier(ref db_name)) = self.tokens.get(self.current_token) {
            self.advance(); // Move to the next token

            // Check if the next token is a semicolon
            return if let Some(Token::Punctuation(';')) = self.tokens.get(self.current_token) {
                // Successfully parsed a RENAME DATABASE statement
                (AstNode::SearchDatabaseStatement(db_name.clone()), None)
            } else {
                (
                    AstNode::Nothing,
                    Some(triadic_error::Compiler::MissSemicolon),
                )
            }
        }
         (
            AstNode::Nothing,
            Some(triadic_error::Compiler::MissIdentifier),
        )
    }
}
