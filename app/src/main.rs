use compiler::sql_runner;
use storagecontroller::BaseControl;

fn main() {
    //#1
    //compiler::hello();
    //server::hello();
    //storage::hello();
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
    /*
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

     */
    //#7
    /*
       //creating main controller for client
       let mut subhan=BaseControl::new();


       //initiate the database path
       subhan.initiate_database("../Testing/");
       //create_parse database
       subhan.create_the_database("subhan");
       subhan.create_the_database("Hospital");
       subhan.create_the_database("UCP");
       subhan.create_the_database("UMT");
       subhan.create_the_database("LUMS");
       //search database
       if subhan.find_this_database("subhan"){
           println!("This database is found");
       }
       //list of database
       subhan.list_down_the_name_database();
       //use database
       subhan.use_this_database("UCP");
       //rename database
       subhan.rename_the_database("ali");
       //remove database
       subhan.remove_the_database();
       //make table
       subhan.use_this_database("UMT");
       //insert data into table
       subhan.add_table("UMT_STUDENT", vec!["ID","Name","Email"], vec![AttributeType::TInt,AttributeType::TString,AttributeType::TString]);
       subhan.insert_to_table("UMT_STUDENT", "ID", "1", Degree::T);
       subhan.insert_to_table("UMT_STUDENT", "Name", "Jon", Degree::T);
       subhan.insert_to_table("UMT_STUDENT", "Email", "jon11@gmial.com", Degree::F);

       for i in 10..=40 {
           let id = i.to_string();
           let name = format!("Student{}", i);
           let email = format!("student{}@example.com", i);

           subhan.insert_to_table("UMT_STUDENT", "ID", &id, Degree::T);
           subhan.insert_to_table("UMT_STUDENT", "Name", &name, Degree::L);
           subhan.insert_to_table("UMT_STUDENT", "Email", &email, Degree::F);
       }
       //show full data table
       //show selective column
       println!("{}",subhan.get_column("UMT_STUDENT".to_string(), "Name".to_string()).unwrap());
       //delete table
       subhan.drop_table("UMT_STUDENT");
       //

       println!("{}",subhuman);

    */
    //#8
    /*
    let mut value: BaseControl = BaseControl::new();
    let query = vec!["CREATE DATABASE ucp;"];

    for i in query {
        let mut answer = sql_runner(i, &mut value);
        println!("{}", answer);
    }

     */
    triadic_error::hello();
}
