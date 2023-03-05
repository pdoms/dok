use std::{fs::File, io::BufReader};
use serde::Deserialize;
use serde_json;


use crate::page::Page;

#[derive(Deserialize, Debug)]
pub struct Dok {
    pages: Vec<Page>,
}


impl Dok {
    fn new_from_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?; 
        let reader = BufReader::new(file);
        let dsrlyzd = serde_json::from_reader(reader)?;
        Ok(dsrlyzd)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_loader() {
        let src = "assets/basic.json";
        let dok = Dok::new_from_json(src);
        println!("{:?}", dok);
    }
}

