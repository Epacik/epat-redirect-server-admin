use actix_web::{get, web, App, HttpServer, Responder};
use rbatis::crud::CRUD;
use links::Links;
use crate::database::links;

#[get("/list_links")]
pub(crate) async fn list_links() -> impl Responder {
    let result : Vec<Links> = crate::database::RB.fetch_list().await.unwrap();

    serde_json::to_string_pretty(&result).unwrap()
}