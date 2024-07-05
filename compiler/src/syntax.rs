use common_structure::SelectEntry;
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

pub mod truncate_parse;
pub mod alter_parse;
pub mod use_parse;
#[derive(Debug, Default)]
pub struct CompilerTableParseEntry {
    pub name: String,
    pub column_name: Vec<String>,
    pub type_plus_constraint: Vec<(AttributeType, Constraints)>,
}
#[derive(Debug, Default)]
pub struct CompilerTableDataEntry {
    pub name: String,
    pub column_name: Vec<String>,
    pub column_data: Vec<Vec<(String, char)>>,
}

#[derive(Debug, Default)]
pub struct AlterTableData {
    pub name: String,
    pub column_name: String,
    pub data_type: AttributeType,
}

#[derive(Debug)]
pub enum AstNode {
    CreateTableStatement(CompilerTableParseEntry),
    DropTableStatement(String),
    InsertTableStatement(CompilerTableDataEntry),
    CreateDatabaseStatement(String),
    DropDatabaseStatement(String),
    TruncateDatabaseStatement,
    TruncateTableStatement(String),
    SearchDatabaseStatement(String),
    RenameDatabaseStatement(String, String),
    RenameTableStatement(String, String),
    ShowDatabaseStatement,
    UseDatabaseStatement(String),
    SelectFullTable(SelectEntry),
    ShowTableStatement,
    Nothing,
    AlterTableAddStatement(AlterTableData),
    AlterTableDropStatement(AlterTableData)
    // ...Continue another Sql statement here.
}
pub struct Parser<'a> {
    tokens: &'a [Token],
    current_token: usize,
}
