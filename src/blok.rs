use serde::Deserialize;
use crate::node::Node; 
use crate::style::{de_style, Styles};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum BlokType {
    Paragraph
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Blok {
    pub blok_type: BlokType,
    pub id: String,
    pub content: Vec<Node>,
    #[serde(deserialize_with = "de_style")]
    pub style: Styles
}
