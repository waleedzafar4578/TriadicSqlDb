use storge::column::Constraints;
use triadic_logic::datatype::AttributeType;
use triadic_logic::degree::Degree;

use crate::lexical::Token;

pub mod create_parse;
pub mod drop_parse;
pub mod parse;
pub mod rename_parse;
pub mod search_parse;
pub mod show_parse;

pub mod insert_parse;
pub mod select_parse;

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
pub struct EqualOperator {
    pub column_name: String,
    pub column_value: String,
    pub degree: Option<Degree>,
}
#[derive(Debug, Default)]
pub struct NotEqualOperator {
    pub column_name: String,
    pub column_value: String,
    pub degree: Option<Degree>,
}
#[derive(Debug, Default)]
pub struct GreaterEqualOperator {
    pub column_name: String,
    pub column_value: String,
    pub degree: Option<Degree>,
}
#[derive(Debug, Default)]
pub struct LessEqualOperator {
    pub column_name: String,
    pub column_value: String,
    pub degree: Option<Degree>,
}
#[derive(Debug, Default)]
pub struct GreaterOperator {
    pub column_name: String,
    pub column_value: String,
    pub degree: Option<Degree>,
}
#[derive(Debug, Default)]
pub struct LessOperator {
    pub column_name: String,
    pub column_value: String,
    pub degree: Option<Degree>,
}
#[derive(Debug, Default)]
pub struct WhereClause {
    pub equal_operator: Option<EqualOperator>,
    pub not_equal_operator: Option<NotEqualOperator>,
    pub greater_equal_operator: Option<GreaterEqualOperator>,
    pub less_equal_operator: Option<LessEqualOperator>,
    pub greater_operator: Option<GreaterOperator>,
    pub less_operator: Option<LessOperator>,
}
#[derive(Debug, Default)]
pub struct SelectEntry {
    pub name: String,
    pub column_name: Vec<String>,
    pub where_clause: Option<WhereClause>,
}

#[derive(Debug)]
pub enum AstNode {
    CreateTableStatement(CompilerTableParseEntry),
    DropTableStatement(String),
    InsertTableStatement(CompilerTableDataEntry),
    CreateDatabaseStatement(String),
    DropDatabaseStatement(String),
    SearchDatabaseStatement(String),
    RenameDatabaseStatement(String, String),
    ShowDatabaseStatement,
    UseDatabaseStatement(String),
    SelectFullTable(SelectEntry),
    ShowTableStatement,
    Nothing,
    // ...Continue another Sql statement here.
}
pub struct Parser<'a> {
    tokens: &'a [Token],
    current_token: usize,
}
