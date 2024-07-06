use common_structure::UpdateTableDataEntry;
use triadic_error::Compiler;

use crate::lexical::Token;
use crate::syntax::{AstNode, Parser};

impl<'a> Parser<'a> {

    pub fn parse_update_statement(&mut self) -> (AstNode, Option<Compiler>) {
        let mut info_select: UpdateTableDataEntry = UpdateTableDataEntry::default();
        self.advance();
        match self.extract_column_name() {
            None => {
                return (AstNode::Nothing, Some(Compiler::MissIdentifier));
            }
            Some(_tb_name) => {
                info_select.name=_tb_name;
            }
        }
        //println!("{:?}",self.tokens.get(self.current_token));
        self.advance();

        if !self.keyword_check("SET") {
            return (AstNode::Nothing, Some(Compiler::MissKeyword));
        }
        self.advance();


        if let Some(_col_name)=self.extract_identifier(){
            info_select.column_name=_col_name;
        }
        else {
            return (
                AstNode::Nothing,
                Some(Compiler::MissIdentifier),
            );
        }

        self.advance();

        if !self.check_operator("="){
            return (
                AstNode::Nothing,
                Some(Compiler::MissKeyword),
            );
        }
        self.advance();
        if !self.open_bracket_check(){
            return (
                AstNode::Nothing,
                Some(Compiler::MissOpenBracket),
            );
        }
        self.advance();

        if let Some(_col_data)=self.get_literal(){
            self.advance();
            if let Some(d)=self.get_value_degree(){
                info_select.column_data=(_col_data,d);
                self.advance();
            }
            else {
                info_select.column_data=(_col_data,'L');
            }
        }
        else {
            return (
                AstNode::Nothing,
                Some(Compiler::MissIdentifier),
            );
        }
        if !self.close_bracket_check(){
            return (
                AstNode::Nothing,
                Some(Compiler::MissCloseBracket),
            );
        }
        self.advance();

        return match self.condition_extract_update(&mut info_select) {
            None => {
                self.advance();
                return if self.terminate_with_semicolon() {
                    (
                        AstNode::UpdateTableStatement(info_select),
                        Some(Compiler::Nothing),
                    )
                } else {
                    (
                        AstNode::Nothing,
                        Some(Compiler::MissSemicolon),
                    )
                }
            }
            Some(_er) => {
                //println!("i am in where clause(Some(Err))");
                _er
            }
        }


    }
    pub fn condition_extract_update(&mut self,  info_select: &mut UpdateTableDataEntry) -> Option<(AstNode, Option<Compiler>)> {

        if !self.keyword_check("WHERE") {
            return Some((AstNode::Nothing, Some(Compiler::MissKeyword)));
        }
        self.advance();

        if let Some(col_name)=self.extract_identifier(){
            self.advance();

            if let Some(&Token::Operator(ref operator)) = self.tokens.get(self.current_token){
                self.advance();

                if !self.open_bracket_check() {
                    return Some((AstNode::Nothing, Some(Compiler::MissOpenBracket)));
                }
                self.advance();

                if let Some(val)=self.get_literal(){
                    self.advance();
                    if let Some(deg)=self.get_value_degree(){
                        self.advance();
                        info_select.where_clause=self.operator_to_clause(col_name,operator.as_str(),Some(val),Some(deg));
                        if !self.close_bracket_check(){
                            return Some((AstNode::Nothing, Some(Compiler::MissCloseBracket)));
                        }
                    }
                    else {
                        info_select.where_clause=self.operator_to_clause(col_name,operator.as_str(),Some(val),None);
                        if !self.close_bracket_check(){
                            return Some((AstNode::Nothing, Some(Compiler::MissCloseBracket)));
                        }
                    }
                }
                else {
                    if let Some(deg)=self.get_value_degree(){
                        self.advance();
                        info_select.where_clause=self.operator_to_clause(col_name,operator.as_str(),None,Some(deg));
                        if !self.close_bracket_check(){
                            return Some((AstNode::Nothing, Some(Compiler::MissCloseBracket)));
                        }
                    }
                    else {
                        return Some((AstNode::Nothing, Some(Compiler::MissValue)));
                    }
                }
            }
            else {
                //here need to change Miss Identifier to operator
                return Some((AstNode::Nothing, Some(Compiler::MissIdentifier)));
            }


        }
        else {
            return Some((AstNode::Nothing, Some(Compiler::MissIdentifier)));
        }
        None
    }

}