use glam::IVec3;

use crate::biome::Biome;
use crate::block_state::BlockStateId;

pub trait Volume {
    fn min(&self) -> IVec3;

    fn max(&self) -> IVec3;

    fn contains(&self, x: i32, y: i32, z: i32) -> bool;

    fn is_area_available(&self, x: i32, y: i32, z: i32) -> bool;
}

pub trait BlockVolume: Volume {
    fn block_at(&self, x: i32, y: i32, z: i32) -> BlockStateId;
}

pub trait BlockVolumeMut: BlockVolume {
    fn set_block_at(&mut self, x: i32, y: i32, z: i32, block: BlockStateId);

    fn remove_block_at(&mut self, x: i32, y: i32, z: i32);
}

pub trait BiomeVolume: Volume {
    fn biome(&self, x: i32, y: i32, z: i32) -> Biome;
}

pub trait BiomeVolumeMut: BiomeVolume {
    fn set_biome(&mut self, x: i32, y: i32, z: i32, biome: Biome);
}
