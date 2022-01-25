mod link;
mod link_with_opengraph;
mod opengraph;

use std::str::FromStr;
use actix_web::{get, put, patch, delete, web,Responder, HttpResponse};
use rbatis::crud::{CRUD, Skip};
use crate::api::v1::link::Link;
use crate::api::v1::link_with_opengraph::LinkWithOpengraph;
use crate::database::{Links, OpenGraph, RB};

//#region link responses
#[put("/api/v1/add_link")]
pub(crate) async fn add_link(link_to_add: web::Json<Link>) -> impl Responder {

    log::info!("Adding link \n{} -> {}", &link_to_add.path, &link_to_add.target);
    let result = RB.save(&Links {
        lnk_id: None,
        lnk_path: link_to_add.path.to_string(),
        lnk_target: link_to_add.target.to_string(),
        lnk_hide_target: link_to_add.hide_target,
    }, &[Skip::Column("lnk_id")]).await;

    if result.is_err() {
        let err = result.unwrap_err();
        log::error!("not found \n{}", err);
        return HttpResponse::BadRequest().body(err.to_string());
    }

    log::info!("Added");
    HttpResponse::Created().body("Added")
}

#[get("/api/v1/get_link/{path:.*}")]
pub(crate) async fn get_link(web::Path(path): web::Path<String>) -> impl Responder {
    log::info!("Getting link data \n{}", &path);

    let option: Option<Links>;
    let id = u32::from_str(&*path);

    if id.is_err() {
        option = RB.fetch_by_column("lnk_path", &path).await.unwrap();
    }
    else {
        option = RB.fetch_by_column("lnk_id", &id.unwrap()).await.unwrap();
    }

    if option.is_none() {
        log::info!("Link not found");
        return HttpResponse::NotFound().body("Link not found");
    }

    let link = option.unwrap();

    let opengraph_wrapper = RB.new_wrapper().eq("log_link_id", link.lnk_id.unwrap()).order_by(true, &["log_tag"]);

    let opengraph : Vec<crate::database::OpenGraph> = RB.fetch_list_by_wrapper(opengraph_wrapper).await.unwrap();
    //RB.fetch_list_by_column("log_link_id",&[link.lnk_id.unwrap()]).await.unwrap();

    let result = LinkWithOpengraph {
        id: link.lnk_id.unwrap(),
        path: link.lnk_path,
        target: link.lnk_target,
        hide_target: link.lnk_hide_target,
        opengraph_tags: opengraph,
    };

    let json = serde_json::to_string_pretty(&result).unwrap();

    log::info!("link found: \n{}", &json);

    HttpResponse::Ok().body(json)
}

#[patch("/api/v1/update_link")]
pub(crate) async fn update_link(link_to_update: web::Json<crate::database::Links>) -> impl Responder {
    log::info!("Updating link \nlink id: {}\nlink: {}\ntarget: {}", &link_to_update.lnk_id.unwrap(),  &link_to_update.lnk_path,  &link_to_update.lnk_target);

    let result = RB.update_by_column("lnk_id", & Links{
        lnk_id: link_to_update.lnk_id,
        lnk_path: link_to_update.lnk_path.to_string(),
        lnk_target: link_to_update.lnk_target.to_string(),
        lnk_hide_target: link_to_update.lnk_hide_target,
    }).await;

    if result.is_err() {
        let err = result.unwrap_err().to_string();
        log::warn!("couldn't update\n{}", &err);
        return HttpResponse::Conflict().body(err);
    }

    log::info!("Updated");
    HttpResponse::Ok().body("Updated")
}

#[delete("/api/v1/remove_link/{path:.*}")]
pub(crate) async fn remove_link(web::Path(path): web::Path<String>) ->  impl Responder {

    log::info!("Removing link \n{}", &path);
    let option: Option<Links>;
    let id_for_finding = u32::from_str(&*path);

    if id_for_finding.is_err() {
        option = RB.fetch_by_column("lnk_path", &path).await.unwrap();
    }
    else {
        option = RB.fetch_by_column("lnk_id", &id_for_finding.unwrap()).await.unwrap();
    }

    if option.is_none(){
        log::warn!("Link was not removed because it does not exist");
        return HttpResponse::BadRequest().body("Link was not removed because it does not exist");
    }

    log::info!("Removing linked OpenGraph tags");
    let id = option.unwrap().lnk_id.unwrap();

    let mut result =  RB.remove_batch_by_column::<OpenGraph, _>("log_link_id", &[id]).await;

    if result.is_err(){
        let err = result.unwrap_err().to_string();
        log::warn!("could not remove linked tags\n{}", &err);
        return HttpResponse::BadRequest().body(err);
    }

    log::info!("removing link");
    result = RB.remove_by_column::<Links, _>("lnk_id", &id).await;

    if result.is_err(){
        let err = result.unwrap_err().to_string();
        log::warn!("could not remove link \n{}", &err);
        return HttpResponse::BadRequest().body(err);
    }

    log::info!("Removed");
    HttpResponse::Ok().body("Removed")
}

//#endregion

//#region opengraph

#[put("/api/v1/add_opengraph_tag")]
pub(crate) async fn add_opengraph_tag(og_tag: web::Json<opengraph::OpenGraph>) -> impl Responder {

    log::info!("Adding opengraph tag \nlink id: {}\ntag: {}\ncontent: {}", &og_tag.id, &og_tag.tag, &og_tag.content);
    let result = RB.save(&OpenGraph {
        log_id: None,
        log_link_id: og_tag.id,
        log_tag: og_tag.tag.to_string(),
        log_content: og_tag.content.to_string()
    }, &[Skip::Column("log_id")]).await;

    if result.is_err() {
        return HttpResponse::Conflict().body(result.unwrap_err().to_string());
    }
    HttpResponse::Ok().body("Added")
}

#[get("/api/v1/get_opengraph_tag/{id}")]
pub(crate) async fn get_opengraph_tag(web::Path(id): web::Path<u32>) -> impl Responder {
    log::info!("Getting opengraph tag data \n{}", &id);

    let tag_result = RB.fetch_by_column::<OpenGraph, _>("log_id", &id).await;

    if tag_result.is_err() {
        let err = tag_result.unwrap_err().to_string();
        log::warn!("tag not found\n{}", &err);
        return HttpResponse::NotFound().body(err);
    }

    let tag = tag_result.unwrap();

    let json_result = serde_json::to_string_pretty(&tag);

    if json_result.is_err() {
        let err:String = json_result.unwrap_err().to_string();
        log::error!("Error creating json string\n{}", &err);
        return HttpResponse::InternalServerError().body(err);
    }


    HttpResponse::Ok().body(json_result.unwrap())
}

#[patch("/api/v1/update_opengraph_tag")]
pub(crate) async fn update_opengraph_tag(tag_to_update: web::Json<crate::database::OpenGraph>) -> impl Responder {
    let result = RB.update_by_column("log_id", & OpenGraph{
        log_id: tag_to_update.log_id,
        log_link_id: tag_to_update.log_link_id,
        log_tag: tag_to_update.log_tag.to_string(),
        log_content: tag_to_update.log_content.to_string(),
    }).await;

    if result.is_err() {
        return HttpResponse::Conflict().body(result.unwrap_err().to_string());
    }

    HttpResponse::Ok().body("Updated")
}

#[delete("/api/v1/remove_opengraph_tag/{id}")]
pub(crate) async fn remove_opengraph_tag(web::Path(id): web::Path<u32>) ->  impl Responder {

    let result = RB.remove_by_column::<OpenGraph, _>("log_id", &id).await;

    if result.is_err(){
        return HttpResponse::BadRequest().body(result.unwrap_err().to_string());
    }

    HttpResponse::Ok().body("Removed")
}

//#endregion

//#region list of links

#[get("/api/v1/list_links")]
pub(crate) async fn list_links() -> impl Responder {
    let links_wrapper = RB.new_wrapper()
        .order_by(true, &["lnk_path"]);
    let links_result : Result<Vec<Links>, _> = RB.fetch_list_by_wrapper(links_wrapper).await;

    if links_result.is_err(){
        return HttpResponse::BadRequest().body(links_result.unwrap_err().to_string());
    }

    let json_result = serde_json::to_string_pretty(&links_result.unwrap());

    if json_result.is_err(){
        return HttpResponse::BadRequest().body(json_result.unwrap_err().to_string());
    }

    HttpResponse::Ok().body(json_result.unwrap())
}

#[get("/api/v1/list_links/{query:.*}")]
pub(crate) async fn list_links_where(web::Path(query): web::Path<String>) -> impl Responder {

    let links_wrapper = RB.new_wrapper()
        .like("lnk_target", &*query)
        .or()
        .like("lnk_path", &*query)
        .order_by(true, &["lnk_path"]);

    let links_result: Result<Vec<Links>, _> = RB.fetch_list_by_wrapper(links_wrapper).await;

    if links_result.is_err(){
        return HttpResponse::BadRequest().body(links_result.unwrap_err().to_string());
    }

    let json_result = serde_json::to_string_pretty(&links_result.unwrap());

    if json_result.is_err(){
        return HttpResponse::BadRequest().body(json_result.unwrap_err().to_string());
    }

    HttpResponse::Ok().body(json_result.unwrap())
}

//#endregion
