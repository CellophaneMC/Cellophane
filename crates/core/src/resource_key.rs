use std::borrow::{Borrow, Cow};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
pub struct ResourceKey<S> {
    string: S,
}

impl<'a> ResourceKey<Cow<'a, str>> {
    pub fn new(string: impl Into<Cow<'a, str>>) -> Result<Self, Error> {
        parse(string.into())
    }
}

impl<S> ResourceKey<S> {
    pub fn as_str(&self) -> &str
        where
            S: AsRef<str>,
    {
        self.string.as_ref()
    }

    pub fn as_str_resource_key(&self) -> ResourceKey<&str>
        where
            S: AsRef<str>,
    {
        ResourceKey {
            string: self.as_str(),
        }
    }

    pub fn to_string_resource_key(&self) -> ResourceKey<String>
        where
            S: AsRef<str>,
    {
        ResourceKey {
            string: self.as_str().to_owned(),
        }
    }

    pub fn namespace(&self) -> &str
        where
            S: AsRef<str>,
    {
        self.namespace_and_value().0
    }

    pub fn value(&self) -> &str
        where
            S: AsRef<str>,
    {
        self.namespace_and_value().1
    }

    pub fn into_inner(self) -> S {
        self.string
    }

    fn namespace_and_value(&self) -> (&str, &str)
        where
            S: AsRef<str>,
    {
        self.as_str()
            .split_once(':')
            .expect("invalid resource identifier")
    }
}

impl<S: AsRef<str>> AsRef<str> for ResourceKey<S> {
    fn as_ref(&self) -> &str {
        self.string.as_ref()
    }
}

impl<S> AsRef<S> for ResourceKey<S> {
    fn as_ref(&self) -> &S {
        &self.string
    }
}

impl<S: Borrow<str>> Borrow<str> for ResourceKey<S> {
    fn borrow(&self) -> &str {
        self.string.borrow()
    }
}

impl From<ResourceKey<&str>> for String {
    fn from(value: ResourceKey<&str>) -> Self {
        value.as_str().to_owned()
    }
}

impl From<ResourceKey<String>> for String {
    fn from(value: ResourceKey<String>) -> Self {
        value.into_inner()
    }
}

impl<'a> From<ResourceKey<Cow<'a, str>>> for Cow<'a, str> {
    fn from(value: ResourceKey<Cow<'a, str>>) -> Self {
        value.into_inner()
    }
}

impl<'a> From<ResourceKey<Cow<'a, str>>> for ResourceKey<String> {
    fn from(value: ResourceKey<Cow<'a, str>>) -> Self {
        Self {
            string: value.string.into(),
        }
    }
}

impl<'a> From<ResourceKey<String>> for ResourceKey<Cow<'a, str>> {
    fn from(value: ResourceKey<String>) -> Self {
        Self {
            string: value.string.into(),
        }
    }
}

impl<'a> From<ResourceKey<&'a str>> for ResourceKey<Cow<'a, str>> {
    fn from(value: ResourceKey<&'a str>) -> Self {
        ResourceKey {
            string: value.string.into(),
        }
    }
}

impl<'a> From<ResourceKey<&'a str>> for ResourceKey<String> {
    fn from(value: ResourceKey<&'a str>) -> Self {
        ResourceKey {
            string: value.string.into(),
        }
    }
}

impl FromStr for ResourceKey<String> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ResourceKey::new(s)?.into())
    }
}

impl FromStr for ResourceKey<Cow<'static, str>> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ResourceKey::<String>::try_from(s).map(From::from)
    }
}

impl<'a> TryFrom<&'a str> for ResourceKey<String> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(ResourceKey::new(value)?.into())
    }
}

impl TryFrom<String> for ResourceKey<String> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(ResourceKey::new(value)?.into())
    }
}

impl<'a> TryFrom<Cow<'a, str>> for ResourceKey<String> {
    type Error = Error;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(ResourceKey::new(value)?.into())
    }
}

impl<'a> TryFrom<&'a str> for ResourceKey<Cow<'a, str>> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'a> TryFrom<String> for ResourceKey<Cow<'a, str>> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<'a> TryFrom<Cow<'a, str>> for ResourceKey<Cow<'a, str>> {
    type Error = Error;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<S: fmt::Debug> fmt::Debug for ResourceKey<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.string.fmt(f)
    }
}

impl<S: fmt::Display> fmt::Display for ResourceKey<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.string.fmt(f)
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Error)]
#[error("invalid resource identifier \"{0}\"")]
pub struct Error(pub String);

fn parse(string: Cow<str>) -> Result<ResourceKey<Cow<str>>, Error> {
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
            Ok(ResourceKey { string })
        }
        None if check_path(&string) => Ok(ResourceKey {
            string: format!("minecraft:{string}").into(),
        }),
        _ => Err(Error(string.into())),
    }
}
