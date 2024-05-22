use crate::lexical::Token;
use crate::syntax::{AstNode, CompilerTableParseEntry, Parser};
use storge::column::Constraints;

impl<'a> Parser<'a> {
    pub(crate) fn parse_create_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        self.advance();
        //checking the next token is DATABASE
        if let Some(Token::Keyword(ref next_keyword)) = self.tokens.get(self.current_token) {
            if next_keyword == "DATABASE" {
                self.advance(); //Move to next token
                return self.parse_create_database_statement();
            } else if next_keyword == "TABLE" {
                self.advance();
                return self.parse_create_table_statement();
            }
        }
        (AstNode::Nothing, Some(triadic_error::Compiler::CREATE))
    }
    fn parse_create_database_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        //here checking that next token is identifier
        if let Some(Token::Identifier(ref db_name)) = self.tokens.get(self.current_token) {
            self.advance();
            return if let Some(Token::Punctuation(';')) = self.tokens.get(self.current_token) {
                //Successfully parsed CREATE a DATABASE statement
                (AstNode::CreateDatabaseStatement(db_name.clone()), None)
            } else {
                (
                    AstNode::Nothing,
                    Some(triadic_error::Compiler::CreateDatabaseIdentifier),
                )
            };
        }
        (
            AstNode::Nothing,
            Some(triadic_error::Compiler::CreateDatabase),
        )
    }
    fn parse_create_table_statement(&mut self) -> (AstNode, Option<triadic_error::Compiler>) {
        //here checking that next token is identifier
        let mut temp: CompilerTableParseEntry = CompilerTableParseEntry {
            name: "".to_string(),
            column_name: vec![],
            type_plus_constraint: vec![],
        };
        let temp_constraint: Constraints = Constraints {
            not_null: false,
            unique: false,
            primary_key: false,
            foreign_key: false,
            check: false,
            check_value: 0,
            default: false,
            default_value: "".to_string(),
            index: false,
            index_type: "".to_string(),
        };

        if let Some(Token::Identifier(ref table_name)) = self.tokens.get(self.current_token) {
            temp.name.clone_from(table_name);
            self.advance();
        } else {
            panic!("You miss table name!");
        }

        //println!("{:?}",self.tokens.get(self.current_token));
        if  Some(&Token::Punctuation('(')) != self.tokens.get(self.current_token){
            panic!("You miss round bracket");
        }
        self.advance();

//------------------------------------------------------


        while self.tokens.get(self.current_token) !=Some(&Token::Punctuation(';')) {
            if self.tokens.get(self.current_token).is_none() {
                break;
            }

            println!("{:?}",self.tokens.get(self.current_token));
            if let Some(Token::Identifier(ref coln)) = self.tokens.get(self.current_token){
                temp.column_name.push(coln.clone()) ;
            }
            self.advance();


            let datatype:String;
             println!("{:?}",self.tokens.get(self.current_token));
            if let Some(Token::Keyword(ref _datatype))=self.tokens.get(self.current_token){
                datatype=_datatype.to_string();
            }
            self.advance();

            while self.tokens.get(self.current_token) !=Some(&Token::Punctuation(',')) {
                if self.tokens.get(self.current_token).is_none() {
                    break;
                }
                if self.tokens.get(self.current_token) ==Some(&Token::Punctuation(')')) &&
                    self.tokens.get(self.current_token+1) ==Some(&Token::Punctuation(';')){
                    break;
                }
                //here
                
                println!("                          :{:?}",self.tokens.get(self.current_token));
                self.advance();
            }

        }



        (
            AstNode::Nothing,
            Some(triadic_error::Compiler::CreateDatabase),
        )
    }
}
