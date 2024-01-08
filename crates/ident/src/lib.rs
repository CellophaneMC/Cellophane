use std::borrow::{Borrow, Cow};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use thiserror::Error;

pub use cellophanemc_ident_macros::parse_ident_str;

#[macro_export]
macro_rules! ident {
    ($string:literal) => {
        $crate::Ident::<&'static str> {
            string: $crate::parse_ident_str!($string),
        }
    };
}

#[derive(Clone, Eq, Ord, Hash)]
pub struct Ident<S> {
    string: S,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Error)]
#[error("invalid resource identifier \"{0}\"")]
pub struct IdentError(pub String);

impl<'a> Ident<Cow<'a, str>> {
    pub fn new(string: impl Into<Cow<'a, str>>) -> Result<Self, IdentError> {
        parse(string.into())
    }
}

fn parse(string: Cow<str>) -> Result<Ident<Cow<str>>, IdentError> {
    let check_namespace = |s: &str| {
        !s.is_empty()
            && s.chars()
                .all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-'))
    };

    let check_path = |s: &str| {
        !s.is_empty()
            && s.chars()
                .all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/'))
    };

    match string.split_once(':') {
        Some((namespace, path)) if check_namespace(namespace) && check_path(path) => {
            Ok(Ident { string })
        }
        None if check_path(&string) => Ok(Ident {
            string: format!("minecraft:{string}").into(),
        }),
        _ => Err(IdentError(string.into())),
    }
}

impl<S: AsRef<str>> AsRef<str> for Ident<S> {
    fn as_ref(&self) -> &str {
        self.string.as_ref()
    }
}

impl<S> AsRef<S> for Ident<S> {
    fn as_ref(&self) -> &S {
        &self.string
    }
}

impl<S: Borrow<str>> Borrow<str> for Ident<S> {
    fn borrow(&self) -> &str {
        self.string.borrow()
    }
}

impl From<Ident<&str>> for String {
    fn from(value: Ident<&str>) -> Self {
        value.string.to_string()
    }
}

impl From<Ident<String>> for String {
    fn from(value: Ident<String>) -> Self {
        value.into_inner()
    }
}

impl<'a> From<Ident<Cow<'a, str>>> for Cow<'a, str> {
    fn from(value: Ident<Cow<'a, str>>) -> Self {
        value.into_inner()
    }
}

impl<'a> From<Ident<Cow<'a, str>>> for Ident<String> {
    fn from(value: Ident<Cow<'a, str>>) -> Self {
        Self {
            string: value.string.into(),
        }
    }
}

impl<'a> From<Ident<String>> for Ident<Cow<'a, str>> {
    fn from(value: Ident<String>) -> Self {
        Self {
            string: value.string.into(),
        }
    }
}

impl<'a> From<Ident<&'a str>> for Ident<Cow<'a, str>> {
    fn from(value: Ident<&'a str>) -> Self {
        Ident {
            string: value.string.into(),
        }
    }
}

impl<'a> From<Ident<&'a str>> for Ident<String> {
    fn from(value: Ident<&'a str>) -> Self {
        Ident {
            string: value.string.into(),
        }
    }
}

impl FromStr for Ident<String> {
    type Err = IdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ident::new(s)?.into())
    }
}

impl FromStr for Ident<Cow<'static, str>> {
    type Err = IdentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ident::<String>::try_from(s).map(From::from)
    }
}

impl<'a> TryFrom<&'a str> for Ident<String> {
    type Error = IdentError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Ident::new(value)?.into())
    }
}

impl TryFrom<String> for Ident<String> {
    type Error = IdentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Ident::new(value)?.into())
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Ident<String> {
    type Error = IdentError;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(Ident::new(value)?.into())
    }
}

impl<'a> TryFrom<&'a str> for Ident<Cow<'a, str>> {
    type Error = IdentError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'a> TryFrom<String> for Ident<Cow<'a, str>> {
    type Error = IdentError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'a> TryFrom<Cow<'a, str>> for Ident<Cow<'a, str>> {
    type Error = IdentError;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<S: Debug> Debug for Ident<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.string.fmt(f)
    }
}

impl<S: fmt::Display> fmt::Display for Ident<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.string.fmt(f)
    }
}

impl<S, T> PartialEq<Ident<T>> for Ident<S>
where
    S: PartialEq<T>,
{
    fn eq(&self, other: &Ident<T>) -> bool {
        self.string == other.string
    }
}

impl<S, T> PartialOrd<Ident<T>> for Ident<S>
where
    S: PartialOrd<T>,
{
    fn partial_cmp(&self, other: &Ident<T>) -> Option<Ordering> {
        self.string.partial_cmp(&other.string)
    }
}
