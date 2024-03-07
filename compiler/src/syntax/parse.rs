use crate::lexical::Token;
use crate::syntax::{AstNode, Parser};

impl<'a> Parser<'a>{
    pub fn new(tokens: &'a [Token])->Self{
        Parser{
            tokens,
            current_token: 0,
        }
    }
    pub fn parse(&mut self)->(AstNode,Option<triadic_error::Compiler>){
        //First check that first token is sql keyword
        if let Some(Token::Keyword(ref keyword))=
            self.tokens.get(self.current_token){
            match keyword.to_uppercase().as_str() {
                "CREATE"=>self.parse_create_statement(),
                "DROP"=>  self.parse_drop_statement(),
                "USE"=>   self.parse_use_statement(),
                "SHOW"=>  panic!(),
                "RENAME"=>panic!(),
                "SEARCH"=>panic!(),
                _ => {
                    (AstNode::Nothing, Some(triadic_error::Compiler::NotAKeyword))
                }
            }
        }else {
            println!("Unexpected token");
            return (AstNode::Nothing,None);
        }
    }
    pub(crate) fn advance(&mut self){
        self.current_token+=1;
    }
}