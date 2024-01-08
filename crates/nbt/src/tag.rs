use std::fmt::Display;

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tag {
    End = 0,
    Byte = 1,
    Short = 2,
    Int = 3,
    Long = 4,
    Float = 5,
    Double = 6,
    ByteArray = 7,
    String = 8,
    List = 9,
    Compound = 10,
    IntArray = 11,
    LongArray = 12,
}

impl TryFrom<u8> for Tag {
    type Error = Error;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::End),
            1 => Ok(Self::Byte),
            2 => Ok(Self::Short),
            3 => Ok(Self::Int),
            4 => Ok(Self::Long),
            5 => Ok(Self::Float),
            6 => Ok(Self::Double),
            7 => Ok(Self::ByteArray),
            8 => Ok(Self::String),
            9 => Ok(Self::List),
            10 => Ok(Self::Compound),
            11 => Ok(Self::IntArray),
            12 => Ok(Self::LongArray),
            _ => Err(Error::InvalidTypeId(v)),
        }
    }
}

impl From<Tag> for u8 {
    fn from(v: Tag) -> Self {
        match v {
            Tag::End => 0,
            Tag::Byte => 1,
            Tag::Short => 2,
            Tag::Int => 3,
            Tag::Long => 4,
            Tag::Float => 5,
            Tag::Double => 6,
            Tag::ByteArray => 7,
            Tag::String => 8,
            Tag::List => 9,
            Tag::Compound => 10,
            Tag::IntArray => 11,
            Tag::LongArray => 12,
        }
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::End => write!(f, "TAG_End"),
            Tag::Byte => write!(f, "TAG_Byte"),
            Tag::Short => write!(f, "TAG_Short"),
            Tag::Int => write!(f, "TAG_Int"),
            Tag::Long => write!(f, "TAG_Long"),
            Tag::Float => write!(f, "TAG_Float"),
            Tag::Double => write!(f, "TAG_Double"),
            Tag::ByteArray => write!(f, "TAG_ByteArray"),
            Tag::String => write!(f, "TAG_String"),
            Tag::List => write!(f, "TAG_List"),
            Tag::Compound => write!(f, "TAG_Compound"),
            Tag::IntArray => write!(f, "TAG_IntArray"),
            Tag::LongArray => write!(f, "TAG_LongArray"),
        }
    }
}
