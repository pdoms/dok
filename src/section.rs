use crate::utils::{Width, Height, Locality, Coord};
use crate::blok::Blok;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct Section {
    id: SectionId,
    root_x: Coord,
    root_y: Coord,
    width: Width, 
    height: Height, 
    //content: Vec<Blok>
}


