use std::hash::Hash;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::error::{Error, Result};
use crate::{Compound, List, Tag, Value};

pub fn from_binary<R, S>(slice: R) -> Result<(Compound<S>, S)>
where
    R: Read,
    S: FromModifiedUtf8 + Hash + Ord,
{
    let mut state = DecodeState::new(slice);
    let root_tag = state.read_tag()?;

    if root_tag != Tag::Compound {
        return Err(Error::RootTagNotCompound(root_tag));
    }

    let root_name = state.read_string::<S>()?;
    let root_value = state.read_compound()?;

    Ok((root_value, root_name))
}

const MAX_DEPTH: usize = 512;

struct DecodeState<R: Read> {
    reader: R,
    depth: usize,
}

impl<R: Read> DecodeState<R> {
    fn new(reader: R) -> Self {
        DecodeState { reader, depth: 0 }
    }

    #[inline]
    fn check_depth<T>(&mut self, f: impl FnOnce(&mut Self) -> Result<T>) -> Result<T> {
        if self.depth >= MAX_DEPTH {
            return Err(Error::RecursionLimitExceeded);
        }

        self.depth += 1;
        let res = f(self);
        self.depth -= 1;
        res
    }

    fn read_tag(&mut self) -> Result<Tag> {
        self.reader.read_u8()?.try_into()
    }

    fn read_value<S>(&mut self, tag: Tag) -> Result<Value<S>>
    where
        S: FromModifiedUtf8 + Hash + Ord,
    {
        match tag {
            Tag::Byte => Ok(self.read_byte()?.into()),
            Tag::Short => Ok(self.read_short()?.into()),
            Tag::Int => Ok(self.read_int()?.into()),
            Tag::Long => Ok(self.read_long()?.into()),
            Tag::Float => Ok(self.read_float()?.into()),
            Tag::Double => Ok(self.read_double()?.into()),
            Tag::ByteArray => Ok(self.read_byte_array()?.into()),
            Tag::String => Ok(Value::String(self.read_string::<S>()?)),
            Tag::List => self.check_depth(|st| Ok(st.read_any_list::<S>()?.into())),
            Tag::Compound => self.check_depth(|st| Ok(st.read_compound::<S>()?.into())),
            Tag::IntArray => Ok(self.read_int_array()?.into()),
            Tag::LongArray => Ok(self.read_long_array()?.into()),
            Tag::End => Err(Error::EndTagInValue),
        }
    }

    fn read_byte(&mut self) -> Result<i8> {
        Ok(self.reader.read_i8()?)
    }

    fn read_short(&mut self) -> Result<i16> {
        Ok(self.reader.read_i16::<BigEndian>()?)
    }

    fn read_int(&mut self) -> Result<i32> {
        Ok(self.reader.read_i32::<BigEndian>()?)
    }

    fn read_long(&mut self) -> Result<i64> {
        Ok(self.reader.read_i64::<BigEndian>()?)
    }

    fn read_float(&mut self) -> Result<f32> {
        Ok(self.reader.read_f32::<BigEndian>()?)
    }

    fn read_double(&mut self) -> Result<f64> {
        Ok(self.reader.read_f64::<BigEndian>()?)
    }

    fn read_byte_array(&mut self) -> Result<Vec<i8>> {
        let len = self.read_int()?;
        let mut buf = vec![0i8; len as usize];
        self.reader.read_i8_into(&mut buf)?;
        Ok(buf)
    }

    fn read_string<S>(&mut self) -> Result<S>
    where
        S: FromModifiedUtf8,
    {
        let len = self.read_short()?;
        let mut buf = vec![0u8; len as usize];
        self.reader.read_exact(&mut buf)?;

        S::from_modified_utf8(&buf)
    }

    fn read_any_list<S>(&mut self) -> Result<List<S>>
    where
        S: FromModifiedUtf8 + Hash + Ord,
    {
        match self.read_tag()? {
            Tag::Byte => Ok(self.read_list(|s| s.read_byte())?.into()),
            Tag::Short => Ok(self.read_list(|s| s.read_short())?.into()),
            Tag::Int => Ok(self.read_list(|s| s.read_int())?.into()),
            Tag::Long => Ok(self.read_list(|s| s.read_long())?.into()),
            Tag::Float => Ok(self.read_list(|s| s.read_float())?.into()),
            Tag::Double => Ok(self.read_list(|s| s.read_double())?.into()),
            Tag::ByteArray => Ok(self.read_list(|s| s.read_byte_array())?.into()),
            Tag::String => Ok(List::String(self.read_list(|s| s.read_string::<S>())?)),
            Tag::List => self.check_depth(|s| Ok(s.read_list(|s| s.read_any_list::<S>())?.into())),
            Tag::Compound => {
                self.check_depth(|s| Ok(s.read_list(|s| s.read_compound::<S>())?.into()))
            }
            Tag::IntArray => Ok(self.read_list(|s| s.read_int_array())?.into()),
            Tag::LongArray => Ok(self.read_list(|s| s.read_long_array())?.into()),
            Tag::End => match self.read_int()? {
                0 => Ok(List::End),
                len => Err(Error::TagEndListWithNonZeroLength(len)),
            },
        }
    }

    fn read_list<T, F>(&mut self, mut read_elem: F) -> Result<Vec<T>>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        let len = self.read_int()?;
        if len.is_negative() {
            return Err(Error::InvalidArrayLength(len));
        }
        let mut buf = Vec::with_capacity(len as usize);
        for _ in 0..len {
            buf.push(read_elem(self)?);
        }
        Ok(buf)
    }

    fn read_compound<S>(&mut self) -> Result<Compound<S>>
    where
        S: FromModifiedUtf8 + Hash + Ord,
    {
        let mut compound = Compound::new();
        loop {
            let tag = self.read_tag()?;
            if tag == Tag::End {
                return Ok(compound);
            }
            let name = self.read_string::<S>()?;
            let value = self.read_value::<S>(tag)?;
            compound.insert(name, value);
        }
    }

    fn read_int_array(&mut self) -> Result<Vec<i32>> {
        let len = self.read_int()?;
        if len.is_negative() {
            return Err(Error::InvalidArrayLength(len));
        }
        let mut buf = vec![0i32; len as usize];
        self.reader.read_i32_into::<BigEndian>(&mut buf)?;
        Ok(buf)
    }

    fn read_long_array(&mut self) -> Result<Vec<i64>> {
        let len = self.read_int()?;
        if len.is_negative() {
            return Err(Error::InvalidArrayLength(len));
        }
        let mut buf = vec![0i64; len as usize];
        self.reader.read_i64_into::<BigEndian>(&mut buf)?;
        Ok(buf)
    }
}

pub trait FromModifiedUtf8: Sized {
    fn from_modified_utf8(bytes: &[u8]) -> Result<Self>;
}

impl FromModifiedUtf8 for String {
    fn from_modified_utf8(bytes: &[u8]) -> Result<Self> {
        match cesu8::from_java_cesu8(bytes) {
            Ok(str) => Ok(str.into_owned()),
            Err(e) => Err(Error::InvalidUtf8(e)),
        }
    }
}

// impl<'de> FromModifiedUtf8 for Cow<'de, str> {
//     fn from_modified_utf8(bytes: &[u8]) -> Result<Self> {
//         cesu8::from_java_cesu8(bytes).map_err(move |e| Error::InvalidUtf8(e))
//     }
// }
