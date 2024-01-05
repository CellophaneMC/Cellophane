use std::convert::TryFrom;

use bitfield_struct::bitfield;
use derive_more::From;
use glam::IVec3;
use thiserror::Error;

#[bitfield(u64)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct PackedBlockPos {
    #[bits(12)]
    pub y: i32,
    #[bits(26)]
    pub z: i32,
    #[bits(26)]
    pub x: i32,
}

impl From<PackedBlockPos> for IVec3 {
    #[inline(always)]
    fn from(p: PackedBlockPos) -> Self {
        IVec3::new(p.x(), p.y(), p.z())
    }
}

impl TryFrom<IVec3> for PackedBlockPos {
    type Error = Error;

    fn try_from(value: IVec3) -> Result<Self, Self::Error> {
        match (value.x, value.y, value.z) {
            (-0x2000000..=0x1ffffff, -0x800..=0x7ff, -0x2000000..=0x1ffffff) => {
                Ok(
                    PackedBlockPos::new()
                        .with_x(value.x)
                        .with_y(value.y)
                        .with_z(value.z)
                )
            }
            _ => Err(Error(value)),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Error, From)]
#[error("block position of {0} is out of range")]
pub struct Error(pub IVec3);
