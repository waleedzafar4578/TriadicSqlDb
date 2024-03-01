use triadic_logic::degree::Degree;
use triadic_logic::tri_var::TriData;

fn main() {
    //#1
    //compiler::hello();
    //server::hello();
    //storge::hello();
    //triadic_logic::hello();

    //#2
    /*let mut value:Degree=Degree::default();
    println!("Status of the value is: {}",value);
    value=Degree::F;
    println!("Status of the value is: {}",value);
    value=Degree::T;
    println!("Status of the value is: {}",value);

     */
    let value=TriData::t_int(40,Degree::T);
    println!("Integer things: {}",value);
    let value=TriData::t_bool(true,Degree::T);
    println!("Bool things: {}",value);
    let value=TriData::t_char('A',Degree::T);
    println!("Character things: {}",value);
    let value=TriData::t_float(45.5,Degree::T);
    println!("Float things: {}",value);
    let value=TriData::t_string("waleed".to_string(),Degree::T);
    println!("String things: {}",value);


}
