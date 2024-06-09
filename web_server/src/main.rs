use actix_cors::Cors;
use actix_web::web::Json;
use actix_web::{get, post};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use compiler::sql_runner;
use serde_json::json;
use storagecontroller::BaseControl;
use triadic_error::FrontSendCode;
use user_auth::structure_of_server::{appuser_to_file, file_to_appuser};
use user_auth::{AppUsers, ClientResponseAccount, CreateAccountJson, GetDatabase, LoginJson, OutputData, PassQueryJson, SelectDatabaseJson, SelectDatabaseRes, TakeTokenJson, TokenResponse, User};


#[get("/gdb")]
async fn get_db(input: web::Json<GetDatabase>) -> HttpResponse {
    println!("{:?}",input);
    let mut ret_ans:Vec<String> =vec![];

    //converting string to AppUser object
    let mut user_data: AppUsers = file_to_appuser();
    //checking this user is already exist or not.
    match user_data.check_token(&input.token) {
        None => {
            ret_ans.push("Wrong Token!".to_string());
        }
        Some(index) => {
            let user = user_data.users.get_mut(index).unwrap();
            let mut t=BaseControl::new();
            t.initiate_database(user.get_path().as_str());
            ret_ans=t.list_down_the_name_database();
        }
    }
    
    //appuser_to_file(user_data);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(ret_ans)
}


#[post("/sdb")]
async fn select_db(input: web::Json<SelectDatabaseJson>) -> HttpResponse {
    println!("{:?}",input);
    let mut ret_ans = SelectDatabaseRes {
        info: String::new(),
    };

    //converting string to AppUser object
    let mut user_data: AppUsers = file_to_appuser();
    //checking this user is already exist or not.
    match user_data.check_token(&input.token) {
        None => {
            ret_ans.info = "Wrong Token!".to_string();
        }
        Some(index) => {
            let user = user_data.users.get_mut(index).unwrap();
            let db_name=input.database_name.to_string();
            if user.set_database(db_name.as_str()) {
                ret_ans.info=format!("{}  database is selected!",input.database_name);
                Ok(())
            } else {
                Err("Failed to set database".to_string())
            }.expect("TODO: panic message");
           
        }
    }
 
    //println!("{:#?}",user_data);
    appuser_to_file(user_data);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(ret_ans)
}

#[post("/ln")]
async fn login(input: web::Json<LoginJson>) -> HttpResponse {
    println!("{:?}",input);
    //set up the returning value structure
    let mut ret_ans = ClientResponseAccount {
        related_info: String::new(),
        token: "".to_string(),
    };

    //converting string to AppUser object
    let mut user_data: AppUsers = file_to_appuser();
    //println!("{:#?}", user_data);
    //checking this user already exists or not.
    match user_data.check_username_exist(&input.username) {
        None => {
            ret_ans.related_info = "This username does not exist!".to_string();
            Err("This username does not exist!".to_string())
        }
        Some(index) => {
            let user = user_data.users.get_mut(index).unwrap();

            if user.verify_password(&input.password) {
                user.generate_token();
                ret_ans.token = user.cloned_token();
                ret_ans.related_info="-1".to_string();
                Ok(user.cloned_token())
            } else {
                ret_ans.related_info = "Invalid password!".to_string();
                Err("Invalid password!".to_string())
            }
        }
    }
    .expect("TODO: panic message");

    appuser_to_file(user_data);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(ret_ans)
}

#[post("/ca")]
async fn create_account(input: web::Json<CreateAccountJson>) -> HttpResponse {
    println!("{:?}",input);
    //set up the returning value structure
    let mut ret_ans = ClientResponseAccount{
        related_info: String::new(),
        token: String::new(),
    };
    println!("{:?}",input);
    
    //converting string to AppUser object
    let mut user_data = file_to_appuser();

    //checking this user is already exist or not.
    match user_data.check_username_exist(&input.username) {
        None => {
            //temporary object for access User functions
            let mut temp_user = User::default();
            ret_ans.related_info = temp_user.set(&input.username, &input.password,&input.confirm);

            user_data.users.push(temp_user);
            appuser_to_file(user_data);
        }
        Some(_) => ret_ans.related_info = "This username is already exist!".to_string(),
    }
    //------------------------------------------

    HttpResponse::Ok()
        .content_type("application/json")
        .json(ret_ans)
}

 fn process_json_data(data: &str, con: &mut BaseControl) -> OutputData {
     
    let  mem: String ;
    let sts: FrontSendCode;
    
    let mut temp=con.load_to_file();
    //println!("{:#?}",temp);
    (sts, mem) = sql_runner(data,&mut temp);
    //println!("{:#?}",con);
    
    temp.save_to_file();
    // Create a modified OutputData with the reversed message
    OutputData {
        query_information: mem.to_string(),
        status: sts.to_string(),
    }
}

#[post("/pq")]
async fn process_query(input: Json<PassQueryJson>) -> HttpResponse {
    println!("{:?}",input);
    let mut ret_ans: OutputData = OutputData {
        query_information: "".to_string(),
        status: "".to_string(),
    };
    println!("Processing the User Query......!");
    //println!("{:?}",input);
    let mut base: BaseControl = BaseControl::new();
    //converting string to AppUser object
    let user_data = file_to_appuser();
    match user_data.get_path_db(&input.token) {
        None => {
            println!("User Token is expired!");
            ret_ans.query_information = "SomeThing wrong with you token".to_string();
        }
        Some((path,db)) => {
            base.initiate_database(path.as_str());
            if !db.is_empty(){
                base.use_this_database(db.as_str());
            }
            ret_ans = process_json_data(input.query.as_str(), &mut base);
        }
    }
    appuser_to_file(user_data);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(ret_ans)
}
#[post("/checkt")]
async fn token_check(input: Json<TakeTokenJson>)->impl Responder{
    println!("{:?}",input);
    let mut ret_ans  = TokenResponse{ find_token: false };
    //converting string to AppUser object
    let user_data = file_to_appuser();
    match user_data.check_token(&input.token) {
        None => {
            ret_ans.find_token=false;
        }
        Some(_) => {
            ret_ans.find_token=true;
        }
    }
    println!("[Client Token:{}]<->[And server answer:{}]",input.token,ret_ans.find_token);
    HttpResponse::Ok()
        .content_type("application/json")
        .json(ret_ans)
}
async fn health_check(_req: HttpRequest) -> impl Responder {
    
    HttpResponse::Ok().finish()
}

async fn editor() -> impl Responder {
    HttpResponse::Ok().body("Editor Page")
}

async fn result() -> impl Responder {
    let result_data = json!({
        "message": "Resul history from server"
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(result_data)
}

async fn help() -> impl Responder {
    let help_data = json!({
        "message": "Help Page from server"
    });

    HttpResponse::Ok()
        .content_type("application/json")
        .json(help_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    
    println!("Server start and on 0.0.0.0:4000");
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(web::resource("/editor").route(web::get().to(editor)))
            .service(web::resource("/result").route(web::get().to(result)))
            .service(web::resource("/help").route(web::get().to(help)))
            .service(web::resource("/health_check").route(web::get().to(health_check)))
            .service(create_account)
            .service(login)
            .service(process_query)
            .service(token_check)
            .service(select_db)
            .service(get_db)
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
