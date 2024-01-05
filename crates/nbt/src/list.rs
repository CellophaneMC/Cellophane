use std::borrow::Cow;
use std::hash::Hash;

use crate::compound::Compound;
use crate::value::Value;

#[derive(Debug, Clone, Default)]
pub enum List<S = String> {
    #[default]
    End,
    Byte(Vec<i8>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    ByteArray(Vec<Vec<i8>>),
    String(Vec<S>),
    List(Vec<List<S>>),
    Compound(Vec<Compound<S>>),
    IntArray(Vec<Vec<i32>>),
    LongArray(Vec<Vec<i64>>),
}

impl<S> List<S> {
    pub fn new() -> Self {
        Self::End
    }

    pub fn len(&self) -> usize {
        match self {
            Self::End => 0,
            Self::Byte(v) => v.len(),
            Self::Short(v) => v.len(),
            Self::Int(v) => v.len(),
            Self::Long(v) => v.len(),
            Self::Float(v) => v.len(),
            Self::Double(v) => v.len(),
            Self::ByteArray(v) => v.len(),
            Self::String(v) => v.len(),
            Self::List(v) => v.len(),
            Self::Compound(v) => v.len(),
            Self::IntArray(v) => v.len(),
            Self::LongArray(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<S> PartialEq for List<S>
    where
        S: Ord + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            List::End => matches!(other, List::End),
            List::Byte(list) => matches!(other, List::Byte(other_list) if list == other_list),
            List::Short(list) => matches!(other, List::Short(other_list) if list == other_list),
            List::Int(list) => matches!(other, List::Int(other_list) if list == other_list),
            List::Long(list) => matches!(other, List::Long(other_list) if list == other_list),
            List::Float(list) => matches!(other, List::Float(other_list) if list == other_list),
            List::Double(list) => matches!(other, List::Double(other_list) if list == other_list),
            List::ByteArray(list) => {
                matches!(other, List::ByteArray(other_list) if list == other_list)
            }
            List::String(list) => matches!(other, List::String(other_list) if list == other_list),
            List::List(list) => matches!(other, List::List(other_list) if list == other_list),
            List::Compound(list) => {
                matches!(other, List::Compound(other_list) if list == other_list)
            }
            List::IntArray(list) => {
                matches!(other, List::IntArray(other_list) if list == other_list)
            }
            List::LongArray(list) => {
                matches!(other, List::LongArray(other_list) if list == other_list)
            }
        }
    }
}

macro_rules! impl_from {
    ($($type:ty, $variant:ident),*) => {
        $(
            impl<S> From<Vec<$type>> for List<S> {
                fn from(v: Vec<$type>) -> Self {
                    Self::$variant(v)
                }
            }
        )*
    };
}

impl_from!(i8, Byte);
impl_from!(i16, Short);
impl_from!(i32, Int);
impl_from!(i64, Long);
impl_from!(f32, Float);
impl_from!(f64, Double);
impl_from!(Vec<i8>, ByteArray);
impl_from!(Vec<i32>, IntArray);
impl_from!(Vec<i64>, LongArray);
impl_from!(List<S>, List);
impl_from!(Compound<S>, Compound);

impl From<Vec<String>> for List<String> {
    fn from(v: Vec<String>) -> Self {
        Self::String(v)
    }
}

impl<'a> From<Vec<Cow<'a, str>>> for List<Cow<'a, str>> {
    fn from(value: Vec<Cow<'a, str>>) -> Self {
        Self::String(value)
    }
}

impl<S> From<Value<S>> for List<S> {
    fn from(value: Value<S>) -> Self {
        match value {
            Value::Byte(v) => Self::Byte(vec![v]),
            Value::Short(v) => Self::Short(vec![v]),
            Value::Int(v) => Self::Int(vec![v]),
            Value::Long(v) => Self::Long(vec![v]),
            Value::Float(v) => Self::Float(vec![v]),
            Value::Double(v) => Self::Double(vec![v]),
            Value::ByteArray(v) => Self::ByteArray(vec![v]),
            Value::String(v) => Self::String(vec![v]),
            Value::List(v) => Self::List(vec![v]),
            Value::Compound(v) => Self::Compound(vec![v]),
            Value::IntArray(v) => Self::IntArray(vec![v]),
            Value::LongArray(v) => Self::LongArray(vec![v]),
        }
    }
}
