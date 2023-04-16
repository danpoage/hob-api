//use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::constants::APPLICATION_JSON;

#[derive(Debug, Deserialize, Serialize)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
    stamp: DateTime<Utc>
}

impl Version {
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major: major,
            minor: minor,
            patch: patch,
            stamp: Utc::now()
        }
    }
}

/* 
impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Version", 4)?;
        state.serialize_field("major", &self.major)?;
        state.serialize_field("minor", &self.minor)?;
        state.serialize_field("patch", &self.patch)?;
        state.serialize_field("stamp", &self.stamp)?;
        state.end()
    }
}
*/

#[get("/version")]
pub async fn display() -> HttpResponse {
    let ver = Version::new(0, 0, 1);

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(ver)
}