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
                (AstNode::Nothing, Some(triadic_error::Compiler::Rename))
            };
        }

         (AstNode::Nothing, None)
    }
    fn parse_rename_database_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        
        if let Some(Token::Identifier(ref _name1))=self.tokens.get(self.current_token) {
            self.advance();
            if self.tokens.get(self.current_token) == (Some(&Token::Punctuation(','))){
                self.advance()
            }
            if let Some(Token::Identifier(ref _name2))=self.tokens.get(self.current_token) {
                self.advance();
                if self.tokens.get(self.current_token) == (Some(&Token::Punctuation(';'))){
                   return  (AstNode::RenameDatabaseStatement(_name1.to_string(),_name2.to_string()),None)
                }
            }
            
        }
        
        
        /*
        // Check if the next token is an identifier (database name)
        if let Some(Token::Identifier(ref db_name)) = self.tokens.get(self.current_token) {
            self.advance(); // Move to the next token

            // Check if the next token is a semicolon
            return if let Some(Token::Punctuation(';')) = self.tokens.get(self.current_token) {
                // Successfully parsed a RENAME DATABASE statement
                (AstNode::RenameDatabaseStatement(db_name.clone(),db_name.clone()), None)
            } else {
                (
                    AstNode::Nothing,
                    Some(triadic_error::Compiler::RenameDatabaseIdentifier),
                )
            }
        }
        
         */
        (
            AstNode::Nothing,
            Some(triadic_error::Compiler::RenameDatabaseIdentifier),
        )
       
    }
    
         
        
}
