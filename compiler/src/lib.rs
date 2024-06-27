//TriadicSqlDb/compiler/src/lib.rs

//! This module provides utilities for processing the user query
//!
//! It includes starting point for sql compiler.
//! - Sql_runner takes two arguments:
//!             1: One is queried that comes from user.
//!             2: Second one is a base control structure
//! that comes from server with loaded user previous information.
//! - Lexical module provides utilities for tokenize the query.
//! - Syntax module provides utilities for a check token sequence is according to grammar.

pub mod lexical;

use crate::lexical::Lexer;

pub mod syntax;
use crate::syntax::{AstNode, Parser};

use storagecontroller::BaseControl;

use triadic_error::engine_error::EngineError;
use triadic_error::{Compiler, FrontSendCode};
use triadic_logic::degree::Degree;

///
/// This sql runner function is responsible for operations:
///     1: Tokenize the stream of character:
///         1: Trim the query.
///         2: Tokenize the trim query.
///     2: Check Sequence the stream of token.
///     3: Find which statement user wants to run.
///     4: Then run user required query.
///     5: Send back response which are correct or error.
///     6: Every response and error goes with code.
///     7: Code for frontend app which recognizes the coming response.
///
///
///
///
///

pub fn sql_runner(query: &str, controller: &mut BaseControl) -> (FrontSendCode, String) {

    let input = query.trim();
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    println!("{:?}",tokens);

    let mut parser = Parser::new(&tokens);
    let (ast, error_type) = parser.parse();
    match ast {
        AstNode::InsertTableStatement(table_data) => {
            return match controller.search_table(table_data.name.clone().as_str()) {
                true => {
                    let walk_in_column_name = table_data.column_data.iter();
                    let mut column_iterator = 0;
                    let mut row_iterator = 0;
                    for val in walk_in_column_name {
                        println!("{:?}", &val);
                        for nm in &table_data.column_name {
                            //println!("{}  ", nm);
                            let (value, value_degree) =
                                &table_data.column_data[row_iterator][column_iterator];
                            //println!("{}:{}",value,char_to_degree(value_degree));
                            if !controller.insert_to_table(
                                table_data.name.as_str(),
                                nm.as_str(),
                                value.as_str(),
                                char_to_degree(*value_degree),
                            ) {
                                return (
                                    FrontSendCode::Err,
                                    "Data is duplicate!".to_string(),
                                );
                            }
                            column_iterator += 1;
                        }
                        column_iterator = 0;
                        row_iterator += 1;
                    }
                    controller.save_to_file();
                    (
                        FrontSendCode::QueryProcessed,
                        "Data Inserted in Table!".to_string(),
                    )
                }
                false => (
                    FrontSendCode::QueryKeywordMissing,
                    "Please first create table.In database not found this table!".to_string(),
                ),
            };
        }
        AstNode::CreateTableStatement(_data) => {
            let answer =
                controller.add_table(&_data.name, _data.column_name, _data.type_plus_constraint);
            controller.save_to_file();
            return (FrontSendCode::QueryProcessed, answer);
            //`println!("table creation:{:#?}",_data);
        }
        AstNode::CreateDatabaseStatement(name) => {
            return engine_error(controller.create_the_database(name.as_str()));
        }
        AstNode::DropDatabaseStatement(name) => {
            return engine_error(controller.remove_the_database(name.as_str()));
        }
        AstNode::SearchDatabaseStatement(name) => {
            let (ans,_)=controller.find_this_database(name.as_str());
            return (
                FrontSendCode::QueryProcessed,
                ans
            )
        }
        AstNode::RenameDatabaseStatement(old_path, new_path) => {
            return (
                FrontSendCode::QueryProcessed,
                controller.rename_the_database(&old_path, &new_path),
            )
        }
        AstNode::ShowDatabaseStatement => {
            let ans = controller.list_down_the_name_database();
            let ath = ans.join(" ");
            return (FrontSendCode::Db, ath);
        }
        AstNode::UseDatabaseStatement(name) => {
            let (ans,con)=controller.find_this_database(name.as_str());
            return if con {
                (
                    FrontSendCode::Use,
                    name,
                )
            } else {
                (
                    FrontSendCode::QueryProcessed,
                    ans,
                )
            }

        }
        AstNode::Nothing => match error_type {
            None => {}
            Some(ty) => {
                return match ty {
                    Compiler::NotAKeyword => {
                        (FrontSendCode::Err, String::from("Error:Query must start from keyword!"))
                    }
                    Compiler::MissKeyword => {
                        (FrontSendCode::Err, String::from("Error:Missed somewhere keyword!"))
                    }
                    Compiler::MissIdentifier => {
                        (FrontSendCode::Err, String::from("Error:Missed Somewhere identifier!"))
                    }
                    Compiler::MissSemicolon => {
                        (FrontSendCode::Err, String::from("Error:Query must end with semicolon!"))
                    }
                    Compiler::Nothing => (FrontSendCode::QueryProcessed, String::from(query)),
                    Compiler::MissColumn => {
                        (FrontSendCode::Err, String::from("Error:Table must have one column!"))
                    }
                    Compiler::MissColumnName => {
                        (FrontSendCode::Err, String::from("Error:Missed Column name!"))
                    }
                    Compiler::MissColumnDatatype => (
                        FrontSendCode::Err,
                        String::from("Error:Missed column datatype"),
                    ),
                    Compiler::MissOpenBracket => {
                        (FrontSendCode::Err, String::from("Error:Missed Somewhere open bracket!"))
                    }
                    Compiler::MissCloseBracket => {
                        (FrontSendCode::Err, String::from("Error:Missed Somewhere close bracket!"))
                    }
                    Compiler::MissValue => {
                        (FrontSendCode::Err, String::from("Error:Query value missing"))
                    }
                    Compiler::ConstraintsPrimary => {
                        (FrontSendCode::Err, String::from("Error:In Query somewhere use primary key,Issue in primary key!"))
                    }
                }
            }
        },
        AstNode::SelectFullTable(_info) => {
            println!("{:?}",_info);
            return (
                FrontSendCode::Table,
                serde_json::to_string_pretty(&controller.show_table(_info.name.as_str(), _info.column_name))
                    .unwrap(),
            )
        }
        AstNode::DropTableStatement(_tb_name) => {
            let answer = controller.drop_table(_tb_name.as_str());
            controller.save_to_file();
            return (FrontSendCode::QueryProcessed, answer);
        }
        AstNode::ShowTableStatement => {
            let ans = controller.list_of_tables();
            let ath = ans.join(" ");
            return (FrontSendCode::Tb, ath);
        }
    }
    (FrontSendCode::QueryEmpty, "Error".to_string())
}

fn engine_error(engine_error: EngineError) -> (FrontSendCode, String) {
    match engine_error {
        EngineError::PathNotSelected => (
            FrontSendCode::SysDatabaseNotSelected,
            "Engine Path Not Selected".to_string(),
        ),
        EngineError::NotFind => (
            FrontSendCode::SysNotFound,
            "Engine Database Not Exist".to_string(),
        ),
        EngineError::DoneYes => (
            FrontSendCode::QueryProcessed,
            "You request is processed".to_string(),
        ),
        EngineError::AlreadyExist => (
            FrontSendCode::AlreadyExist,
            "Database is already exist!".to_string(),
        ),
        EngineError::IsCreated => (
            FrontSendCode::QueryProcessed,
            "Database Created".to_string(),
        ),
        EngineError::IsFound => (FrontSendCode::QueryProcessed, "Database Found".to_string()),
        EngineError::IsRemove => (
            FrontSendCode::QueryProcessed,
            "Requested thing Removed".to_string(),
        ),
    }
}

pub fn char_to_degree(input: char) -> Degree {
    match input {
        'T' => Degree::T,
        'L' => Degree::L,
        'F' => Degree::F,
        _ => Degree::L,
    }
}
