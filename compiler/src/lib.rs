use crate::lexical::Lexer;
use crate::syntax::{AstNode, Parser};
use storagecontroller::BaseControl;
use triadic_error::Compiler;

pub mod lexical;
pub mod syntax;
pub fn hello() {
    println!("Hello from compiler side");
}

pub fn sql_runner(query: &str, controller: &mut BaseControl) -> String {
    let input = query.trim();
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(&tokens);
    let (ast, error_type) = parser.parse();
    match ast {
        AstNode::SelectStatement => {}
        AstNode::CreateTableStatement => {}
        AstNode::CreateDatabaseStatement(name) => {
            controller.create_the_database(name.as_str());
            return format!("Database is Create with the Name of: {}",name)
        }
        AstNode::DropDatabaseStatement(name) => {
            controller.remove_the_database();
            return format!("Database is Delete with the Name of: {}",name)
        }
        AstNode::SearchDatabaseStatement(_) => {}
        AstNode::RemoveDatabaseStatement(_) => {}
        AstNode::RenameDatabaseStatement(_) => {}
        AstNode::ShowDatabaseStatement => {
            let ans=controller.list_down_the_name_database();
            let ath=ans.join(" ");
            return ath;
        }
        AstNode::UseDatabaseStatement(name) => {
            controller.use_this_database(name.as_str());
            return format!("{} This database is selected",name)
        }
        AstNode::Nothing => {
            match error_type {
                None => {}
                Some(ty) => {
                    return match
                    ty {
                        Compiler::NotAKeyword => {
                            "Query Must Start From Sql Keyword!".to_string()
                        }
                        Compiler::CREATE => {
                            "Please write proper Sql Keyword after CREATE\nList of Sql keywords which use after CREATE [DATABASE,TABLE]".to_string()
                        }
                        Compiler::CreateDatabase => {
                            "Please write the name of database which you want to create!".to_string()
                        }
                        Compiler::CreateDatabaseIdentifier => {
                            "Query Must be ended with semicolon!".to_string()
                        }

                        Compiler::Drop => {
                            "!".to_string()
                        }
                        Compiler::DropDatabase => {
                            "!".to_string()
                        }
                        Compiler::DropDatabaseIdentifier => {
                            "!".to_string()
                        }
                        Compiler::Use => {
                            "!".to_string()
                        }
                        Compiler::UseDatabase => {
                            "!".to_string()
                        }
                        Compiler::UseDatabaseIdentifier => {
                            "!".to_string()
                        }
                        Compiler::Show => {
                            "!".to_string()
                        }
                        Compiler::ShowDatabase => {
                            "!".to_string()
                        }
                        Compiler::Rename => {
                            "!".to_string()
                        }
                        Compiler::RenameDatabase => {
                            "!".to_string()
                        }
                        Compiler::RenameDatabaseIdentifier => {
                            "!".to_string()
                        }
                        Compiler::Search => {
                            "!".to_string()
                        }
                        Compiler::SearchDatabase => {
                            "!".to_string()
                        }
                        Compiler::SearchDatabaseIdentifier => {
                            "!".to_string()
                        }
                    }
                }
            }
        }
    }
    "Error".to_string()
}
