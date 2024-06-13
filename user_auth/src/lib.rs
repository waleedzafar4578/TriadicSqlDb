use serde::{Deserialize, Serialize};
pub mod structure_of_server;
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    token: String,
    username: String,
    password: String,
    pub unique_id: String,
    pub selected_database: String,
    timestamp: u64,
    expiration_time: u64,
}
pub struct TFile;
#[derive(Default,Debug, Serialize, Deserialize)]
pub struct AppUsers {
    pub users: Vec<User>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginJson {
    pub username: String,
    pub password: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectDatabaseJson {
    pub token: String,
    pub database_name: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GetDatabase {
    pub token: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SelectDatabaseRes {
    pub info: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PassQueryJson {
    pub token: String,
    pub query: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ClientResponseAccount {
    pub related_info: String,
    pub token:String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    pub message: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OutputData {
    pub query_information: String,
    pub status: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountJson {
    pub username: String,
    pub password: String,
    pub confirm:String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TakeTokenJson {
    pub token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub find_token: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FilesDownload {
   pub code:String
}
