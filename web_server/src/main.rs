use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    let mem:String=data.message;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(web::resource("/process_json").route(web::post().to(handle_json)))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}