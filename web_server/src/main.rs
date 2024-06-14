use actix_cors::Cors;
use actix_web::web::Json;
use actix_web::{get, post};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use compiler::sql_runner;
use serde_json::json;
use storagecontroller::BaseControl;
use triadic_error::FrontSendCode;
use user_auth::structure_of_server::{appuser_to_file, file_to_appuser};
use user_auth::{AppUsers, ClientResponseAccount, CreateAccountJson, FilesDownload, GetDatabase, LoginJson, OutputData, PassQueryJson, SelectDatabaseJson, SelectDatabaseRes, TakeTokenJson, TokenResponse, User};
use std::fs::File;
use std::io::Write;
use actix_files::NamedFile;
use std::path::Path;
use futures::io;
use zip::write::FileOptions;
use zip::ZipWriter;
use tokio::io::AsyncReadExt;
use tokio::fs;

#[get("/download_database/{id}")]
async fn download_database(req: HttpRequest,id: web::Path<(u32,)>) -> impl Responder {
    let user_id = id.into_inner().0;
    if user_id!=03160445748
    {
        return HttpResponse::InternalServerError().body("Your are not admin!)--<HEHE>");
    }
    let folder_path = "AllUserData";
    let zip_file_path = "database.zip";

    if let Err(e) = zip_dir(folder_path, zip_file_path).await {
        return HttpResponse::InternalServerError().body(format!("Failed to zip folder: {}", e));
    }

    match NamedFile::open(zip_file_path) {
        Ok(file) => file.into_response(&req),
        Err(_) => HttpResponse::InternalServerError().body("Could not open the zip file."),
    }
}

async fn zip_dir(src_dir: &str, dst_file: &str) -> io::Result<()> {
    let path = Path::new(src_dir);
    let file = File::create(dst_file)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let mut stack = vec![path.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let mut dir = fs::read_dir(&dir).await?;

        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();

            if path.is_file() {
                let relative_path = match path.strip_prefix(path.parent().unwrap()) {
                    Ok(rp) => rp,
                    Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
                };

                // Convert relative_path to a string with directory structure
                let relative_path_str = relative_path
                    .to_str()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid UTF-8 in path"))?;

                // Get the parent directory of the file
                let parent_dir = path.parent().unwrap();

                // Get the relative path of the parent directory
                let parent_relative_path = match parent_dir.strip_prefix(src_dir) {
                    Ok(prp) => prp,
                    Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
                };
                let parent_relative_path_str = parent_relative_path
                    .to_str()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid UTF-8 in path"))?;

                // Combine parent directory path and relative file path
                let full_path_in_zip =
                    format!("{}/{}", parent_relative_path_str, relative_path_str);

                zip.start_file(full_path_in_zip, options)?;
                let mut f = fs::File::open(&path).await?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).await?;
                zip.write_all(&buffer)?;
            } else if path.is_dir() {
                stack.push(path.clone());
            }
        }
    }

    zip.finish()?;
    Ok(())
}

#[get("/download_userdata/{id}")]
async fn download_userdata(req: HttpRequest,id: web::Path<(u32,)>) -> impl Responder {
    let user_id = id.into_inner().0;
    if user_id!=03160445748
    {
        return HttpResponse::InternalServerError().body("Your are not admin!)--<HEHE>");
    }
    match NamedFile::open_async("userdata").await {
        Ok(file) => file.into_response(&req),
        Err(_) => HttpResponse::InternalServerError().body("Could not open the file."),
    }
}


#[post("/gdb")]
async fn get_db(input: Json<GetDatabase>) -> HttpResponse {
    //println!("{:?}",input);
    let mut ret_ans:String ="None".to_string();
    //converting string to AppUser object
    let mut user_data: AppUsers = file_to_appuser();
    //checking this user is already exist or not.
    match user_data.check_token(&input.token) {
        None => {
            HttpResponse::InternalServerError().body("Token is wrong or expire.")
        }
        Some(index) => {
            let  user = user_data.users.get_mut(index).unwrap();
            ret_ans=user.selected_database.clone();
            HttpResponse::Ok()
                .content_type("application/json")
                .json(ret_ans)
        }
    }

}


#[post("/sdb")]
async fn select_db(input: Json<SelectDatabaseJson>) -> HttpResponse {
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
async fn login(input: Json<LoginJson>) -> HttpResponse {
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

        }
        Some(index) => {
            let user = user_data.users.get_mut(index).unwrap();

            if user.verify_password(&input.password) {
                user.generate_token();
                ret_ans.token = user.cloned_token();
                ret_ans.related_info="-1".to_string();
            } else {
                ret_ans.related_info = "Invalid password!".to_string();

            }
        }
    }
    appuser_to_file(user_data);
    HttpResponse::Ok()
        .content_type("application/json")
        .json(ret_ans)
}

#[post("/ca")]
async fn create_account(input: Json<CreateAccountJson>) -> HttpResponse {
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

            match std::fs::create_dir_all(&temp_user.unique_id){
                Ok(_) => {
                    println!("New user folder created");
                    user_data.users.push(temp_user);
                    appuser_to_file(user_data);
                }
                Err(_) => {
                    println!("Facing issue to create user folder.");
                    ret_ans.related_info = "Your account failed to create!System Error".to_string()
                }
            }

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
     //println!("{:#?}",con);

     if con.check_database_selected(){
         let mut temp=con.load_to_file();
         temp.set_db_true();
         //println!("----{:#?}",temp);
         (sts, mem) = sql_runner(data,&mut temp);
         //temp.save_to_file();
     }
     else {
         let mut temp=con;
         //println!("{:#?}",temp);
         (sts, mem) = sql_runner(data,&mut temp);
         //temp.save_to_file();
     }

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
    let mut user_data = file_to_appuser();
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
    println!("{:?}",ret_ans);
    if ret_ans.status =="Use".to_string(){
       match  user_data.check_token(&input.token){
           None => {}
           Some(_index) => {
               let user = user_data.users.get_mut(_index).unwrap();
               if user.set_database(ret_ans.query_information.as_str()){
                   ret_ans.query_information=format!("{} is selected!",ret_ans.query_information);
                   ret_ans.status="QP".to_string();
               }
               else {
                   ret_ans.query_information=format!("Failed to select this {}!",ret_ans.query_information);
                   ret_ans.status="QP".to_string();
               }
           }
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
            .service(download_userdata)
            .service(download_database)
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
