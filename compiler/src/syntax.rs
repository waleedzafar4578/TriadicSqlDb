use crate::lexical::Token;
pub mod parse;
pub mod create_parse;
pub mod drop_parse;
#[derive(Debug)]
pub enum AstNode{
    SelectStatement,
    CreateTableStatement,
    CreateDatabaseStatement(String),
    DropDatabaseStatement(String),
    SearchDatabaseStatement(String),
    RemoveDatabaseStatement(String),
    RenameDatabaseStatement(String),
    ShowDatabaseStatement,
    UseDatabaseStatement(String),
    Nothing,
    // ...Continue another Sql statement here.
}
pub struct Parser<'a>{
    tokens:& 'a[Token] ,
    current_token:usize,
}























