use crate::lexical::Lexer;
use crate::syntax::{AstNode, Parser};
use storagecontroller::BaseControl;
use triadic_error::{Compiler, FrontSendCode};

pub mod lexical;
pub mod syntax;
pub fn hello() {
    println!("Hello from compiler side");
}

pub fn sql_runner(query: &str, controller: &mut BaseControl) -> (FrontSendCode,String) {
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
            return (FrontSendCode::QOkDDLC,format!("Database is Create with the Name of: {}",name))
        }
        AstNode::DropDatabaseStatement(name) => {
            controller.remove_the_database();
            return (FrontSendCode::QOkDDLC,format!("Database is Delete with the Name of: {}",name))
        }
        AstNode::SearchDatabaseStatement(_) => {}
        AstNode::RemoveDatabaseStatement(_) => {}
        AstNode::RenameDatabaseStatement(_) => {}
        AstNode::ShowDatabaseStatement => {
            let ans=controller.list_down_the_name_database();
            let ath=ans.join(" ");
            return (FrontSendCode::QOkDDLSH,ath);
        }
        AstNode::UseDatabaseStatement(name) => {
            controller.use_this_database(name.as_str());
            return (FrontSendCode::QOkDDLU,format!("{} This database is selected",name))
        }
        AstNode::Nothing => {
            match error_type {
                None => {}
                Some(ty) => {
                   return  match ty {
                        Compiler::NotAKeyword => {
                            (FrontSendCode::QNTK,query.to_string())
                        }
                        Compiler::CREATE => {
                            (FrontSendCode::QERRDDLC0,query.to_string())
                        }
                        Compiler::CreateDatabase => {
                            (FrontSendCode::QERRDDLC1,query.to_string())
                        }
                        Compiler::CreateDatabaseIdentifier => {
                            (FrontSendCode::QERRDDLC2,query.to_string())
                        }
                        Compiler::Drop => {
                            (FrontSendCode::QERRDDLD0,query.to_string())
                        }
                        Compiler::DropDatabase => {
                            (FrontSendCode::QERRDDLD1,query.to_string())
                        }
                        Compiler::DropDatabaseIdentifier => {
                            (FrontSendCode::QERRDDLD2,query.to_string())
                        }
                        Compiler::Use => {
                            (FrontSendCode::QERRDDLU0,query.to_string())
                        }
                        Compiler::UseDatabase => {
                            (FrontSendCode::QERRDDLU1,query.to_string())
                        }
                        Compiler::UseDatabaseIdentifier => {
                            (FrontSendCode::QERRDDLU2,query.to_string())
                        }
                        Compiler::Show => {
                            (FrontSendCode::QERRDDLSH0,query.to_string())
                        }
                        Compiler::ShowDatabase => {
                            (FrontSendCode::QERRDDLSH1,query.to_string())
                        }
                        Compiler::Rename => {
                            (FrontSendCode::QERRDDLR0,query.to_string())
                        }
                        Compiler::RenameDatabase => {
                            (FrontSendCode::QERRDDLR1,query.to_string())
                        }
                        Compiler::RenameDatabaseIdentifier => {
                            (FrontSendCode::QERRDDLR2,query.to_string())
                        }
                        Compiler::Search => {
                            (FrontSendCode::QERRDDLSE0,query.to_string())
                        }
                        Compiler::SearchDatabase => {
                            (FrontSendCode::QERRDDLSE1,query.to_string())
                        }
                        Compiler::SearchDatabaseIdentifier => {
                            (FrontSendCode::QERRDDLSE2,query.to_string())
                        }
                    }

                }
            }
        }
    }
    (FrontSendCode::QEm,"Error".to_string())
}
