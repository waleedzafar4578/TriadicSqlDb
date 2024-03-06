use storagecontroller::BaseControl;
use crate::lexical::Lexer;
use crate::syntax::{AstNode, Parser};

pub mod lexical;
pub mod syntax;
pub fn hello() {
    println!("Hello from compiler side");
}


pub fn sql_runner(query:&str, mut controller: &mut BaseControl) ->String{
    controller.initiate_database("../Testing/");
    let input=query.trim();
    let mut lexer=Lexer::new(input);
    let tokens=lexer.tokenize();
    let mut parser=Parser::new(&tokens);
    let (ast,db_name)=parser.parse();
    match ast {
        AstNode::SelectStatement => {}
        AstNode::CreateTableStatement => {}
        AstNode::CreateDatabaseStatement(name) => {
            controller.create_the_database(name.as_str());
            return format!("Database is Create with the Name of: {}",name)
        }
        AstNode::DropDatabaseStatement(_) => {}
        AstNode::SearchDatabaseStatement(_) => {}
        AstNode::RemoveDatabaseStatement(_) => {}
        AstNode::RenameDatabaseStatement(_) => {}
        AstNode::ShowDatabaseStatement => {}
        AstNode::UseDatabaseStatement(_) => {}
        AstNode::Nothing => {
            return "Wrong statement".to_string();
        }
    }
    "Error".to_string()
}