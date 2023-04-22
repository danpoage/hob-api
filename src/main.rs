#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};

//use crate::models::{Card,Hero,Ally};

mod constants;
mod entities;
mod stats;
mod traits;
mod version;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    //let c = Card::new();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(version::display)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
