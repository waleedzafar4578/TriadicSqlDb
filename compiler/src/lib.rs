use crate::lexical::Lexer;
use crate::syntax::{AstNode, Parser};
use storagecontroller::BaseControl;
use triadic_error::engine_error::{EngineErrorCreate, EngineErrorDrop};
use triadic_error::{Compiler, FrontSendCode};

pub mod lexical;
pub mod syntax;

pub fn hello() {
    println!("Hello from compiler side");
}

pub fn sql_runner(query: &str, controller: &mut BaseControl) -> (FrontSendCode, String) {
    controller.initiate_database("../../servertesting/");
    let input = query.trim();
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    println!("{:?}",tokens);

    let mut parser = Parser::new(&tokens);
    let (ast, error_type) = parser.parse();
    match ast {
        AstNode::SelectStatement => {}
        AstNode::CreateTableStatement(data) => {
            
        }
        AstNode::CreateDatabaseStatement(name) => {
            return match controller.create_the_database(name.as_str()) {
                EngineErrorCreate::PathNotSelected => {
                    (
                        FrontSendCode::EPNS,
                        "Please First Connect your system with server".to_string(),
                    )
                }
                EngineErrorCreate::AlreadyExist => {
                    (
                        FrontSendCode::EAE,
                        format!("Database is already exist with the Name of: {}", name),
                    )
                }
                EngineErrorCreate::DoneYes => {
                    (
                        FrontSendCode::QOkDDLC,
                        format!("Database is Create with the Name of: {}", name),
                    )
                }
            };
        }
        AstNode::DropDatabaseStatement(name) => {
            match controller.remove_the_database(name.as_str()){
                EngineErrorDrop::PathNotSelected => {
                    (
                        FrontSendCode::EPNS,
                        "Engine Path Not Selected".to_string(),
                    )
                }
                EngineErrorDrop::NotFind => {
                    (
                        FrontSendCode::ENE,
                        "Engine Database Not Exist".to_string(),
                    )
                }
                EngineErrorDrop::DoneYes => {
                    (
                        FrontSendCode::QOkDDLD,
                        "Database Dropped".to_string(),
                    )
                }
            };
        }
        AstNode::SearchDatabaseStatement(name) => {
            match controller.find_this_database(name.as_str()) {
                true => {
                    (
                        FrontSendCode::QOkDDLSE,
                        "Database Found".to_string(),
                    )
                }
                false => {
                    (
                        FrontSendCode::EAE,
                        "Database Not Found".to_string(),
                    )
                }
            };
        }
        AstNode::RenameDatabaseStatement(old_path,new_path) => {
               match controller.rename_the_database(&old_path, &new_path) {
                   true => {
                       (
                           FrontSendCode::QOkDDLR,
                           "Database Rename".to_string(),
                       )
                   }
                   false => {
                       (
                           FrontSendCode::EAE,
                           "Database Not Found".to_string(),
                       )
                   }
               };
        }
        AstNode::ShowDatabaseStatement => {
            let ans = controller.list_down_the_name_database();
            let ath = ans.join(" ");
            return (FrontSendCode::QOkDDLSH, ath);
        }
        AstNode::UseDatabaseStatement(name) => {
            match controller.use_this_database(name.as_str()){
                true => {
                    (
                        FrontSendCode::QOkDDLU,
                        "Database Found".to_string(),
                    )
                }
                false => {
                    (
                        FrontSendCode::EAE,
                        "Database Not Found".to_string(),
                    )
                }
            };
        }
        AstNode::Nothing => match error_type {
            None => {}
            Some(ty) => {
                return match ty {
                    Compiler::NotAKeyword => (FrontSendCode::QNTK, query.to_string()),
                    Compiler::CREATE => (FrontSendCode::QERRDDLC0, query.to_string()),
                    Compiler::CreateDatabase => (FrontSendCode::QERRDDLC1, query.to_string()),
                    Compiler::CreateDatabaseIdentifier => {
                        (FrontSendCode::QERRDDLC2, query.to_string())
                    }
                    Compiler::Drop => (FrontSendCode::QERRDDLD0, query.to_string()),
                    Compiler::DropDatabase => (FrontSendCode::QERRDDLD1, query.to_string()),
                    Compiler::DropDatabaseIdentifier => {
                        (FrontSendCode::QERRDDLD2, query.to_string())
                    }
                    Compiler::Use => (FrontSendCode::QERRDDLU0, query.to_string()),
                    Compiler::UseDatabase => (FrontSendCode::QERRDDLU1, query.to_string()),
                    Compiler::UseDatabaseIdentifier => {
                        (FrontSendCode::QERRDDLU2, query.to_string())
                    }
                    Compiler::Show => (FrontSendCode::QERRDDLSH0, query.to_string()),
                    Compiler::ShowDatabase => (FrontSendCode::QERRDDLSH1, query.to_string()),
                    Compiler::Rename => (FrontSendCode::QERRDDLR0, query.to_string()),
                    Compiler::RenameDatabase => (FrontSendCode::QERRDDLR1, query.to_string()),
                    Compiler::RenameDatabaseIdentifier => {
                        (FrontSendCode::QERRDDLR2, query.to_string())
                    }
                    Compiler::Search => (FrontSendCode::QERRDDLSE0, query.to_string()),
                    Compiler::SearchDatabase => (FrontSendCode::QERRDDLSE1, query.to_string()),
                    Compiler::SearchDatabaseIdentifier => {
                        (FrontSendCode::QERRDDLSE2, query.to_string())
                    }
                    Compiler::AddUser => (FrontSendCode::QERRDDLSE2, query.to_string()),
                    Compiler::CheckUser => (FrontSendCode::QERRDDLSE2, query.to_string()),
                }
            }
        },
        AstNode::AddUser(name) => {
            let path = format!("../Testing/{}/", name);
            return if controller.initiate_database(path.as_str()) {
                (FrontSendCode::QOkDDLC, "".to_string())
            } else {
                (
                    FrontSendCode::QOkDDLC,
                    "Failed to Add User into Server".to_string(),
                )
            };
        }
        AstNode::CheckUser(name) => {
            let path = format!("../Testing/{}/", name);
            return if controller.initiate_database(path.as_str()) {
                (FrontSendCode::QOkDDLC, "".to_string())
            } else {
                (
                    FrontSendCode::QOkDDLC,
                    "Failed to Check User into Server".to_string(),
                )
            };
        }
    }
    (FrontSendCode::QEm, "Error".to_string())
}
