use crate::syntax::{AlterTableData, AstNode, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_alter_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        let mut table_creation_attributes_result: AlterTableData =AlterTableData::default();

        self.advance();

        if !self.keyword_check("TABLE") {
            return (AstNode::Nothing, Some(triadic_error::Compiler::MissKeyword))
        }
        self.advance();

        match self.extract_identifier() {
            None => {
                return  (
                    AstNode::Nothing,
                    Some(triadic_error::Compiler::MissIdentifier),
                )
            }
            Some(_name) => {
                table_creation_attributes_result.name.clone_from(&_name);
            }
        }

        self.advance();

        return if self.keyword_check("ADD") {
            self.advance();

            match self.extract_identifier() {
                None => {
                    return (
                        AstNode::Nothing,
                        Some(triadic_error::Compiler::MissIdentifier),
                    )
                }
                Some(_name) => {
                    table_creation_attributes_result.column_name.clone_from(&_name);
                }
            }
            self.advance();

            match self.datatype_checker() {
                None => {
                    return (
                        AstNode::Nothing,
                        Some(triadic_error::Compiler::MissColumnDatatype),
                    )
                }
                Some(_data_type) => {
                    table_creation_attributes_result.data_type = _data_type;
                }
            }
            self.advance();

            match self.terminate_with_semicolon() {
                true => {
                    (
                        AstNode::AlterTableAddStatement(table_creation_attributes_result),
                        Some(triadic_error::Compiler::Nothing),
                    )
                }
                false => {
                    (
                        AstNode::Nothing,
                        Some(triadic_error::Compiler::MissSemicolon),
                    )
                }
            }
        } else if self.keyword_check("DROP") {

            
            self.advance();
            if !self.keyword_check("COLUMN") {
                return (
                    AstNode::Nothing,
                    Some(triadic_error::Compiler::MissKeyword),
                )
            }
            self.advance();
            self.show_current_token("after COLUMN");
            match self.extract_identifier() {
                None => {
                    return (
                        AstNode::Nothing,
                        Some(triadic_error::Compiler::MissIdentifier),
                    )
                }
                Some(_name) => {
                    table_creation_attributes_result.column_name.clone_from(&_name);
                }
            }
            self.advance();

            match self.terminate_with_semicolon() {
                true => {
                    (
                        AstNode::AlterTableDropStatement(table_creation_attributes_result),
                        Some(triadic_error::Compiler::Nothing),
                    )
                }
                false => {
                    (
                        AstNode::Nothing,
                        Some(triadic_error::Compiler::MissSemicolon),
                    )
                }
            }
        } else {
            (AstNode::Nothing, Some(triadic_error::Compiler::MissKeyword))
        }




    }


}
