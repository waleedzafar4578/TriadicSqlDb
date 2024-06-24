use std::fmt::Display;


pub mod engine_error;


pub enum Compiler {
    Nothing,
    MissColumn,
    MissColumnName,
    MissColumnDatatype,
    NotAKeyword,
    MissKeyword,
    MissIdentifier,
    MissSemicolon,
    MissOpenBracket,
    MissCloseBracket,
    MissValue,
    ConstraintsPrimary
}
#[derive(Debug)]
pub enum FrontSendCode {
    //Query Empty
    QueryEmpty,
    //Query doesn't start with keyword
    QueryKeywordMissing,
    //Query Parse that belongs to Data Definition Language Create keyword
    QueryIdentifierMissing,
    QuerySemiColonMissing,
    QueryColumnMissing,
    QueryColumnNameMissing,
    QueryColumnDatatypeMissing,
    QueryOpenBracketMissing,
    QueryCloseBracketMissing,
    SysDatabaseNotSelected,
    AlreadyExist,
    QueryProcessed,
    SysFound,
    SysNotFound,
    ValueDuplicate,
    Table,
    Use,
    Db,
    Err,
    Tb,
    
}

impl Display for FrontSendCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FrontSendCode::QueryEmpty => { String::from("QueryEmpty") }
            FrontSendCode::QueryKeywordMissing => { String::from("QueryKeywordMissing") }
            FrontSendCode::QueryIdentifierMissing => { String::from("QueryIdentifierMissing") }
            FrontSendCode::QuerySemiColonMissing => { String::from("QuerySemiColonMissing") }
            FrontSendCode::SysDatabaseNotSelected => { String::from("SysDatabaseNotSelected") }
            FrontSendCode::AlreadyExist => {String::from("AlreadyExist")}
            FrontSendCode::QueryProcessed => {String::from("QP")}
            FrontSendCode::SysFound => {String::from("SysFound")}
            FrontSendCode::SysNotFound => {String::from("SysNotFound")}
            FrontSendCode::QueryColumnMissing => {String::from("QueryColumnMissing")}
            FrontSendCode::QueryColumnNameMissing => {String::from("QueryColumnNameMissing")}
            FrontSendCode::QueryColumnDatatypeMissing => {String::from("QueryColumnDatatypeMissing")}
            FrontSendCode::QueryOpenBracketMissing => {String::from("QueryOpenBracketMissing")}
            FrontSendCode::QueryCloseBracketMissing => {String::from("QueryCloseBracketMissing")}
            FrontSendCode::ValueDuplicate => {String::from("ValueDuplicated")}
            FrontSendCode::Table => {String::from("Table")}
            FrontSendCode::Use => {String::from("Use")}
            FrontSendCode::Db => {String::from("Db")}
            FrontSendCode::Err => {String::from("Err")}
            FrontSendCode::Tb => {String::from("Tb")}
        };
        write!(f, "{}", str)
    }
}
