use serde::{Deserialize};

#[derive(Deserialize)]
pub struct OpenGraph {
    pub id: i32,
    pub tag: String,
    pub content: String,
}

