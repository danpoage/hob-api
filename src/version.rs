use actix_web::web::{Json, Path};
use actix_web::HttpResponse;

//use crate::constants::APPLICATION_JSON;

#[get("/version")]
pub async fn display() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json("{\"version\":1.0.0}")
}