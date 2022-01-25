#![deny(warnings)]
#[macro_use]
extern crate rbatis;
extern crate lazy_static;

mod database;
mod api;
mod authentication;
mod logger;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web, http};
use actix_cors::Cors;
use actix_web_httpauth::middleware::HttpAuthentication;
use log::LevelFilter;

static LOGGER: logger::Logger = logger::Logger;

use crate::authentication::validate_authorization;

#[get("/api/test_connection")]
async fn test_connection() -> impl Responder {
    HttpResponse::Ok().body("Connected")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let set_logger_result = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug));
    if set_logger_result.is_err(){
        panic!("{}", set_logger_result.unwrap_err().to_string());
    }

    log::info!("initialization");
    log::info!("Connecting to database");
    database::create_connection().await;

    // log::info!("Building ssl context");
    //
    // let mut builder = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();
    // builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    log::info!("Creating http server");
    HttpServer::new(|| {
        let _cors = Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allowed_headers(vec![http::header::AUTHORIZATION])
                        .supports_credentials();

        let auth = HttpAuthentication::bearer(validate_authorization);
        
        App::new()
        //.wrap(cors)
        .wrap(auth)
        

        .service(test_connection)
        .service(api::v1::add_link)
        .service(api::v1::get_link)
        .service(api::v1::update_link)
        .service(api::v1::remove_link)
        .service(api::v1::add_opengraph_tag)
        .service(api::v1::get_opengraph_tag)
        .service(api::v1::update_opengraph_tag)
        .service(api::v1::remove_opengraph_tag)
        .service(api::v1::list_links)
        .service(api::v1::list_links_where)
        .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
        .bind("0.0.0.0:8079")?
       // .bind_openssl("0.0.0.0:8080", builder)?
        .run()
        .await
}

