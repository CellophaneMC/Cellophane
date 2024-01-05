use io::{Read, Write};
use std::borrow::Cow;
use std::hash::Hash;
use std::io;
use std::marker::PhantomData;

use bit_set::BitSet;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use glam::IVec3;
use uuid::Uuid;

use cellophanemc_core::block_pos::PackedBlockPos;
use cellophanemc_core::chunk_pos::{ChunkSection, DynPalette, PaletteContainer};
use cellophanemc_core::palette::Palette;
use cellophanemc_nbt::aa::{Value as NbtValue, Value};

use crate::error::{Error, Result};
use crate::var_int::VarInt;

pub trait Decoder: Sized {
    fn read(reader: &mut impl Read) -> Result<Self>;
}

pub trait Encoder {
    fn write(&self, writer: &mut impl Write) -> Result<()>;
}

macro_rules! integer_impl {
    ($($int:ty, $read_fn:tt, $write_fn:tt),* $(,)?) => {
        $(
            impl Decoder for $int {

                fn read(reader: &mut impl std::io::Read) -> Result<Self> {
                    reader.$read_fn::<byteorder::BigEndian>().map_err(From::from)
                }
            }

            impl Encoder for $int {
                fn write(&self, writer: &mut impl std::io::Write) -> Result<()> {
                    writer.$write_fn::<byteorder::BigEndian>(*self).map_err(From::from)
                }
            }
        )*
    }
}

integer_impl! {
    u16, read_u16, write_u16,
    u32, read_u32, write_u32,
    u64, read_u64, write_u64,

    i16, read_i16, write_i16,
    i32, read_i32, write_i32,
    i64, read_i64, write_i64,

    f32, read_f32, write_f32,
    f64, read_f64, write_f64,
}

impl Decoder for u8 {
    fn read(mut reader: &mut impl Read) -> Result<Self> {
        reader.read_u8().map_err(From::from)
    }
}

impl Encoder for u8 {
    fn write(&self, mut writer: &mut impl Write) -> Result<()> {
        writer.write_u8(*self).map_err(From::from)
    }
}

impl Decoder for i8 {
    fn read(mut reader: &mut impl Read) -> Result<Self> {
        reader.read_i8().map_err(From::from)
    }
}

impl Encoder for i8 {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        writer.write_i8(*self).map_err(From::from)
    }
}

impl Decoder for bool {
    fn read(mut reader: &mut impl Read) -> Result<Self> {
        reader.read_u8().map_err(From::from).map(|x| x != 0)
    }
}

impl Encoder for bool {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        writer.write_u8(if *self { 1 } else { 0 }).map_err(From::from)
    }
}

const STRING_MAX_LENGTH: usize = 32767;

impl Decoder for String {
    fn read(reader: &mut impl Read) -> Result<Self> {
        let len = VarInt::read(reader)?.0 as usize;

        let mut bytes = vec![0u8; len];
        reader.read_exact(&mut bytes)?;

        let str = std::str::from_utf8(&bytes).map_err(|e| Error::Utf8(e))?;

        Ok(str.to_owned())
    }
}

impl Encoder for String {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        let bytes = self.as_bytes();

        VarInt(bytes.len() as i32).write(writer)?;
        writer.write_all(bytes).map_err(From::from)
    }
}

impl Decoder for Uuid {
    fn read(reader: &mut impl Read) -> Result<Self> {
        let mut bytes = [0u8; 16];
        reader.read_exact(&mut bytes).map_err(Error::from)?;
        Ok(Uuid::from_bytes(bytes))
    }
}

impl Encoder for Uuid {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        writer.write_all(self.as_bytes()).map_err(Error::from)
    }
}

impl Decoder for BitSet {
    fn read(reader: &mut impl Read) -> Result<Self> {
        let serialized: Vec<u64> = VarIntPrefixedVec::read(reader)?.0.into_owned();

        let num_bits = serialized.len() * 64;
        let mut bitset = BitSet::with_capacity(num_bits);

        for (chunk_idx, &u64_value) in serialized.iter().enumerate() {
            for pos in 0..64 {
                if u64_value & (1u64 << pos) != 0 {
                    bitset.insert(chunk_idx * 64 + pos);
                }
            }
        }

        Ok(bitset)
    }
}

impl Encoder for BitSet {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        let mut serialized = Vec::new();
        let num_bits = self.len();

        for chunk in self.iter().collect::<Vec<usize>>().chunks(64) {
            let mut u64_value = 0u64;
            for &bit_idx in chunk {
                if bit_idx < num_bits {
                    let pos = bit_idx % 64;
                    u64_value |= 1u64 << pos;
                }
            }
            serialized.push(u64_value);
        }

        VarIntPrefixedVec::from(serialized).write(writer)
    }
}

impl Encoder for ChunkSection {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        let non_air_blocks = self.count_non_air_blocks() as u16;
        println!("non air blocks: {}", non_air_blocks);
        writer.write_u16::<BigEndian>(non_air_blocks)?;
        self.block_states.write(writer)?;
        self.biomes.write(writer)?;
        Ok(())
    }
}

impl<T: Into<u16> + Copy + Eq + Hash> Encoder for PaletteContainer<T> {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        println!("bits per aa: {}", self.storage.bits_per_value());
        let bits_per_value = self.storage.bits_per_value() as u8;
        writer.write_u8(bits_per_value)?;
        match &self.palette {
            DynPalette::Empty(_) => VarInt(0).write(writer)?,
            DynPalette::Single(palette) => VarInt(palette.0.into() as i32).write(writer)?,
            DynPalette::Liner(palette) => {
                VarInt(palette.len() as i32).write(writer)?;
                for entry in &palette.0 {
                    let a = T::into(*entry);
                    VarInt(a as i32).write(writer)?;
                }
            }
            DynPalette::Hash(palette) => {
                VarInt(palette.len() as i32).write(writer)?;
                for entry in &palette.entries {
                    let a = T::into(*entry);
                    VarInt(a as i32).write(writer)?;
                }
            }
        };
        println!("storage data length: {:?}", self.storage.bits.len());
        VarInt(self.storage.bits.len() as i32).write(writer)?;

        if self.storage.bits_per_value() > 0 {
            for x in self.storage.bits.iter() {
                writer.write_u64::<BigEndian>(*x)?;
            }
        }

        Ok(())
    }
}

impl Decoder for IVec3 {
    fn read(reader: &mut impl Read) -> Result<Self> {
        let block_pos = PackedBlockPos::from(reader.read_u64::<BigEndian>()?).into();
        Ok(block_pos)
    }
}

impl Encoder for IVec3 {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        let pos = PackedBlockPos::try_from(*self).map_err(Error::from)?;
        pos.write(writer)
    }
}

impl Decoder for PackedBlockPos {
    fn read(reader: &mut impl Read) -> Result<Self> {
        Ok(PackedBlockPos::from(reader.read_u64::<BigEndian>()?))
    }
}

impl Encoder for PackedBlockPos {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        writer.write_u64::<BigEndian>((*self).into()).map_err(Error::from)
    }
}

impl Encoder for NbtValue {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        writer.write_i8(self.id() as i8).map_err(Error::from)?;

        fn write_raw(nbt: &NbtValue, dst: &mut impl Write) -> io::Result<()> {
            match *nbt {
                Value::Byte(val) => dst.write_i8(val),
                Value::Short(val) => dst.write_i16::<BigEndian>(val),
                Value::Int(val) => dst.write_i32::<BigEndian>(val),
                Value::Long(val) => dst.write_i64::<BigEndian>(val),
                Value::Float(val) => dst.write_f32::<BigEndian>(val),
                Value::Double(val) => dst.write_f64::<BigEndian>(val),
                Value::ByteArray(ref val) => {
                    dst.write_i32::<BigEndian>(val.len() as i32)?;
                    for &i in val {
                        dst.write_i8(i)?;
                    }
                    Ok(())
                }
                Value::String(ref val) => {
                    let encoded = cesu8::to_java_cesu8(val.as_str());
                    dst.write_i16::<BigEndian>(encoded.len() as i16)?;
                    dst.write_all(&encoded)?;
                    Ok(())
                }
                Value::List(ref vals) => {
                    if vals.is_empty() {
                        dst.write_u8(0)?; // TAG_End
                        dst.write_i32::<BigEndian>(0)?;
                    } else {
                        let first_id = vals[0].id();
                        dst.write_u8(first_id)?;
                        dst.write_i32::<BigEndian>(vals.len() as i32)?;
                        for val in vals {
                            write_raw(val, dst)?;
                        }
                    }
                    Ok(())
                }
                Value::Compound(ref vals) => {
                    for (name, ref val) in vals.iter() {
                        dst.write_u8(val.id())?;
                        let name = cesu8::to_java_cesu8(name.as_str());
                        dst.write_i16::<BigEndian>(name.len() as i16)?;
                        dst.write_all(&name)?;
                        write_raw(val, dst)?;
                    }
                    dst.write_u8(0)?; // TAG_End
                    Ok(())
                }
                Value::IntArray(ref vals) => {
                    dst.write_i32::<BigEndian>(vals.len() as i32)?;
                    for &val in vals {
                        dst.write_i32::<BigEndian>(val)?;
                    }
                    Ok(())
                }
                Value::LongArray(ref vals) => {
                    dst.write_i32::<BigEndian>(vals.len() as i32)?;
                    for &val in vals {
                        dst.write_i64::<BigEndian>(val)?;
                    }
                    Ok(())
                }
                Value::Empty => {
                    dst.write_u8(0)?;
                    Ok(())
                }
            }
        }

        write_raw(self, writer).map_err(Error::from)
    }
}

impl Decoder for NbtValue {
    fn read(reader: &mut impl Read) -> Result<Self> {
        let id = i8::read(reader)?;
        Ok(Value::Byte(id))
    }
}

impl<T> Decoder for Option<T>
    where
        T: Decoder,
{
    fn read(reader: &mut impl Read) -> Result<Self> {
        let present = bool::read(reader)?;
        if present {
            Ok(Some(T::read(reader)?))
        } else {
            Ok(None)
        }
    }
}

impl<T> Encoder for Option<T>
    where
        T: Encoder,
{
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        match self {
            Some(value) => {
                true.write(writer)?;
                value.write(writer)?;
            }
            None => {
                false.write(writer)?;
            }
        }
        Ok(())
    }
}

const MAX_LENGTH: usize = 1024 * 1024; // 2^20 elements

pub struct FixedLengthVec<'a, const i: usize, T>(pub Cow<'a, [T]>) where [T]: ToOwned<Owned=Vec<T>>;

impl<'a, const i: usize, T> From<FixedLengthVec<'a, i, T>> for Vec<T>
    where
        [T]: ToOwned<Owned=Vec<T>>
{
    fn from(x: FixedLengthVec<'a, i, T>) -> Self {
        let a = x.0;
        let b = a.into_owned();
        b
    }
}

impl<'a, const i: usize, T> From<&'a [T]> for FixedLengthVec<'a, i, T>
    where
        [T]: ToOwned<Owned=Vec<T>>
{
    fn from(slice: &'a [T]) -> Self {
        Self(Cow::Borrowed(slice))
    }
}

impl<'a, const i: usize, T> From<Vec<T>> for FixedLengthVec<'a, i, T>
    where
        [T]: ToOwned<Owned=Vec<T>>
{
    fn from(vec: Vec<T>) -> Self {
        Self(Cow::Owned(vec))
    }
}

pub struct FixedBitSet<const i: usize>(pub BitSet);

impl<const i: usize> Decoder for FixedBitSet<i> {
    fn read(reader: &mut impl Read) -> Result<Self> {
        let mut bytes = vec![0u8; i / 8 + if i % 8 != 0 { 1 } else { 0 }];
        reader.read_exact(&mut bytes)?;
        Ok(Self(BitSet::from_bytes(&bytes)))
    }
}

impl<const i: usize> Encoder for FixedBitSet<i> {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        if self.0.len() > i {
            return Err(Error::BitSetLargerThanExpected(self.0.len(), i));
        }
        let bit_bytes = self.0.clone().into_bit_vec().to_bytes();
        for _ in 0..(i / 8 + if i % 8 != 0 { 1 } else { 0 }) - bit_bytes.len() {
            writer.write_u8(0)?;
        }
        Ok(())
    }
}

impl<const i: usize> Into<BitSet> for FixedBitSet<i> {
    fn into(self) -> BitSet {
        self.0
    }
}

pub type Fixed256VecU8<'a> = FixedLengthVec<'a, 256, u8>;

impl<'a, T, const i: usize> Decoder for FixedLengthVec<'a, i, T>
    where
        T: Decoder,
        [T]: ToOwned<Owned=Vec<T>>,
{
    fn read(reader: &mut impl Read) -> Result<Self> {
        let mut vec = Vec::with_capacity(i);
        for _ in 0..i {
            vec.push(T::read(reader)?);
        }
        Ok(Self(Cow::Owned(vec)))
    }
}

impl<'a, T, const i: usize> Encoder for FixedLengthVec<'a, i, T>
    where
        T: Encoder,
        [T]: ToOwned<Owned=Vec<T>>,
{
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        for item in self.0.iter() {
            item.write(writer)?;
        }
        Ok(())
    }
}

pub struct LengthPrefixedVec<'a, P, T>(pub Cow<'a, [T]>, PhantomData<P>)
    where
        [T]: ToOwned<Owned=Vec<T>>;

impl<'a, P, T> Decoder for LengthPrefixedVec<'a, P, T>
    where
        T: Decoder,
        [T]: ToOwned<Owned=Vec<T>>,
        P: Decoder + TryInto<usize>,
        P::Error: std::error::Error + Send + Sync + 'static,
{
    fn read(reader: &mut impl Read) -> Result<Self> {
        let len = P::read(reader)?.try_into().map_err(|e| Error::Io(
            io::Error::new(io::ErrorKind::InvalidData, e)
        ))?;

        // if len > MAX_LENGTH {
        // bail!(
        //     "length {} exceeds maximum allowed length of {}",
        //     len,
        //     MAX_LENGTH
        // )
        // }

        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(T::read(reader)?);
        }

        Ok(Self(Cow::Owned(vec), PhantomData))
    }
}

impl<'a, P, T> Encoder for LengthPrefixedVec<'a, P, T>
    where
        T: Encoder,
        [T]: ToOwned<Owned=Vec<T>>,
        P: Encoder + TryFrom<usize>,
        P::Error: std::error::Error + Send + Sync + 'static,
{
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        let len = P::try_from(self.0.len()).map_err(|e| Error::Io(
            io::Error::new(io::ErrorKind::InvalidData, e)
        ))?;
        len.write(writer)?;

        for item in self.0.iter() {
            item.write(writer)?;
        }

        Ok(())
    }
}

impl<'a, P, T> From<LengthPrefixedVec<'a, P, T>> for Vec<T>
    where
        [T]: ToOwned<Owned=Vec<T>>,
{
    fn from(x: LengthPrefixedVec<'a, P, T>) -> Self {
        x.0.into_owned()
    }
}

impl<'a, P, T> From<&'a [T]> for LengthPrefixedVec<'a, P, T>
    where
        [T]: ToOwned<Owned=Vec<T>>,
{
    fn from(slice: &'a [T]) -> Self {
        Self(Cow::Borrowed(slice), PhantomData)
    }
}

impl<'a, P, T> From<Vec<T>> for LengthPrefixedVec<'a, P, T>
    where
        [T]: ToOwned<Owned=Vec<T>>,
{
    fn from(vec: Vec<T>) -> Self {
        Self(Cow::Owned(vec), PhantomData)
    }
}

pub type VarIntPrefixedVec<'a, T> = LengthPrefixedVec<'a, VarInt, T>;
pub type ShortPrefixedVec<'a, T> = LengthPrefixedVec<'a, u16, T>;

pub struct LengthInferredVecU8<'a>(pub Cow<'a, [u8]>);

impl<'a> Decoder for LengthInferredVecU8<'a> {
    fn read(reader: &mut impl Read) -> Result<Self> {
        let mut vec = Vec::new();
        reader.read_to_end(&mut vec).map_err(Error::from)?;
        Ok(LengthInferredVecU8(Cow::Owned(vec)))
    }
}

impl<'a> Encoder for LengthInferredVecU8<'a> {
    fn write(&self, writer: &mut impl Write) -> Result<()> {
        writer.write_all(&self.0).map_err(Error::from)
    }
}

impl<'a> From<&'a [u8]> for LengthInferredVecU8<'a> {
    fn from(slice: &'a [u8]) -> Self {
        LengthInferredVecU8(Cow::Borrowed(slice))
    }
}

impl<'a> From<LengthInferredVecU8<'a>> for Vec<u8> {
    fn from(x: LengthInferredVecU8<'a>) -> Self {
        x.0.into_owned()
    }
}
