use common_structure::{EqualOperator, GreaterEqualOperator, GreaterOperator, LessEqualOperator, LessOperator, NotEqualOperator, WhereClause};
use triadic_error::Compiler;
use crate::char_to_degree;
use crate::lexical::Token;
use crate::syntax::{AstNode, Parser, SelectEntry};

impl<'a> Parser<'a> {
    pub fn parse_select_statement(&mut self) -> (AstNode, Option<Compiler>) {
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
                AstNode::SelectFullTable(info_select),
                Some(Compiler::Nothing),
            )

        }

        return match self.condition_extract(&mut info_select) {
            None => {
                println!("i am in where clause(None)");
                (
                    AstNode::SelectFullTable(info_select),
                    Some(Compiler::Nothing),
                )
            }
            Some(_er) => {
                println!("i am in where clause(Some(Err))");
                _er
            }
        }


    }
    pub fn condition_extract(&mut self,  info_select: &mut SelectEntry) -> Option<(AstNode, Option<Compiler>)> {

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
    pub fn operator_to_clause(&self,col_name:String,operator:&str,value:Option<String>,deg:Option<char>)->Option<WhereClause>{
        let mut get_condition=WhereClause::default();
        match operator {
            "="=>{
                match deg {
                    None => {
                        get_condition.equal_operator=Some(EqualOperator::set(col_name, value, None))
                    }
                    Some(_d) => {
                        get_condition.equal_operator=Some(EqualOperator::set(col_name,value,Some(char_to_degree(_d))))
                    }
                }

            }
            ">="=>{
                match deg {
                    None => {
                        get_condition.greater_equal_operator=Some(GreaterEqualOperator::set(col_name, value, None))
                    }
                    Some(_d) => {
                        get_condition.greater_equal_operator=Some(GreaterEqualOperator::set(col_name,value,Some(char_to_degree(_d))))
                    }
                }
            }
            "<="=>{
                match deg {
                    None => {
                        get_condition.less_equal_operator=Some(LessEqualOperator::set(col_name, value, None))
                    }
                    Some(_d) => {
                        get_condition.less_equal_operator=Some(LessEqualOperator::set(col_name,value,Some(char_to_degree(_d))))
                    }
                }
            }
            "!="=>{
                match deg {
                    None => {
                        get_condition.not_equal_operator=Some(NotEqualOperator::set(col_name, value, None))
                    }
                    Some(_d) => {
                        get_condition.not_equal_operator=Some(NotEqualOperator::set(col_name,value,Some(char_to_degree(_d))))
                    }
                }
            }
            ">"=>{
                match deg {
                    None => {
                        get_condition.greater_operator=Some(GreaterOperator::set(col_name, value, None))
                    }
                    Some(_d) => {
                        get_condition.greater_operator=Some(GreaterOperator::set(col_name,value,Some(char_to_degree(_d))))
                    }
                }
            }
            "<"=>{
                match deg {
                    None => {
                        get_condition.less_operator=Some(LessOperator::set(col_name, value, None))
                    }
                    Some(_d) => {
                        get_condition.less_operator=Some(LessOperator::set(col_name,value,Some(char_to_degree(_d))))
                    }
                }
            }
            _ => {
                return None
            }
        }
        Some(get_condition)
    }
    pub fn show_current_token(&self,info:&str){
        println!("{}{:?}",info,self.tokens.get(self.current_token));
    }
}
