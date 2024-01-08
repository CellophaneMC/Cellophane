use std::fmt;

use crate::error::Error;

pub mod de;
pub mod ser;

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::Serde(format!("{msg}"))
    }
}
