use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum BlokType {
    Paragraph
}

#[derive(Deserialize, Debug)]
pub struct Blok<'b> {
    blok_type: BlokType,
    id: &'b str,
}
