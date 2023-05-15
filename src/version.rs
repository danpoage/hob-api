//use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::constants::APPLICATION_JSON;

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    name: String,
    major: u8,
    minor: u8,
    patch: u8,
    stamp: DateTime<Utc>,
}

impl Version {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major: major,
            minor: minor,
            patch: patch,
            stamp: Utc::now(),
            name: format!("{}.{}.{}", major, minor, patch),
        }
    }
}

#[get("/version")]
pub async fn get() -> HttpResponse {
    let ver = Version::new(0, 0, 1);

    HttpResponse::Ok().content_type(APPLICATION_JSON).json(ver)
}
