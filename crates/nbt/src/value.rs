use std::borrow::Cow;
use std::hash::Hash;

use crate::error::Error;
use crate::{Compound, List, Tag};

#[derive(Debug, Clone)]
pub enum Value<S = String> {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(S),
    List(List<S>),
    Compound(Compound<S>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

macro_rules! impl_value {
    ($name:ident, $($lifetime:lifetime)?, ($($deref:tt)*), $($reference:tt)*) => {
        macro_rules! as_number {
            ($method_name:ident, $ty:ty, $($deref)*) => {
                #[doc = concat!("If this value is a number, returns the `", stringify!($ty), "` representation of this value.")]
                pub fn $method_name(&self) -> Option<$ty> {
                    #[allow(trivial_numeric_casts)]
                    match self {
                        Self::Byte(v) => Some($($deref)* v as $ty),
                        Self::Short(v) => Some($($deref)* v as $ty),
                        Self::Int(v) => Some($($deref)* v as $ty),
                        Self::Long(v) => Some($($deref)* v as $ty),
                        Self::Float(v) => Some(v.floor() as $ty),
                        Self::Double(v) => Some(v.floor() as $ty),
                        _ => None,
                    }
                }
            }
        }

        macro_rules! as_number_float {
            ($method_name:ident, $ty:ty, $($deref)*) => {
                #[doc = concat!("If this value is a number, returns the `", stringify!($ty), "` representation of this value.")]
                pub fn $method_name(&self) -> Option<$ty> {
                    #[allow(trivial_numeric_casts)]
                    match self {
                        Self::Byte(v) => Some($($deref)* v as $ty),
                        Self::Short(v) => Some($($deref)* v as $ty),
                        Self::Int(v) => Some($($deref)* v as $ty),
                        Self::Long(v) => Some($($deref)* v as $ty),
                        Self::Float(v) => Some($($deref)* v as $ty),
                        Self::Double(v) => Some($($deref)* v as $ty),
                        _ => None,
                    }
                }
            }
        }

        impl <$($lifetime,)? S> $name<$($lifetime,)? S> {
            /// Returns the type of this value.
            pub fn tag(&self) -> Tag {
                match self {
                    Self::Byte(_) => Tag::Byte,
                    Self::Short(_) => Tag::Short,
                    Self::Int(_) => Tag::Int,
                    Self::Long(_) => Tag::Long,
                    Self::Float(_) => Tag::Float,
                    Self::Double(_) => Tag::Double,
                    Self::ByteArray(_) => Tag::ByteArray,
                    Self::String(_) => Tag::String,
                    Self::List(_) => Tag::List,
                    Self::Compound(_) => Tag::Compound,
                    Self::IntArray(_) => Tag::IntArray,
                    Self::LongArray(_) => Tag::LongArray,
                }
            }

            /// Returns whether this value is a number, i.e. a byte, short, int, long, float or double.
            pub fn is_number(&self) -> bool {
                match self {
                    Self::Byte(_) | Self::Short(_) | Self::Int(_) | Self::Long(_) | Self::Float(_) | Self::Double(_) => true,
                    _ => false,
                }
            }

            as_number!(as_i8, i8, $($deref)*);
            as_number!(as_i16, i16, $($deref)*);
            as_number!(as_i32, i32, $($deref)*);
            as_number!(as_i64, i64, $($deref)*);
            as_number_float!(as_f32, f32, $($deref)*);
            as_number_float!(as_f64, f64, $($deref)*);

            /// If this value is a number, returns the `bool` representation of this value.
            pub fn as_bool(&self) -> Option<bool> {
                self.as_i8().map(|v| v != 0)
            }
        }

        impl <$($lifetime,)? S> From<$($reference)* i8> for $name<$($lifetime,)? S> {
            fn from(v: $($reference)* i8) -> Self {
                Self::Byte(v)
            }
        }

        impl <$($lifetime,)? S> From<$($reference)* i16> for $name<$($lifetime,)? S> {
            fn from(v: $($reference)* i16) -> Self {
                Self::Short(v)
            }
        }

        impl <$($lifetime,)? S> From<$($reference)* i32> for $name<$($lifetime,)? S> {
            fn from(v: $($reference)* i32) -> Self {
                Self::Int(v)
            }
        }

        impl <$($lifetime,)? S> From<$($reference)* i64> for $name<$($lifetime,)? S> {
            fn from(v: $($reference)* i64) -> Self {
                Self::Long(v)
            }
        }

        impl <$($lifetime,)? S> From<$($reference)* f32> for $name<$($lifetime,)? S> {
            fn from(v: $($reference)* f32) -> Self {
                Self::Float(v)
            }
        }

        impl <$($lifetime,)? S> From<$($reference)* f64> for $name<$($lifetime,)? S> {
            fn from(v: $($reference)* f64) -> Self {
                Self::Double(v)
            }
        }

        impl <$($lifetime,)? S> From<$($reference)* List<S>> for $name<$($lifetime,)? S> {
            fn from(v: $($reference)* List<S>) -> Self {
                Self::List(v)
            }
        }

        impl <$($lifetime,)? S> From<$($reference)* Compound<S>> for $name<$($lifetime,)? S> {
            fn from(v: $($reference)* Compound<S>) -> Self {
                Self::Compound(v)
            }
        }

        impl <$($lifetime,)? S> PartialEq<Self> for $name<$($lifetime,)? S> where S: Ord + Hash {
            fn eq(&self, other: &Self) -> bool {
                match self {
                    Self::Byte(v) => matches!(other, Self::Byte(other_v) if v == other_v),
                    Self::Short(v) => matches!(other, Self::Short(other_v) if v == other_v),
                    Self::Int(v) => matches!(other, Self::Int(other_v) if v == other_v),
                    Self::Long(v) => matches!(other, Self::Long(other_v) if v == other_v),
                    Self::Float(v) => matches!(other, Self::Float(other_v) if v == other_v),
                    Self::Double(v) => matches!(other, Self::Double(other_v) if v == other_v),
                    Self::ByteArray(v) => matches!(other, Self::ByteArray(other_v) if v == other_v),
                    Self::String(v) => matches!(other, Self::String(other_v) if v == other_v),
                    Self::List(v) => matches!(other, Self::List(other_v) if v == other_v),
                    Self::Compound(v) => matches!(other, Self::Compound(other_v) if v == other_v),
                    Self::IntArray(v) => matches!(other, Self::IntArray(other_v) if v == other_v),
                    Self::LongArray(v) => matches!(other, Self::LongArray(other_v) if v == other_v),
                }
            }
        }
    }
}

impl_value!(Value,,(*),);

impl From<String> for Value<String> {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl<'a> From<&'a str> for Value<String> {
    fn from(s: &'a str) -> Self {
        Self::String(s.to_owned())
    }
}

impl<'a> From<Cow<'a, str>> for Value<String> {
    fn from(s: Cow<'a, str>) -> Self {
        Self::String(s.into_owned())
    }
}

impl From<String> for Value<Cow<'_, str>> {
    fn from(v: String) -> Self {
        Self::String(Cow::Owned(v))
    }
}

impl<'a> From<&'a String> for Value<Cow<'a, str>> {
    fn from(v: &'a String) -> Self {
        Self::String(Cow::Borrowed(v))
    }
}

impl<'a> From<&'a str> for Value<Cow<'a, str>> {
    fn from(v: &'a str) -> Self {
        Self::String(Cow::Borrowed(v))
    }
}

impl<'a> From<Cow<'a, str>> for Value<Cow<'a, str>> {
    fn from(v: Cow<'a, str>) -> Self {
        Self::String(v)
    }
}

impl<S> From<Vec<i8>> for Value<S> {
    fn from(v: Vec<i8>) -> Self {
        Self::ByteArray(v)
    }
}

impl<S> From<Vec<u8>> for Value<S> {
    fn from(v: Vec<u8>) -> Self {
        unsafe { Self::ByteArray(std::mem::transmute(v)) }
    }
}

impl<S> From<Vec<i32>> for Value<S> {
    fn from(v: Vec<i32>) -> Self {
        Self::IntArray(v)
    }
}

impl<S> From<Vec<i64>> for Value<S> {
    fn from(v: Vec<i64>) -> Self {
        Self::LongArray(v)
    }
}

#[cfg(feature = "uuid")]
impl<S> From<uuid::Uuid> for Value<S> {
    fn from(value: uuid::Uuid) -> Self {
        let (most, least) = value.as_u64_pair();

        let first = (most >> 32) as i32;
        let second = most as i32;
        let third = (least >> 32) as i32;
        let fourth = least as i32;

        Value::IntArray(vec![first, second, third, fourth])
    }
}

#[cfg(feature = "serde")]
pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: serde::Serialize,
{
    todo!()
}
