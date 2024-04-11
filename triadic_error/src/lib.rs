pub fn hello() {
    println!("Hello from error library");
}

pub mod engine_error;
pub enum Compiler {
    NotAKeyword,
    CREATE,
    CreateDatabase,
    CreateDatabaseIdentifier,
    Drop,
    DropDatabase,
    DropDatabaseIdentifier,
    Use,
    UseDatabase,
    UseDatabaseIdentifier,
    Show,
    ShowDatabase,
    Rename,
    RenameDatabase,
    RenameDatabaseIdentifier,
    Search,
    SearchDatabase,
    SearchDatabaseIdentifier,
    AddUser,
    CheckUser,
}
#[derive(Debug)]
pub enum FrontSendCode {
    //Query Empty
    QEm,
    //Query doesn't start with keyword
    QNTK,
    //Query Parse that belongs to Data Definition Language Create keyword
    QOkDDLC,
    //Query Error Create ____ ____ __
    QERRDDLC0,
    //Query Error Creates Database ____ __
    QERRDDLC1,
    //Query Error Create Database exam __
    QERRDDLC2,
    //Engine Error for this type of query
    EPNS,
    EAE,
    //Query Parse that belongs to Data Definition Language Show keyword
    QOkDDLSH,
    QERRDDLSH0,
    QERRDDLSH1,
    //Query Parse that belongs to Data Definition Language Drop keyword
    QOkDDLD,
    QERRDDLD0,
    QERRDDLD1,
    QERRDDLD2,
    //Query Parse that belongs to Data Definition Language Use keyword
    QOkDDLU,
    QERRDDLU0,
    QERRDDLU1,
    QERRDDLU2,
    //Query Parse that belongs to Data Definition Language Rename keyword
    QOkDDLR,
    QERRDDLR0,
    QERRDDLR1,
    QERRDDLR2,
    //Query Parse that belongs to Data Definition Language Select keyword
    QOkDDLSE,
    QERRDDLSE0,
    QERRDDLSE1,
    QERRDDLSE2,
    //Query Parse that belongs to Data Definition Language System keyword
    QOkSYSA,
    QOkSYSA0,
    //Query Parse that belongs to Data Definition Language Show keyword
    QOkSYSC,
    QOkSYSC0,
    //Engine Error
    
}

impl ToString for FrontSendCode {
    fn to_string(&self) -> String {
        match self {
            FrontSendCode::QEm => String::from("QEM"),
            FrontSendCode::QNTK => String::from("QNTK"),
            FrontSendCode::QOkDDLC => String::from("QOkDDLC"),
            FrontSendCode::QERRDDLC0 => String::from("QERRDDLC0"),
            FrontSendCode::QERRDDLC1 => String::from("QERRDDLC1"),
            FrontSendCode::QERRDDLC2 => String::from("QERRDDLC2"),
            FrontSendCode::QOkDDLSH => String::from("QOkDDLSH"),
            FrontSendCode::QERRDDLSH0 => String::from("QERRDDLSH0"),
            FrontSendCode::QERRDDLSH1 => String::from("QERRDDLSH1"),
            FrontSendCode::QOkDDLD => String::from("QOkDDLD"),
            FrontSendCode::QERRDDLD0 => String::from("QERRDDLD0"),
            FrontSendCode::QERRDDLD1 => String::from("QERRDDLD1"),
            FrontSendCode::QERRDDLD2 => String::from("QERRDDLD2"),
            FrontSendCode::QOkDDLU => String::from("QOkDDLU"),
            FrontSendCode::QERRDDLU0 => String::from("QERRDDLU0"),
            FrontSendCode::QERRDDLU1 => String::from("QERRDDLU1"),
            FrontSendCode::QERRDDLU2 => String::from("QERRDDLU2"),
            FrontSendCode::QOkDDLR => String::from("QOkDDLR"),
            FrontSendCode::QERRDDLR0 => String::from("QERRDDLR0"),
            FrontSendCode::QERRDDLR1 => String::from("QERRDDLR1"),
            FrontSendCode::QERRDDLR2 => String::from("QERRDDLR2"),
            FrontSendCode::QOkDDLSE => String::from("QOkDDLSE"),
            FrontSendCode::QERRDDLSE0 => String::from("QERRDDLSE0"),
            FrontSendCode::QERRDDLSE1 => String::from("QERRDDLSE1"),
            FrontSendCode::QERRDDLSE2 => String::from("QERRDDLSE2"),
            FrontSendCode::QOkSYSA => String::from("QOkSYSA"),
            FrontSendCode::QOkSYSA0 => String::from("QOkSYSA0"),
            FrontSendCode::QOkSYSC => String::from("QOkSYSC"),
            FrontSendCode::QOkSYSC0 => String::from("QOkSYSC0"),
            FrontSendCode::EPNS => String::from("EPNS"),
            FrontSendCode::EAE => String::from("EAE"),
        }
    }
}
