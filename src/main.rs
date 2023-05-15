#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};

//use structs::Link;
//use enums::*;

mod constants;
mod structs;
mod enums;
mod traits;
mod version;
mod products;
mod repos;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

/* 
    let l = Link{
        link_source: enums::LinkSource::HallOfBeorn,
        link_type: LinkType::Blog,
        url: String::from("http://hallofbeorn.wordpress.com")
    };
*/

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(version::get)
            .service(products::list)
            .service(products::get)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
