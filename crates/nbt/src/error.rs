use std::fmt::Display;

use thiserror::Error;

use crate::Tag;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("NBT format error: {0}")]
    Serde(String),
    #[error("Invalid NBT tag byte: '{0}'")]
    InvalidTypeId(u8),
    #[cfg(feature = "binary")]
    #[error("Invalid UTF-8 string: {0}")]
    InvalidUtf8(#[from] cesu8::Cesu8DecodingError),
    #[error("Encountered type '{0}', which has no corresponding NBT tag")]
    UnrepresentableType(&'static str),
    #[error("Invalid array length: {0}")]
    InvalidArrayLength(i32),
    #[error("Invalid TAG_End list length: {0}")]
    TagEndListWithNonZeroLength(i32),
    #[error("Key must be a string")]
    KeyMustBeAString,
    #[error("Float key must be finite (got NaN or +/-inf)")]
    FloatKeyMustBeFinite,
    #[error("End tag found in value")]
    EndTagInValue,
    #[error("Root tag is not a TAG_Compound (got {0:?})")]
    RootTagNotCompound(Tag),
    #[error("Reached maximum recursion depth")]
    RecursionLimitExceeded,
}

#[cfg(feature = "serde")]
impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Error::Serde(msg.to_string())
    }
}
