use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use compiler::sql_runner;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use storagecontroller::BaseControl;
use triadic_error::{Compiler, FrontSendCode};
#[derive(Default, Clone)]
struct AppState {
    base_controls: Arc<RwLock<HashMap<String, BaseControl>>>,
}
#[derive(Debug, Serialize, Deserialize)]
struct InputData {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutputData {
    reversed_message: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileData {
    coming: Vec<String>,
    going: Vec<String>,
}
fn process_json_data(data: InputData, con: &mut BaseControl) -> OutputData {
    // Perform any modifications on the data if needed,
    // For example, reverse the message
    let mut mem: String = data.message;
    let mut sts:FrontSendCode;
    (sts,mem) = sql_runner(mem.as_str(), con);
    // Create a modified OutputData with the reversed message
    let modified_output = OutputData {
        reversed_message: mem.to_string(),
        status: sts.to_string(),
    };

    return modified_output;
}
async fn handle_json(
    input: web::Json<InputData>,
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let input_data: InputData = input.into_inner();
    println!("Received JSON Data: {:?}", input_data);

    let client_ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_default();

    let base_controls = data.base_controls.clone(); // Clone the Arc for access
    let mut base_controls = base_controls.write().unwrap(); // Acquire a write lock

    let base_control = base_controls
        .entry(client_ip.clone())
        .or_insert_with(|| BaseControl::new());
    let path = format!("../Testing/{}/", client_ip);
    base_control.initiate_database(path.as_str());
    // Perform modifications on the received data
    let modified_data = process_json_data(input_data, base_control);
    println!("{}", base_control);
    // Serialize the modified data to a JSON response
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(modified_data))
}

async fn health_check(req: HttpRequest) -> impl Responder {
    let client_ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_default();
    let user_agent = req
        .headers()
        .get("user-agent")
        .map(|value| value.to_str().unwrap_or_default())
        .unwrap_or_default();

    // Log or use client information as needed
    println!("Client IP: {}", client_ip);
    println!("User-Agent: {}", user_agent);

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

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(app_state.clone())) // Clone the Arc for each App instance
            .service(web::resource("/editor").route(web::get().to(editor)))
            .service(web::resource("/result").route(web::get().to(result)))
            .service(web::resource("/help").route(web::get().to(help)))
            .service(web::resource("/process_json").route(web::post().to(handle_json)))
            .service(web::resource("/health_check").route(web::get().to(health_check)))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
