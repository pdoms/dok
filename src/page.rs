use crate::section::Section;
use crate::utils::{Width, Height};
use serde::Deserialize;
use crate::style::{Styles, de_style};


#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum PageType {
    Template(u32),
    Running 
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Page {
    pub page_type: PageType,
    pub size: PageSizes,
    pub number: Option<u32>,
    pub margins: PageMargins,
    pub sections: Vec<Section>,
    #[serde(deserialize_with = "de_style")]
    pub style: Styles
}


