use std::fmt;
#[derive(Debug)]
pub enum DokError {
    NoOp(String),
    UnknownBlockType(String),
    ErrGenId,
}

impl fmt::Display for DokError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DokError::NoOp(v) => f.write_fmt(format_args!("DOK-ERROR: {} is a no-op.", v)),
            DokError::UnknownBlockType(v) => {
                f.write_fmt(format_args!("DOK-ERROR: unknown BlokType '{}'", v))
            }
            DokError::ErrGenId => f.write_str("ERROR: generating random id"),
        }
    }
}

impl std::error::Error for DokError {}
