//use triadic_logic::degree::Degree;
//use triadic_logic::tri_var::TriData;

use storge::column::Column;
use storge::table::Table;
use triadic_logic::datatype::AttributeType;
use triadic_logic::datatype::AttributeType::{TInt, TString};
use triadic_logic::degree::Degree;
use triadic_logic::degree::Degree::{F, L, T};

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

    //#3
/*
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

 */
    //#4
    /*
    let mut value=Column::new("Id",AttributeType::TInt);
    value.set_int_cell(1,Degree::T);
    value.set_int_cell(2,Degree::L);
    value.set_int_cell(3,Degree::T);
    value.set_int_cell(4,Degree::F);
    value.set_int_cell(5,Degree::L);
    println!("Full Column:\n{}",value);
    println!("Specific value:{}",value.get_column_data(2).unwrap());

     */
    //#5
    /*
    let mut value=Table::new("Student");
    value.add_column(Column::new("id",TInt));
    value.add_column(Column::new("name",TString));
    value.add_col_data("id","1",T);
    value.add_col_data("id","2",L);
    value.add_col_data("id","3",F);
    value.add_col_data("name","wali",T);

    println!("{}",value);

     */
    //#6
    storagecontroller::hello();
    let mut value=Table::new("Student");
    value.add_column(Column::new("id",TInt));
    value.add_column(Column::new("name",TString));
    value.add_col_data("id","1",T);
    value.add_col_data("id","2",L);
    value.add_col_data("id","3",F);
    value.add_col_data("name","wali",T);

    value.save_to_file("../".to_string());
    let mut checking:Table=Table::new("");
    checking=checking.load_to_file("../".to_string(),"Student".to_string());
    println!("Load from the file: {}",checking);

}
