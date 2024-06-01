use crate::lexical::Token;
use crate::syntax::{AstNode, CompilerTableDataEntry, Parser};

impl<'a> Parser<'a> {
    pub(crate) fn parse_insert_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        self.advance();
        if self.keyword_check("INTO") {
            return self.getting_information_for_query();
        }
        (AstNode::Nothing, Some(triadic_error::Compiler::NotAKeyword))
    }
    fn getting_information_for_query(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        //println!("Inside Getting function");
        let mut getting_data: CompilerTableDataEntry = CompilerTableDataEntry {
            name: "".to_string(),
            column_name: vec![],
            column_data: vec![],
        };
        //Must be table name.
        self.advance();
        match self.extract_identifier() {
            None => {
                return (
                    AstNode::Nothing,
                    Some(triadic_error::Compiler::MissIdentifier),
                );
            }
            Some(_table_name) => {
                getting_data.name = _table_name;
                self.advance();
            }
        }
        //println!("Table name is fetch:{}", getting_data.name.clone());
        match self.get_list_of_column() {
            None => {
                //println!("None call");
                return (
                    AstNode::Nothing,
                    Some(triadic_error::Compiler::MissColumnName),
                );
            }
            Some(_list_name) => {
                //println!("list of column name");
                getting_data.column_name = _list_name;
            }
        }

        if !self.keyword_check("VALUES") {
            return (AstNode::Nothing, Some(triadic_error::Compiler::MissKeyword));
        }
        //println!("Values keyword found");
        self.advance();

        while !self.terminate_with_semicolon() {
            match self.get_list_of_values() {
                None => {
                    return (AstNode::Nothing, Some(triadic_error::Compiler::MissColumnName));
                }
                Some(_data) => {
                    //self.advance();
                    getting_data.column_data.push(_data);
                    //println!("{:?}",_data);
                }
            }
            //println!("{:?}",self.tokens.get(self.current_token));
            if !self.comma_check() {
                if self.terminate_with_semicolon(){
                    break;
                }
                else {
                    return (AstNode::Nothing, Some(triadic_error::Compiler::MissSemicolon));
                }
            }

            //println!("inside loop:{:?}",self.tokens.get(self.current_token));
            self.advance();
        }
        //println!("{:#?}", getting_data);
        (AstNode::InsertTableStatement(getting_data), None)
    }
    fn get_list_of_column(&mut self) -> Option<Vec<String>> {
        //println!("Inside of list of column name");
        let mut column_name: Vec<String> = vec![];
        if !self.open_bracket_check() {
            return None;
        }
        self.advance();
        //println!("Open bracket find");
        while !self.close_bracket_check() {
            match self.extract_identifier() {
                None => {
                    //println!("column name not find");
                    return None;
                }
                Some(_column_name) => {
                    //println!("column name :{}",_column_name.clone());
                    column_name.push((_column_name));


                }
            }
            self.advance();
            match self.comma_check() {
                true => {
                    self.advance();
                }
                false => {
                    if self.close_bracket_check() {
                        self.advance();
                        return Some(column_name);
                    }
                    return None;
                }
            }
        }
        Some(column_name)
    }
    fn get_list_of_values(&mut self) -> Option<Vec<(String,char)>> {
        //println!("Inside of list of column name");
        let mut column_data: Vec<(String,char)> = vec![];
        if !self.open_bracket_check() {
            return None;
        }
        self.advance();
        //println!("Open bracket find");
        while !self.close_bracket_check() {
            match self.get_literal() {
                None => {
                    //println!("");
                    return None;
                }
                Some(_column_value) => {
                    //println!("column name :{}",_column_name.clone());
                    self.advance();
                    match self.get_value_degree() {
                        None => {}
                        Some(_d) => {
                            //println!("{}:{}",_column_value,_d);
                            column_data.push((_column_value,_d));
                        }
                    }

                }
            }
            self.advance();
            match self.comma_check() {
                true => {
                    self.advance();
                }
                false => {
                    if self.close_bracket_check() {
                        self.advance();
                        return Some(column_data);
                    }
                    return None;
                }
            }
        }
        Some(column_data)
    }
}
