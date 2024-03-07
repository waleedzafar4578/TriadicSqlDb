use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use compiler::sql_runner;
use serde::{Deserialize, Serialize};
use storagecontroller::BaseControl;

#[derive(Debug, Serialize, Deserialize)]
struct InputData {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutputData {
    reversed_message: String,
}
fn process_json_data(data: InputData, con: &BaseControl) -> OutputData {
    // Perform any modifications on the data if needed
    // For example, reverse the message
    let mut mem: String = data.message;
    let mut value = BaseControl::new();
    mem = sql_runner(mem.as_str(), &mut value);
    // Create a modified OutputData with the reversed message
    let modified_output = OutputData {
        reversed_message: mem.to_string(),
    };

    return modified_output;
}
async fn handle_json(
    input: web::Json<InputData>,
    base_control: web::Data<BaseControl>,
) -> Result<HttpResponse> {
    // Deserialize the received JSON string
    let input_data: InputData = input.into_inner();
    println!("Received JSON Data: {:?}", input_data);
    let base_control_ref = base_control.get_ref();

    // Perform modifications on the received data
    let modified_data = process_json_data(input_data, base_control_ref);

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
    HttpResponse::Ok().body("Result Page")
}

async fn help() -> impl Responder {
    HttpResponse::Ok().body("Help Page")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let val = BaseControl::new();
    let base = web::Data::new(val);
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(base.clone())
            .service(web::resource("/editor").route(web::get().to(editor)))
            .service(web::resource("/result").route(web::get().to(result)))
            .service(web::resource("/help").route(web::get().to(help)))
            .service(web::resource("/process_json").route(web::post().to(handle_json)))
            .service(web::resource("/health_check").route(web::get().to(health_check)))
        // Add this line
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
