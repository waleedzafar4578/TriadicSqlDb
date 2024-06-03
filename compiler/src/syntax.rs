use storge::column::Constraints;
use triadic_logic::datatype::AttributeType;

use crate::lexical::Token;

pub mod create_parse;
pub mod drop_parse;
pub mod parse;
pub mod rename_parse;
pub mod search_parse;
pub mod show_parse;

pub mod insert_parse;
pub mod select_parse;


#[derive(Debug)]
pub struct CompilerTableParseEntry{
    pub name:String,
    pub column_name:Vec<String>,
    pub type_plus_constraint:Vec<(AttributeType,Constraints)>,
}
#[derive(Debug)]
pub struct CompilerTableDataEntry{
    pub name:String,
    pub column_name:Vec<String>,
    pub column_data:Vec<Vec<(String,char)>>,
}

#[derive(Debug)]
pub enum AstNode {
    CreateTableStatement(CompilerTableParseEntry),
    InsertTableStatement(CompilerTableDataEntry),
    CreateDatabaseStatement(String),
    DropDatabaseStatement(String),
    SearchDatabaseStatement(String),
    RenameDatabaseStatement(String,String),
    ShowDatabaseStatement,
    UseDatabaseStatement(String),
    SelectFullTable((Vec<String>,String)),
    Nothing,
    // ...Continue another Sql statement here.
}
pub struct Parser<'a> {
    tokens: &'a [Token],
    current_token: usize,
}
