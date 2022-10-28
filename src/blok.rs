use crate::{BlokType, MetaData, MetaDataItem};
use crate::node::Node;
use std::collections::HashMap;
use serde_derive::{Serialize, Deserialize};


/// A struct that represents the data structure of a blok, which holds a [`Vec`]
/// of [`Node`]s. It thus represents unit of content, which can be any of the [`BlokType`]s.
/// e.g., a Paragraph, or a table cell.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blok {
    id: String,
    blok_type: BlokType,
    nodes: Vec<Node>,
    meta_data: Option<MetaData>,
    nodes_len: usize
}

impl Blok {
    /// Given an id or index and a [`BlokType`] a new Blok is created.
    pub fn new<S: Into<String>>(i: S, blok_type: BlokType) -> Blok {
        let id = format!("{}_{}", i.into(), blok_type.abbreviate());
        Blok {
            id,
            blok_type,
            nodes: Vec::new(),
            meta_data: None,
            nodes_len: 0,
        }
    } 
    /// Returns reference to value
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Returns blok type
    pub fn blok_type(&self) -> BlokType {
        self.blok_type.clone()
    }

    ///Returns a clone of the [`MetaData`] HashMap wrapped in an Option
    pub fn meta_data(&self) -> Option<MetaData> {
        self.meta_data.clone()
    }
    
    ///Adds meta_data item, if entry already exists, it will be overridden.
    pub fn add_meta_data(&mut self, entry: MetaDataItem) {
        if self.meta_data.is_some() {
            let mut m = self.meta_data.clone().unwrap();
            m.insert(entry.0, entry.1);
            self.meta_data = Some(m);
        } else {
            let mut map = HashMap::new();
            map.insert(entry.0, entry.1);
            self.meta_data = Some(map);
        }
    }

    /// Returns length of [`Vec`] of [`Node`]s 
    pub fn nodes_len(&self) -> usize {
        self.nodes_len
    }
    /// Returns the [`Vec`] of [`Node`]s 
    pub fn nodes(&self) -> Vec<Node> {
        self.nodes.clone()
    }

    /// Adds a new [`Node`] to end of self.nodes  
    pub fn push_node(&mut self, n: Node) {
        self.nodes.push(n)
    } 
    /// Sets or replaces [`Vec`] of [`Node`]s
    pub fn set_nodes(&mut self, nodes: Vec<Node>) {
        self.nodes = nodes
    }
    /// Appends a [`Vec`] of [`Node`]s to end of self.nodes  
    pub fn append_nodes(&mut self, mut nodes: Vec<Node>) {
        self.nodes.append(&mut nodes)
    }
    /// Searches for a node by a given id. Retruns an [`Option`]
    pub fn get_node_by_id<I>(&self, id: I) -> Option<&Node> 
    where
        I: Into<String> + Copy
    {
        self.nodes.iter().find(|n| n.id() == id.into())
    }

    /// Searches for a node by accepting a closure that returns `true` or `false`. Retruns an [`Option`] of a [`None`] clone or [`None`]
    pub fn get_node<P>(&self, predicate: P) -> Option<Node> 
    where 
        P: Fn(&Node) -> bool, 
    {
        for node in self.nodes.clone() {
            if predicate(&node) {
                return Some(node)
            }
        }
        None
    }
    /// Removes a [`Node`] by id.
    pub fn remove_node_by_id<I>(&mut self, id: I) 
    where
        I: Into<String> + Copy
    {
        self.nodes.retain(|n| n.id() != id.into())
    }


    /// Returns the text of all nodes in one [`String`]. 
    pub fn plain_text(&self) -> String {
        self.nodes.iter().map(|n| n.text()).collect::<String>()
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blok_new() {
        let bt = BlokType::Paragraph;
        let b = Blok::new(0.to_string(), bt.clone());
        assert_eq!(b.nodes_len(), 0);
        assert_eq!(b.id, "0_par");
        assert_eq!(b.blok_type(), bt);
    }



    #[test]
    fn blok_methods() {
        let bt = BlokType::Paragraph;
        let mut b = Blok::new(0.to_string(), bt);
        b.add_meta_data(MetaDataItem("width".to_string(), "12.0px".to_string()));
        let mut md = HashMap::new();
        md.insert("width".to_string(), "12.0px".to_string());
        assert_eq!(b.meta_data(), Some(md));
    }


}
