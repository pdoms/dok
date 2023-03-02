

pub type Width = f32;
pub type Height = f32;
pub type Coord = f32;


pub(crate) enum Locality {
    Top(Coord),
    Right(Coord),
    Bottom(Coord),
    Left(Coord)
}
