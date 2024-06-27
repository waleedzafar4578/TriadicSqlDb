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
#[derive(Debug, Default, Clone)]
pub struct EqualOperator {
    pub column_name: String,
    pub column_value: String,
    pub degree: Option<Degree>,
}

impl EqualOperator {
    pub fn set(col_name:String,value:String,deg:Option<Degree>)->Self{
        Self{
            column_name:col_name,
            column_value:value,
            degree: deg,
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct NotEqualOperator {
    pub column_name: String,
    pub column_value: String,
    pub degree: Option<Degree>,
}
impl NotEqualOperator {
    pub fn set(col_name:String,value:String,deg:Option<Degree>)->Self{
        Self{
            column_name:col_name,
            column_value:value,
            degree: deg,
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct GreaterEqualOperator {
    pub column_name: String,
    pub column_value: String,
    pub degree: Option<Degree>,
}
impl GreaterEqualOperator {
    pub fn set(col_name:String,value:String,deg:Option<Degree>)->Self{
        Self{
            column_name:col_name,
            column_value:value,
            degree: deg,
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct LessEqualOperator {
    pub column_name: String,
    pub column_value: String,
    pub degree: Option<Degree>,
}
impl LessEqualOperator {
    pub fn set(col_name:String,value:String,deg:Option<Degree>)->Self{
        Self{
            column_name:col_name,
            column_value:value,
            degree: deg,
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct GreaterOperator {
    pub column_name: String,
    pub column_value: String,
    pub degree: Option<Degree>,
}
impl GreaterOperator {
    pub fn set(col_name:String,value:String,deg:Option<Degree>)->Self{
        Self{
            column_name:col_name,
            column_value:value,
            degree: deg,
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct LessOperator {
    pub column_name: String,
    pub column_value: String,
    pub degree: Option<Degree>,
}
impl LessOperator {
    pub fn set(col_name:String,value:String,deg:Option<Degree>)->Self{
        Self{
            column_name:col_name,
            column_value:value,
            degree: deg,
        }
    }
}
#[derive(Debug, Default,Clone)]
pub struct WhereClause {
    pub equal_operator: Option<EqualOperator>,
    pub not_equal_operator: Option<NotEqualOperator>,
    pub greater_equal_operator: Option<GreaterEqualOperator>,
    pub less_equal_operator: Option<LessEqualOperator>,
    pub greater_operator: Option<GreaterOperator>,
    pub less_operator: Option<LessOperator>,
}
#[derive(Debug, Default, Clone)]
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
