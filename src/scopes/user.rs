use crate::extractors::authentication_token::{AuthenticationToken, Claims};
use actix_web::{web, HttpResponse, Scope};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

pub fn user_scope() -> Scope {
    web::scope("/user")
        .route("/encode", web::post().to(encode_token))
        .route("/protected", web::get().to(protected_route))
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

// Example on how it probably should be handled
async fn protected_route(auth_token: AuthenticationToken) -> HttpResponse {
    println!("{:#?}", auth_token);
    HttpResponse::Ok().json(Response {
        message: "Authorized".to_owned(),
    })
}
