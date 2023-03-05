use serde::Deserialize;

pub type Width = f32;
pub type Height = f32;
pub type Coord = f32;

#[derive(Deserialize, Debug)]
pub(crate) enum Locality {
    Top(Coord),
    Right(Coord),
    Bottom(Coord),
    Left(Coord)
}
