//! A collection of data structures for semi-sequencial document data.
//!
pub mod blok;
mod error;
pub mod node;
pub mod nodestyle;
mod rand_id;

use serde_derive::{Serialize, Deserialize};

use crate::blok::Blok;
use crate::error::DokError;
use std::{collections::HashMap, fmt::Display};

/// A data structure that holds data for a whole document.
pub struct Dok {
    content: Vec<Blok>,
}

/// Blok Types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlokType {
    Paragraph,
}

impl BlokType {
    pub fn from_str(t: &str) -> Result<BlokType, DokError> {
        match t {
            "paragraph" => Ok(BlokType::Paragraph),
            _ => Err(DokError::UnknownBlockType(t.to_string())),
        }
    }

    pub fn abbreviate(&self) -> String {
        match self {
            BlokType::Paragraph => "par".to_string(),
        }
    }
}

impl Display for BlokType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlokType::Paragraph => f.write_str("paragraph"),
        }
    }
}

/// The MetaData type holds data describing the given blok or gibven node.
pub type MetaData = HashMap<String, String>;
/// Utility struct to define one metadata item, as key value pair.
pub struct MetaDataItem(String, String);
