use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

use crate::nodestyle::NodeStyle;
use crate::rand_id::gen_id;
use crate::{MetaData, MetaDataItem};

/// The Node struct is the building block of a blok. I.e., it holds
/// styles corresponding to a part of a text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    id: String,
    text: String,
    styles: Vec<NodeStyle>,
    meta_data: Option<MetaData>,
    // index set when added to `Blok`
    #[serde(skip_serializing)]
    internal_idx: Option<usize>,
}

/// Returns an empty node. I.e. creates a random `id`, uses an empty [`String`] 
/// for text, an empty [`Vec`] for `styles` and [`None`] for `meta_data` and `internal_idx`
impl Default for Node {
    fn default() -> Self {
        let id = match gen_id() {
            Ok(v) => v,
            Err(_) => String::new(),
        };
        Node {
            id,
            text: String::new(),
            styles: Vec::new(),
            meta_data: None,
            internal_idx: None,
        }
    }
}

impl Node {
    ///Creates a new [`Node`] with provided data. Styles need to be known at creation.
    pub fn new_all<I: Into<String>, T: Into<String>>(
        id: I,
        text: T,
        styles: Vec<NodeStyle>,
        internal_idx: Option<usize>,
    ) -> Node {
        Node {
            id: id.into(),
            text: text.into(),
            meta_data: None,
            styles,
            internal_idx,
        }
    }
    ///Creates a new [`Node`] with the provided text and sequence/id. Styles do not need to be
    ///provided and can be added later on with [`Node.add_style`], [`Node.add_styles`] or [`Node.set_style`].
    pub fn new<I: Into<String>, T: Into<String>>(id: I, text: T) -> Node {
        Node {
            id: id.into(),
            text: text.into(),
            styles: Vec::new(),
            meta_data: None,
            internal_idx: None,
        }
    }

    ///Adds one [`NodeStyle`] to node.
    pub fn add_style(&mut self, s: NodeStyle) {
        self.styles.push(s)
    }
    ///Appends a [`NodeStyle`] [`Vec`] to a node.
    pub fn append_styles(&mut self, mut styles: Vec<NodeStyle>) {
        self.styles.append(&mut styles)
    }

    ///Set or replace [`Vec`] of [`NodeStyle`]
    pub fn set_style(&mut self, styles: Vec<NodeStyle>) {
        self.styles = styles;
    }
    
    ///Returns a reference to the value
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    ///Returns a reference to the value
    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn styles(&self) -> Vec<NodeStyle> {
        self.styles.clone()
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

    ///Sets or replaces text.
    pub fn set_text<S: Into<String>>(&mut self, s: S) {
        self.text = s.into()
    }

    ///Adds/appends to the text. (if provided text does not start with whitespace
    ///or the current text does not end with whitespace a whitespace is added between both items.)
    pub fn add_text(&mut self, t: &str) {
        if !self.text.is_empty()  && !self.text.ends_with(' ') && !t.starts_with(' ') {
            self.text.push(' ');
        }
        self.text.push_str(t);
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node_default() {
        let n = Node::default();
        assert_eq!(n.id().len(), 6);
        assert!(n.text().is_empty());
        assert!(n.styles().is_empty());   
        assert_eq!(n.meta_data(), None);
        assert_eq!(n.internal_idx, None);
    }

    #[test]
    fn new_node_new() {
        let n = Node::new("id".to_string(), "text");
        assert_eq!(n.id(), "id");
        assert_eq!(n.text(), "text");
        assert!(n.styles().is_empty());   
        assert_eq!(n.meta_data(), None);
        assert_eq!(n.internal_idx, None);
    }

    #[test]
    fn new_node_new_all() {
        let id = "id";
        let txt = "text";
        let i_idx = Some(0);
        let styles = vec![NodeStyle::Bold(true)];
        let n = Node::new_all(id, txt, styles.clone(), i_idx);
        assert_eq!(n.id(), "id");
        assert_eq!(n.text(), "text");
        assert_eq!(n.styles(), styles);   
        assert_eq!(n.meta_data(), None);
        assert_eq!(n.internal_idx.unwrap(), 0);
    }

    #[test]
    fn node_methods() {
        use crate::nodestyle::{NodeStyle, Alignment , Unit};
        let mut n = Node::default();
        let mut styles = vec![NodeStyle::FontSize(Unit::Pt(12.0)), NodeStyle::Background("#00000".to_string())];
        let style = NodeStyle::Align(Alignment::Left);
        n.append_styles(styles.clone());
        assert_eq!(n.styles(), styles.clone());
        n.add_style(style.clone());
        styles.push(style);
        assert_eq!(n.styles(), styles);
        n.set_style(Vec::new());
        assert!(n.styles().is_empty());
        n.add_meta_data(MetaDataItem("width".to_string(), "12.0px".to_string()));
        let mut md = HashMap::new();
        md.insert("width".to_string(), "12.0px".to_string());
        assert_eq!(n.meta_data(), Some(md));
        n.add_text("text");
        assert_eq!(n.text(), "text");
        n.add_text("again ");
        assert_eq!(n.text(), "text again ");
        n.add_text("and");
        assert_eq!(n.text(), "text again and");
        n.add_text(" again");
        assert_eq!(n.text(), "text again and again");
        n.set_text("");
        assert!(n.text().is_empty());
    }
}

