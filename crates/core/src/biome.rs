use std::fmt;
use glam::IVec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Biome(pub u16);

impl Into<u16> for Biome {
    #[inline(always)]
    fn into(self) -> u16 {
        self.0
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct BiomePos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BiomePos {
    pub const fn new(
        x: i32,
        y: i32,
        z: i32,
    ) -> Self {
        Self { x, y, z }
    }
}

impl From<IVec3> for BiomePos {
    fn from(pos: IVec3) -> Self {
        Self {
            x: pos.x.div_euclid(4),
            y: pos.y.div_euclid(4),
            z: pos.z.div_euclid(4),
        }
    }
}

impl fmt::Display for BiomePos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&(self.x, self.y, self.z), f)
    }
}
