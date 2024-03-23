use crate::lexical::Token;
use crate::syntax::{AstNode, Parser};

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            current_token: 0,
        }
    }
    pub fn parse(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        //First check that first token is sql keyword
        if let Some(Token::Keyword(ref keyword)) = self.tokens.get(self.current_token) {
            match keyword.to_uppercase().as_str() {
                "ADDUSER" => self.parse_adduser_statement(),
                "CHECKUSER" => self.parse_checkuser_statement(),
                "CREATE" => self.parse_create_statement(),
                "DROP" => self.parse_drop_statement(),
                "USE" => self.parse_use_statement(),
                "SHOW" => self.parse_show_statement(),
                "RENAME" => self.parse_rename_statement(),
                "SEARCH" => self.parse_search_statement(),
                _ => (AstNode::Nothing, Some(triadic_error::Compiler::NotAKeyword)),
            }
        } else {
            println!("Unexpected token");
             (AstNode::Nothing, None)
        }
    }
    pub(crate) fn advance(&mut self) {
        self.current_token += 1;
    }
}
