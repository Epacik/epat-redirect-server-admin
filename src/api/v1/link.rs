use serde::{Deserialize, Serialize, Serializer, ser::SerializeStruct};

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