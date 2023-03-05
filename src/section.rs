use crate::style::{de_style, Styles};
use crate::utils::{Width, Height, Locality, Coord};
use crate::blok::Blok;
use serde::Deserialize;


#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum SectionId {
    Head,
    Right,
    Left,
    Foot,
    Main 
}

impl SectionId {
    fn root(&self) -> (Locality, Locality) {
        use crate::utils::Locality::*;
        match self {
            Self::Head  => (Top(0.0),     Left(0.0)),
            Self::Right => (Top(0.0),     Right(0.0)),
            Self::Left  => (Top(0.0),     Left(0.0)),
            Self::Foot  => (Bottom(0.0),  Left(0.0)),
            Self::Main  => (Top(0.0),     Left(0.0)),
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Section {
    pub id: SectionId,
    pub root_x: Coord,
    pub root_y: Coord,
    pub width: Width, 
    pub height: Height, 
    pub content: Vec<Blok>,
    #[serde(deserialize_with="de_style")]
    pub style: Styles
}


