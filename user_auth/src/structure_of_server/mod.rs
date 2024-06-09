use crate::{AppUsers, TFile};
pub mod appuser_fn;
pub mod tfile_fn;
pub mod user_fn;
pub fn file_to_appuser() -> AppUsers {
    // Stream object used to read from the file
    let mut stream = String::new();
    if let Err(err) = TFile::read_from_file().map(|vst| stream = vst) {
        println!("Unable to read file: {}", err);
    }
    // Converting string to AppUser object
    let user_data: AppUsers = serde_json::from_str(&stream).unwrap_or_else(|err| {
        println!("Error parsing JSON: {}", err);
        // You may want to return a default value or handle the error in another way
        Default::default()
    });
    
    user_data
}
pub fn appuser_to_file(obj: AppUsers) {
    match serde_json::to_string_pretty(&obj) {
        Ok(json_string) => match TFile::write_to_file(&json_string) {
            Ok(_) => println!("Successfully wrote to file!"),
            Err(err) => println!("Failed to write to file: {}", err),
        },
        Err(err) => println!("Failed to serialize UserData to JSON: {}", err),
    }
}
