use actix_cors::Cors;
use actix_web::web::Json;
use actix_web::{get, post};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use compiler::sql_runner;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::{Arc, RwLock};
use std::{env, fmt, io};
use storagecontroller::BaseControl;
use triadic_error::FrontSendCode;
use UserAuth::structure_of_server::{appuser_to_file, file_to_appuser};
use UserAuth::{AppUsers, ClientResponseAccount, CreateAccountJson, InputData, LoginJson, OutputData, PassQueryJson, SelectDatabaseJson, TakeTokenJson, TFile, TokenResponse, User};

#[derive(Default, Clone)]
struct AppState {
    base_controls: Arc<RwLock<HashMap<String, BaseControl>>>,
}
#[post("/sdb")]
async fn select_db(input: web::Json<SelectDatabaseJson>) -> HttpResponse {
    let mut ret_ans = ClientResponseAccount {
        related_info: String::new(),
        token: String::new(),
    };

    //converting string to AppUser object
    let mut user_data: AppUsers = file_to_appuser();
    //checking this user is already exist or not.
    match user_data.check_token(&input.token) {
        None => {
            ret_ans.related_info = "Wrong Token!".to_string();
            Err("This username does not exist!".to_string())
        }
        Some(index) => {
            let user = user_data.users.get_mut(index).unwrap();

            match user.set_database(input.database_name.as_str()) {
                true => {}
                false => {}
            }
            Ok(user.cloned_token())
        }
    }
    .expect("TODO: panic message");

    appuser_to_file(user_data);

    HttpResponse::Ok()
        .content_type("application/json")
        .json(ret_ans)
}

#[post("/ln")]
async fn login(input: web::Json<LoginJson>) -> HttpResponse {
    //set up the returning value structure
    let mut ret_ans = ClientResponseAccount {
        related_info: String::new(),
        token: "".to_string(),
    };

    //converting string to AppUser object
    let mut user_data: AppUsers = file_to_appuser();
    println!("{:#?}", user_data);
    //checking this user is already exist or not.
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
    //set up the returning value structure
    let mut ret_ans = ClientResponseAccount{
        related_info: String::new(),
        token: String::new(),
    };

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
    let mut mem: String = String::new();
    let sts: FrontSendCode;
    (sts, mem) = sql_runner(data, con);
    // Create a modified OutputData with the reversed message
    OutputData {
        query_information: mem.to_string(),
        status: sts.to_string(),
    }
}
#[post("/pq")]
async fn process_query(input: Json<PassQueryJson>) -> HttpResponse {
    let mut ret_ans: OutputData = OutputData {
        query_information: "".to_string(),
        status: "".to_string(),
    };
    println!("processQuery Function");
    println!("{:?}",input);
    let mut base: BaseControl = BaseControl::new();
    //converting string to AppUser object
    let mut user_data = file_to_appuser();
    match user_data.get_path(&input.token) {
        None => {
            ret_ans.query_information = "SomeThing wrong with you token".to_string();
        }
        Some(path) => {
            base.initiate_database(path.as_str());
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
    let mut ret_ans  = TokenResponse{ find_token: false };
    //converting string to AppUser object
    let mut user_data = file_to_appuser();
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
    /*
    let client_ip = _req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_default();
    let user_agent = _req
        .headers()
        .get("user-agent")
        .map(|value| value.to_str().unwrap_or_default())
        .unwrap_or_default();


     */
    // Log or use client information as needed
    //println!("Client IP: {}", client_ip);
    //println!("User-Agent: {}", user_agent);

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
    let app_state = AppState::default();
    println!("Server start and on localhost:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(app_state.clone())) // Clone the Arc for each App instance
            .service(web::resource("/editor").route(web::get().to(editor)))
            .service(web::resource("/result").route(web::get().to(result)))
            .service(web::resource("/help").route(web::get().to(help)))
            //.service(web::resource("/process_json").route(web::post().to(handle_json)))
            .service(web::resource("/health_check").route(web::get().to(health_check)))
            .service(create_account)
            .service(login)
            .service(process_query)
            .service(token_check)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
