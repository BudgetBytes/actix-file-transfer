use actix_web::{web, App, HttpServer};
use std::io::Result;
use std::path::Path;
use std::{env, fs};
mod extractors;
mod scopes;
use scopes::user::user_scope;

#[actix_web::main]
async fn main() -> Result<()> {
    let key = "FILE_PATH";
    env::set_var(key, "/tmp/.file-transfer");
    let file_path = env::var("FILE_PATH").unwrap();
    if !Path::new(&file_path).exists() {
        fs::create_dir(file_path)?;
    }

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::<String>::new("secret".to_owned()))
            .service(user_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
