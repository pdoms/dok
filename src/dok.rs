use std::{fs::{File, self}, io::BufReader};
use serde::{Deserializer, Deserialize};
use serde_json;


use crate::page::Page;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Dok {
    pub title: String,
    pub pages: Vec<Page>,
}



impl Dok {
    fn new_from_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?; 
        let reader = BufReader::new(file);
        let dsrlyzd: Self = serde_json::from_reader(reader)?;
        Ok(dsrlyzd)
    }
}



#[cfg(test)]
mod tests {

    use super::*;
    use crate::node::Node;
    use crate::blok::{BlokType, Blok};
    use crate::page::{PageType, PageSizes};
    use crate::section::{Section, SectionId};
    use crate::style::{Styles, Style};
    use crate::metadata::Metadata;

    #[test]
    fn json_loader() {
        let src = "assets/basic.json";
        let dok = Dok::new_from_json(src);
        let mut blok_style = Styles::with_capacity(1);
        blok_style.insert("align".to_owned(), Style::Align("left".to_string()));
        let mut node_style = Styles::with_capacity(1);
        node_style.insert("fontSize".to_owned(), Style::FontSize(12.0));
        let expected: Dok = Dok {
            title: "base".to_string().to_string(),
            pages: vec![
                Page {
                    page_type: PageType::Running,
                    size: PageSizes::A4,
                    number: Some(1),
                    margins: [72.0, 72.0, 54.0, 72.0],
                    style: Styles::new(),
                    sections: vec![
                        {
                            Section {
                                id: SectionId::Main,
                                root_x: 72.0,
                                root_y: 72.0,
                                width: 452.0,
                                height: 698.0,
                                style: Styles::new(),
                                content: vec![
                                    {
                                        Blok {
                                            id: "par_1".to_string(),
                                            blok_type: BlokType::Paragraph,
                                            style: blok_style,
                                            content: vec![
                                                Node {
                                                    id: "par_1_node_1".to_string(),
                                                    text: "Hello World".to_string(),
                                                    style: node_style,
                                                    metadata: Metadata::new()
                                                }
                                            ],
                                            metadata: Metadata::new()
                                            
                                        }
                                    }
                                ]
                            }
                        }
                    ]
                }
            ]
        }; 
        assert_eq!(dok.unwrap(), expected);
    }
}

