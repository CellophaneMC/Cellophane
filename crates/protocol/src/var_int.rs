use std::io::{Read, Write};
use std::num::TryFromIntError;

use byteorder::ReadBytesExt;
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::error::Error;
use crate::io::{Decoder, Encoder};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct VarInt(pub i32);

impl VarInt {
    pub const MAX_SIZE: usize = 5;

    pub const fn written_size(self) -> usize {
        match self.0 {
            0 => 1,
            n => (31 - n.leading_zeros() as usize) / 7 + 1,
        }
    }

    pub async fn read_async(reader: &mut (impl AsyncRead + Unpin + Send)) -> Result<Self, VarIntDecodeError> {
        let mut len = 0;
        for i in 0..Self::MAX_SIZE {
            use tokio::io::AsyncReadExt;
            let byte = reader.read_u8().await.map_err(|_| VarIntDecodeError::Incomplete)?;
            len |= (byte as i32 & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(VarInt(len));
            }
        }
        Err(VarIntDecodeError::TooLarge)
    }

    pub async fn write_async(
        &self,
        writer: &mut (impl AsyncWrite + Unpin + Send),
    ) -> Result<(), crate::error::Error> {
        let mut buf = Vec::with_capacity(self.written_size());
        self.write(&mut buf)?;

        writer.write_all(&buf).await.map_err(From::from)
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value)
    }
}

impl From<VarInt> for i32 {
    fn from(x: VarInt) -> Self {
        x.0
    }
}

impl TryFrom<VarInt> for usize {
    type Error = TryFromIntError;

    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl TryFrom<usize> for VarInt {
    type Error = TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        value.try_into().map(VarInt)
    }
}

impl Decoder for VarInt {
    fn read(mut reader: &mut impl Read) -> crate::error::Result<Self> {
        let mut val = 0;
        for i in 0..Self::MAX_SIZE {
            let byte = reader.read_u8().map_err(|_| Error::VarInt(VarIntDecodeError::Incomplete))?;
            val |= (byte as i32 & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(VarInt(val));
            }
        }

        Err(Error::VarInt(VarIntDecodeError::TooLarge))
    }
}

impl Encoder for VarInt {
    fn write(&self, mut writer: &mut impl Write) -> crate::error::Result<()> {
        let x = self.0 as u64;
        let stage1 = (x & 0x000000000000007f)
            | ((x & 0x0000000000003f80) << 1)
            | ((x & 0x00000000001fc000) << 2)
            | ((x & 0x000000000fe00000) << 3)
            | ((x & 0x00000000f0000000) << 4);

        let leading = stage1.leading_zeros();

        let unused_bytes = (leading - 1) >> 3;
        let bytes_needed = 8 - unused_bytes;

        // set all but the last MSBs
        let msbs = 0x8080808080808080;
        let msbmask = 0xffffffffffffffff >> (((8 - bytes_needed + 1) << 3) - 1);

        let merged = stage1 | (msbs & msbmask);
        let bytes = merged.to_le_bytes();

        writer.write_all(unsafe { bytes.get_unchecked(..bytes_needed as usize) })?;

        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Error)]
pub enum VarIntDecodeError {
    #[error("incomplete VarInt decode")]
    Incomplete,
    #[error("VarInt is too large")]
    TooLarge,
}
