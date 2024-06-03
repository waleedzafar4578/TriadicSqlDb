use crate::syntax::{AstNode, Parser};
use std::ptr::null;

impl<'a> Parser<'a> {
    pub fn parse_select_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        let mut columns: Vec<String> = vec![];
        self.advance();
        if !self.check_operator("*") {
            match self.get_list_of_column() {
                None => {}
                Some(_list) => {
                    columns = _list;
                }
            }
        } else {
            self.advance()
        }
        //println!("{:?}",self.tokens.get(self.current_token));

        if !self.keyword_check("FROM") {
            return (AstNode::Nothing, Some(triadic_error::Compiler::MissKeyword));
        }
        self.advance();
        match self.extract_identifier() {
            None => (
                AstNode::Nothing,
                Some(triadic_error::Compiler::MissIdentifier),
            ),
            Some(_table_name) => {
                self.advance();
                if self.terminate_with_semicolon() {
                    (
                        AstNode::SelectFullTable((columns, _table_name)),
                        Some(triadic_error::Compiler::Nothing),
                    )
                } else {
                    (
                        AstNode::Nothing,
                        Some(triadic_error::Compiler::MissSemicolon),
                    )
                }
            }
        }
    }
}
