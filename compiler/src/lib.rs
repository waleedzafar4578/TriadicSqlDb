use crate::lexical::Lexer;

pub mod lexical;
pub fn hello() {
    println!("Hello from compiler side");
}


pub fn sql_runner(query:&str)->String{
    let input=query.trim();
    let mut lexer=Lexer::new(input);
    let tokens=lexer.tokenize();
    format!("{:?}",tokens)
}