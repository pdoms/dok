use serde_derive::{Serialize, Deserialize};
use std::fmt::{self, Debug, Display};

use crate::error::DokError;

/// Represents measurement units
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum Unit {
    Pt(f64),
    Px(f64),
    Mm(f64),
}

impl Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Unit::Pt(v) => f.write_fmt(format_args!("{}pt", v)),
            Unit::Px(v) => f.write_fmt(format_args!("{}px", v)),
            Unit::Mm(v) => f.write_fmt(format_args!("{}mm", v)),
        }
    }
}

/// Represents Style
#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum NodeStyle {
    Bold(bool),
    Underline(bool),
    Italic(bool),
    Align(Alignment),
    FontType(String),
    FontSize(Unit),
    Background(String),
    Color(String),
}

/// Variants of Alignment
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum Alignment {
    Left,
    Right,
    Center,
    Justify,
}

impl Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Alignment::Left => f.write_str("left"),
            Alignment::Right => f.write_str("right"),
            Alignment::Center => f.write_str("center"),
            Alignment::Justify => f.write_str("justify"),
        }
    }
}

impl Debug for NodeStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeStyle::Bold(v) => f.write_fmt(format_args!("bold: {}", v)),
            NodeStyle::Underline(v) => f.write_fmt(format_args!("underline: {}", v)),
            NodeStyle::Italic(v) => f.write_fmt(format_args!("italic: {}", v)),
            NodeStyle::Align(v) => f.write_fmt(format_args!("align: {}", v)),
            NodeStyle::FontType(v) => f.write_fmt(format_args!("fontType: {}", v)),
            NodeStyle::FontSize(v) => f.write_fmt(format_args!("fontSize: {}", v)),
            NodeStyle::Background(v) => f.write_fmt(format_args!("background: {}", v)),
            NodeStyle::Color(v) => f.write_fmt(format_args!("color: {}", v)),
        }
    }
}

impl NodeStyle {
    pub fn is_bool_variant(&self) -> bool {
        match self {
            NodeStyle::Bold(_) => true,
            NodeStyle::Italic(_) => true,
            NodeStyle::Underline(_) => true,
            _ => false,
        }
    }
    pub fn not(&self) -> Result<NodeStyle, DokError> {
        match self {
            NodeStyle::Bold(v) => Ok(NodeStyle::Bold(!v)),
            NodeStyle::Italic(v) => Ok(NodeStyle::Italic(!v)),
            NodeStyle::Underline(v) => Ok(NodeStyle::Underline(!v)),
            _ => Err(DokError::NoOp("'Not-ing' a non-boolean value".to_string())),
        }
    }
}
