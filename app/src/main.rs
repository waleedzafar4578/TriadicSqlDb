use triadic_logic::Degree;

fn main() {
    //#1
    //compiler::hello();
    //server::hello();
    //storge::hello();
    //triadic_logic::hello();
    //#2
    let mut value:Degree=Degree::default();
    println!("Status of the value is: {}",value);
    value=Degree::F;
    println!("Status of the value is: {}",value);
    value=Degree::T;
    println!("Status of the value is: {}",value);

}
