use crate::section::Section;
use crate::utils::{Width, Height, Locality};
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub enum PageType {
    Template(u32),
    Running 
}

#[derive(Deserialize, Debug)]
pub enum PageSizes {
    A4
}

impl PageSizes {
    fn dimensions(&self) -> (Height, Width) {
        match *self {
            PageSizes::A4 => (842.0, 596.0)
        }
    }
}


//TRBL
type PageMargins = [f32; 4];
#[derive(Deserialize, Debug)]
pub struct Page {
    page_type: PageType,
    size: PageSizes,
    number: Option<u32>,
    margins: PageMargins,
    sections: Vec<Section>,
}


