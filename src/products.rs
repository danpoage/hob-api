use actix_web::HttpResponse;
use actix_web::web::{Path};
//use serde::{Deserialize, Serialize};

use crate::constants::APPLICATION_JSON;
use crate::repos::ProductRepo;
//use crate::structs::Product;

#[get("/products")]
pub async fn list() -> HttpResponse {

    let products = ProductRepo::list();

    HttpResponse::Ok().content_type(APPLICATION_JSON).json(products)
}

#[get("/products/{code}")]
pub async fn get(path: Path<(String,)>) -> HttpResponse {

    let code = path.into_inner();

    let products = ProductRepo::list();

    let result = products.iter().find(|&p| p.code.as_str() == code.0);

    if result.is_some() {
        println!("found product for code {}", code.0);
        return HttpResponse::Ok().content_type(APPLICATION_JSON).json(Some(result));
    }
    
    HttpResponse::NotFound().content_type(APPLICATION_JSON).json("{\"status:\": \"not found\"}")
}