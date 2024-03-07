use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use compiler::sql_runner;
use storagecontroller::BaseControl;

#[derive(Debug, Serialize, Deserialize)]
struct InputData {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutputData {
    reversed_message: String,
}
fn process_json_data(data: InputData) -> OutputData {
    // Perform any modifications on the data if needed
    // For example, reverse the message
    let mut mem:String=data.message;
    let mut value=BaseControl::new();
    mem=sql_runner(mem.as_str(),&mut value);
    // Create a modified OutputData with the reversed message
    let modified_output = OutputData {
        reversed_message: mem.to_string(),
    };

    return modified_output
}
async fn handle_json(input: web::Json<InputData>) -> Result<HttpResponse> {
    // Deserialize the received JSON string
    let input_data: InputData = input.into_inner();
    println!("Received JSON Data: {:?}", input_data);

    // Perform modifications on the received data
    let modified_data = process_json_data(input_data);

    // Serialize the modified data to a JSON response
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(modified_data))
}

async fn health_check() -> impl Responder {
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
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(web::resource("/editor").route(web::get().to(editor)))
            .service(web::resource("/result").route(web::get().to(result)))
            .service(web::resource("/help").route(web::get().to(help)))
            .service(web::resource("/process_json").route(web::post().to(handle_json)))
            .service(web::resource("/health_check").route(web::get().to(health_check))) // Add this line
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
