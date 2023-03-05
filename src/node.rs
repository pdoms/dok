use serde::Deserialize;

use crate::style::{de_style, Styles};
 


#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Node {
    pub id: String,
    pub text: String,
    #[serde(deserialize_with = "de_style")]
    pub style: Styles
}
