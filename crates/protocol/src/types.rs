use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::{Decoder, Encoder, VarInt};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Angle(pub u8);

impl Decoder for Angle {
    fn read(mut reader: &mut impl Read) -> crate::error::Result<Self> {
        let value = reader.read_u8()?;
        Ok(Angle(value))
    }
}

impl Encoder for Angle {
    fn write(&self, mut writer: &mut impl Write) -> crate::error::Result<()> {
        writer.write_u8(self.0)?;
        Ok(())
    }
}

impl From<u8> for Angle {
    fn from(value: u8) -> Self {
        Angle(value)
    }
}

impl From<Angle> for u8 {
    fn from(x: Angle) -> Self {
        x.0
    }
}

impl From<Angle> for f32 {
    fn from(x: Angle) -> Self {
        x.0 as f32 * 360.0 / 256.0
    }
}

impl From<f32> for Angle {
    fn from(x: f32) -> Self {
        Angle((x * 256.0 / 360.0) as u8)
    }
}

impl From<Angle> for f64 {
    fn from(x: Angle) -> Self {
        x.0 as f64 * 360.0 / 256.0
    }
}

impl From<f64> for Angle {
    fn from(x: f64) -> Self {
        Angle((x * 256.0 / 360.0) as u8)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum Slot {
    Empty,
    Data {
        item_id: usize,
        count: u8,
        nbt: cellophanemc_nbt::aa::Value,
    },
}

impl Decoder for Slot {
    fn read(reader: &mut impl Read) -> crate::error::Result<Self> {
        if reader.read_u8()? == 0 {
            Ok(Slot::Empty)
        } else {
            let item_id = VarInt::read(reader)?.0 as usize;
            let count = reader.read_u8()?;
            let nbt = cellophanemc_nbt::aa::Value::read(reader)?;
            Ok(Slot::Data {
                item_id,
                count,
                nbt,
            })
        }
    }
}

impl Encoder for Slot {
    fn write(&self, writer: &mut impl Write) -> crate::error::Result<()> {
        match self {
            Slot::Empty => {
                writer.write_u8(0)?;
            }
            Slot::Data {
                item_id,
                count,
                nbt,
            } => {
                writer.write_u8(1)?;
                VarInt(*item_id as i32).write(writer)?;
                writer.write_u8(*count)?;
                nbt.write(writer)?;
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub enum Hand {
    #[default]
    Main,
    Off,
}

impl Decoder for Hand {
    fn read(reader: &mut impl Read) -> crate::error::Result<Self> {
        let value = reader.read_u8()?;
        match value {
            0 => Ok(Hand::Main),
            1 => Ok(Hand::Off),
            _ => Err(crate::error::Error::InvalidDiscriminant(value as i32)),
        }
    }
}

impl Encoder for Hand {
    fn write(&self, writer: &mut impl Write) -> crate::error::Result<()> {
        match self {
            Hand::Main => writer.write_u8(0)?,
            Hand::Off => writer.write_u8(1)?,
        }
        Ok(())
    }
}
