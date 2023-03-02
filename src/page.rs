use crate::section::Section;
use crate::utils::{Width, Height, Locality};
use crate::style::Styles;


pub enum PageType {
    Template(u32),
    Running 
}

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
type PageMargins = (Locality, Locality, Locality, Locality);

pub struct Page {
    page_size: PageSizes,
    page_type: PageType,
    margins: PageMargins,
    sections: Vec<Section>,
    styles: Styles
}


