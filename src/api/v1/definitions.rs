use serde::{Serialize, Serializer, Deserialize};
use serde::ser::SerializeStruct;

#[derive(Deserialize)]
pub(crate) struct Link {
    pub path: String,
    pub target: String,
    pub hide_target: i32,
}

impl Serialize for Link {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("LinkWithOpengraph", 3)?;
        s.serialize_field("path", &self.path)?;
        s.serialize_field("target", &self.target)?;
        s.serialize_field("hide_target", &self.hide_target)?;
        s.end()
    }
}

#[derive(Deserialize)]
pub(crate) struct LinkWithOpengraph {
    pub id: i32,
    pub path: String,
    pub target: String,
    pub hide_target: i32,
    pub opengraph_tags: Vec<crate::database::OpenGraph>,
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

#[derive(Deserialize)]
pub struct OpenGraph {
    pub id: i32,
    pub tag: String,
    pub content: String,
}

