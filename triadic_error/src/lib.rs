pub fn hello(){
    println!("Hello from error library");
}

pub enum Compiler{
    NotAKeyword,
    CREATE,CreateDatabase,CreateDatabaseIdentifier,
    Drop,DropDatabase,DropDatabaseIdentifier,
    Use,UseDatabase,UseDatabaseIdentifier
}
