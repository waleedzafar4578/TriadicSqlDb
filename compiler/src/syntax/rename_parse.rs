use crate::lexical::Token;
use crate::syntax::{AstNode, Parser};
impl<'a> Parser<'a> {
    pub(crate) fn parse_rename_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        self.advance(); // Move to the next token

        // Check if the next token is DATABASE
        if let Some(Token::Keyword(ref next_keyword)) = self.tokens.get(self.current_token) {
            return if next_keyword == "DATABASE" {
                self.advance(); // Move to the next token

                // Delegate to a separate function for parsing RENAME DATABASE
                self.parse_rename_database_statement()
            } else {
                (AstNode::Nothing, Some(triadic_error::Compiler::MissKeyword))
            };
        }

         (AstNode::Nothing, None)
    }
    fn parse_rename_database_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        return if let Some(Token::Identifier(ref _name1)) = self.tokens.get(self.current_token) {
            self.advance();
            if self.tokens.get(self.current_token) == (Some(&Token::Punctuation(','))) {
                self.advance()
            }
            if let Some(Token::Identifier(ref _name2)) = self.tokens.get(self.current_token) {
                self.advance();
                if self.tokens.get(self.current_token) == (Some(&Token::Punctuation(';'))) {
                    (AstNode::RenameDatabaseStatement(_name1.to_string(), _name2.to_string()), None)
                } else {
                    (
                        AstNode::Nothing,
                        Some(triadic_error::Compiler::MissSemicolon),
                    )
                }
            } else {
                (
                    AstNode::Nothing,
                    Some(triadic_error::Compiler::MissIdentifier),
                )
            }
        } else {
            (
                AstNode::Nothing,
                Some(triadic_error::Compiler::MissIdentifier),
            )
        }
        
       
       
    }
    
         
        
}
