use crate::utils::{Locality, Width, Height};
use crate::blok::Blok;



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

pub struct Section {
    id: SectionId,
    root_x: Locality,
    root_y: Locality,
    width: Width, 
    height: Height, 
    content: Vec<Blok>
}


