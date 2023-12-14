use actix_web::{web, App, HttpServer};
use std::path::Path;
use std::{env, fs};
use std::{
    fs::File,
    io::{Read as _, Result},
};
mod extractors;
mod scopes;
use openssl::{
    pkey::{PKey, Private},
    ssl::{SslAcceptor, SslMethod},
};
use scopes::user::user_scope;

#[actix_web::main]
async fn main() -> Result<()> {
    let key = "FILE_PATH";
    env::set_var(key, "/tmp/.file-transfer");
    let file_path = env::var("FILE_PATH").unwrap();
    if !Path::new(&file_path).exists() {
        fs::create_dir(file_path)?;
    }

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key(&load_encrypted_private_key())
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::<String>::new("secret".to_owned()))
            .service(user_scope())
			.service(
				actix_files::Files::new("/", "./static")
					.show_files_listing()
					.index_file("index.html")
					.use_last_modified(true),
			)
    })
    .bind_openssl("0.0.0.0:8080", builder)?
    .run()
    .await
}

fn load_encrypted_private_key() -> PKey<Private> {
    let mut file = File::open("key.pem").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    PKey::private_key_from_pem_passphrase(&buffer, b"password").unwrap()
}
