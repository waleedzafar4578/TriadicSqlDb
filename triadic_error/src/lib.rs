

pub fn hello(){
    println!("Hello from error library");
}

pub enum Compiler{
    NotAKeyword,
    CREATE,CreateDatabase,CreateDatabaseIdentifier,
    Drop,DropDatabase,DropDatabaseIdentifier,
    Use,UseDatabase,UseDatabaseIdentifier,
    Show,ShowDatabase,
    Rename,RenameDatabase,RenameDatabaseIdentifier,
    Search,SearchDatabase,SearchDatabaseIdentifier,
}
#[derive(Debug)]
pub enum FrontSendCode{
    QEm,QNTK,
    QOkDDLC,QERRDDLC0,QERRDDLC1,QERRDDLC2,
    QOkDDLSH,QERRDDLSH0,QERRDDLSH1,
    QOkDDLD,QERRDDLD0,QERRDDLD1,QERRDDLD2,
    QOkDDLU,QERRDDLU0,QERRDDLU1,QERRDDLU2,
    QOkDDLR,QERRDDLR0,QERRDDLR1,QERRDDLR2,
    QOkDDLSE,QERRDDLSE0,QERRDDLSE1,QERRDDLSE2,
}

impl ToString for FrontSendCode {
    fn to_string(&self) -> String {
        match self {
            FrontSendCode::QEm => {
                String::from("QEM")
            }
            FrontSendCode::QNTK => {
                String::from("QNTK")
            }
            FrontSendCode::QOkDDLC => {
                String::from("QOkDDLC")
            }
            FrontSendCode::QERRDDLC0 => {
                String::from("QERRDDLC0")
            }
            FrontSendCode::QERRDDLC1 => {
                String::from("QERRDDLC1")
            }
            FrontSendCode::QERRDDLC2 => {
                String::from("QERRDDLC2")
            }
            FrontSendCode::QOkDDLSH => {
                String::from("QOkDDLSH")
            }
            FrontSendCode::QERRDDLSH0 => {
                String::from("QERRDDLSH0")
            }
            FrontSendCode::QERRDDLSH1 => {
                String::from("QERRDDLSH1")
            }
            FrontSendCode::QOkDDLD => {
                String::from("QOkDDLD")
            }
            FrontSendCode::QERRDDLD0 => {
                String::from("QERRDDLD0")
            }
            FrontSendCode::QERRDDLD1 => {
                String::from("QERRDDLD1")
            }
            FrontSendCode::QERRDDLD2 => {
                String::from("QERRDDLD2")
            }
            FrontSendCode::QOkDDLU => {
                String::from("QOkDDLU")
            }
            FrontSendCode::QERRDDLU0 => {
                String::from("QERRDDLU0")
            }
            FrontSendCode::QERRDDLU1 => {
                String::from("QERRDDLU1")
            }
            FrontSendCode::QERRDDLU2 => {
                String::from("QERRDDLU2")
            }
            FrontSendCode::QOkDDLR => {
                String::from("QOkDDLR")
            }
            FrontSendCode::QERRDDLR0 => {
                String::from("QERRDDLR0")
            }
            FrontSendCode::QERRDDLR1 => {
                String::from("QERRDDLR1")
            }
            FrontSendCode::QERRDDLR2 => {
                String::from("QERRDDLR2")
            }
            FrontSendCode::QOkDDLSE => {
                String::from("QOkDDLSE")
            }
            FrontSendCode::QERRDDLSE0 => {
                String::from("QERRDDLSE0")
            }
            FrontSendCode::QERRDDLSE1 => {
                String::from("QERRDDLSE1")
            }
            FrontSendCode::QERRDDLSE2 => {
                String::from("QERRDDLSE2")
            }
        }
    }
}