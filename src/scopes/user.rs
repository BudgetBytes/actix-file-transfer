use crate::extractors::authentication_token::{AuthenticationToken, Claims};
use actix_files;
use actix_multipart::Multipart;
use actix_web::{
    http::header::{ContentDisposition, DispositionType},
    web, Error, HttpRequest, HttpResponse, Scope, Responder,
};
use chrono::{Duration, Utc};
use futures_util::StreamExt as _;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/encode", web::post().to(encode_token))
        .route("/upload", web::post().to(handle_upload))
        .route("/getfiles", web::get().to(handle_get_files))
        .route("/download/{filename:.*}", web::get().to(handle_download))
}

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct EncodeResponse {
    message: String,
    token: String,
}

#[derive(Deserialize)]
struct UserCredentials {
    username: String,
    hash: String,
}

async fn encode_token(
    user_credentials: web::Json<UserCredentials>,
    secret: web::Data<String>,
) -> HttpResponse {
    let username = user_credentials.username.clone();
    let hash = user_credentials.hash.clone();
    if hash != "e548ea88bdb6cfd05ee5258bba0b7ac8ce6a5676035bada4039b6c641e0a6840" {
        return HttpResponse::BadRequest().json(EncodeResponse {
            message: "Wrong credentials".to_owned(),
            token: "".to_string(),
        });
    }

    let exp: usize = (Utc::now() + Duration::days(365)).timestamp() as usize;
    let claims: Claims = Claims { username, exp };
    let token: String = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();
    HttpResponse::Ok().json(EncodeResponse {
        message: "Successfully created account.".to_owned(),
        token: token,
    })
}

async fn handle_upload(
    _auth_token: AuthenticationToken,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;

        // Get the filename from the content disposition header
        let content_disposition = field.content_disposition();

        let filename = content_disposition
            .get_filename()
            .map_or("unnamed_file".to_string(), |name| name.to_string());

        let file_path = env::var("FILE_PATH").unwrap_or("".to_string());
        // Create a file with the given filename
        let mut file = File::create(format!("{}/{}", file_path, filename))?;

        // Iterate over the chunks and write them to the file
        while let Some(chunk) = field.next().await {
            let chunk = chunk?;
            file.write_all(&chunk)?;
        }
    }

    Ok(HttpResponse::Ok().json(Response {
        message: "File saved successfully".to_owned(),
    }))
}

#[derive(Serialize)]
struct FileList {
    files: Vec<String>,
}

async fn handle_get_files(_auth_token: AuthenticationToken) -> impl Responder {
    let mut files: Vec<String> = Vec::new();
    let file_path = env::var("FILE_PATH").unwrap();
    let paths = fs::read_dir(file_path).unwrap();
    for path in paths {
        files.push(path.unwrap().file_name().into_string().unwrap())
    }
    return web::Json(FileList {
        files: files
    })
}

async fn handle_download(
    req: HttpRequest,
    _auth_token: AuthenticationToken,
) -> Result<actix_files::NamedFile, Error> {
    let path: String = req.match_info().query("filename").parse().unwrap();
    let file_name = path.trim_matches('"');
    let file_path = format!("/tmp/.file-transfer/{file_name}");
    let file = actix_files::NamedFile::open(file_path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}
