use common_structure::SelectEntry;
use triadic_error::Compiler;
use crate::syntax::{AstNode, Parser};

impl<'a> Parser<'a> {
    pub fn parse_delete_statement(&mut self) -> (AstNode, Option<Compiler>) {
        let mut info_select: SelectEntry = SelectEntry::default();
        self.advance();

        if !self.keyword_check("FROM") {
            return (AstNode::Nothing, Some(Compiler::MissKeyword));
        }
        self.advance();
        if let Some(_table_name)=self.extract_identifier(){
            info_select.name=_table_name;
        }
        else {
            return (
                AstNode::Nothing,
                Some(Compiler::MissIdentifier),
            );
        }

        self.advance();
        if self.terminate_with_semicolon() {
            return (
                AstNode::DeleteTable(info_select),
                Some(Compiler::Nothing),
            )

        }

        return match self.condition_extract(&mut info_select) {
            None => {
                //println!("I am in where clause(None)");
                (
                    AstNode::DeleteTable(info_select),
                    Some(Compiler::Nothing),
                )
            }
            Some(_er) => {
                //println!("I am in where clause(Some(Err))");
                _er
            }
        }


    }
}