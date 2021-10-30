#[macro_use]
extern crate rbatis;
extern crate lazy_static;

mod database;
mod handlers;

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/*")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    database::connection::create().await;

    HttpServer::new(|| App::new()
        .service(index)
        .service(handlers::list_links::list_links)
        .service(handlers::get_link_data::get_link_data))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}