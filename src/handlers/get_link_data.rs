use actix_web::{get, web, App, HttpServer, Responder};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use rbatis::crud::CRUD;
use links::Links;
use crate::database::links;

#[get("/get_link_data/{id}")]
pub(crate) async fn get_link_data(web::Path((id)): web::Path<(u32)>) -> impl Responder {
    let option : Option<Links> = crate::database::RB.fetch_by_column("lnk_id", &id).await.unwrap();
    let link = option.unwrap();
    let opengraph : Vec<crate::database::open_graph::OpenGraph> = crate::database::RB.fetch_list_by_column("log_link_id",&[id]).await.unwrap();

    let result: LinkWithOpengraph = LinkWithOpengraph{
        id: link.lnk_id,
        path: link.lnk_path,
        target: link.lnk_target,
        hide_target: link.lnk_hide_target,
        opengraph_tags: opengraph,
    };


    serde_json::to_string_pretty(&result).unwrap()
}

struct LinkWithOpengraph {
    pub id: i32,
    pub path: String,
    pub target: String,
    pub hide_target: i32,
    pub opengraph_tags: Vec<crate::database::open_graph::OpenGraph>,
}

impl Serialize for LinkWithOpengraph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("LinkWithOpengraph", 5)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("path", &self.path)?;
        s.serialize_field("target", &self.target)?;
        s.serialize_field("hide_target", &self.hide_target)?;
        s.serialize_field("opengraph_tags", &self.opengraph_tags)?;
        s.end()
    }
}