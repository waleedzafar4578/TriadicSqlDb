use crate::syntax::{AstNode, Parser, SelectEntry};

impl<'a> Parser<'a> {
    pub fn parse_select_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        let mut info_select: SelectEntry = SelectEntry::default();
        self.advance();
        if !self.check_operator("*") {
            match self.get_list_of_column() {
                None => {}
                Some(_list) => {
                    info_select.column_name = _list;
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
        if let Some(_table_name)=self.extract_identifier(){
            info_select.name=_table_name;
        }
        else {
            return (
                AstNode::Nothing,
                Some(triadic_error::Compiler::MissIdentifier),
            );
        }

        self.advance();
        if self.terminate_with_semicolon() {
            return (
                AstNode::SelectFullTable(info_select),
                Some(triadic_error::Compiler::Nothing),
            )

        }

        if !self.keyword_check("WHERE") {
            return (AstNode::Nothing, Some(triadic_error::Compiler::MissKeyword));
        }

        (
            AstNode::SelectFullTable(info_select),
            Some(triadic_error::Compiler::Nothing),
        )
    }
}
