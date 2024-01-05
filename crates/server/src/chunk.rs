use glam::{IVec3, ivec3};

use cellophanemc_core::biome::Biome;
use cellophanemc_core::block_state::BlockStateId;
use cellophanemc_core::chunk_pos::{ChunkPos, ChunkSection, PaletteContainer, Strategy};
use cellophanemc_core::volume::{BiomeVolume, BiomeVolumeMut, BlockVolume, BlockVolumeMut, Volume};

pub struct Chunk {
    chunk_pos: ChunkPos,
    pub sections: Vec<ChunkSection>,
    min_build_height: i32,
}

impl Chunk {
    pub fn new(chunk_pos: ChunkPos) -> Self {
        let mut sections = vec![ChunkSection {
            block_states: PaletteContainer::new(Strategy::Chunk, BlockStateId(0)),
            biomes: PaletteContainer::new(Strategy::Biome, Biome(0)),
        }; 24];

        for mut section in sections.iter_mut() {
            for biome_x in 0..4 {
                for biome_y in 0..4 {
                    for biome_z in 0..4 {
                        section.biomes.set(biome_x, biome_y, biome_z, Biome(0));
                    }
                }
            }
        }

        Self {
            chunk_pos,
            sections,
            min_build_height: -64,
        }
    }

    #[inline(always)]
    fn block_to_section(y: i32) -> i32 {
        y >> 4
    }

    #[inline(always)]
    fn min_section_y(&self) -> i32 {
        Chunk::block_to_section(self.min_build_height)
    }

    #[inline(always)]
    fn section_index_from_section_y(&self, section_y: i32) -> usize {
        (section_y - self.min_section_y()) as usize
    }

    #[inline(always)]
    fn section_index(&self, block_y: i32) -> usize {
        self.section_index_from_section_y(Chunk::block_to_section(block_y))
    }
}

impl Volume for Chunk {
    fn min(&self) -> IVec3 {
        ivec3(0, -64, 0) + IVec3::from(self.chunk_pos)
    }

    fn max(&self) -> IVec3 {
        self.min() + ivec3(15, 384, 15)
    }

    fn contains(&self, x: i32, y: i32, z: i32) -> bool {
        let min = self.min();
        let max = self.max();
        x >= min.x && x <= max.x
            && y >= min.y && y <= max.y
            && z >= min.z && z <= max.z
    }

    fn is_area_available(&self, x: i32, y: i32, z: i32) -> bool {
        self.contains(x, y, z)
    }
}

impl BlockVolume for Chunk {
    fn block_at(&self, x: i32, y: i32, z: i32) -> BlockStateId {
        let section_y = self.section_index(y);
        let block_state = self.sections[section_y].block_states.get((x & 15) as usize, (y & 15) as usize, (z & 15) as usize);
        block_state.unwrap_or(BlockStateId(0))
    }
}

impl BlockVolumeMut for Chunk {
    fn set_block_at(&mut self, x: i32, y: i32, z: i32, block: BlockStateId) {
        let section_y = self.section_index(y);
        self.sections[section_y].block_states.set((x & 15) as usize, (y & 15) as usize, (z & 15) as usize, block);
    }

    fn remove_block_at(&mut self, x: i32, y: i32, z: i32) {
        self.set_block_at(x, y, z, BlockStateId(0))
    }
}

impl BiomeVolume for Chunk {
    fn biome(&self, x: i32, y: i32, z: i32) -> Biome {
        let section_y = self.section_index(y);
        let biome = self.sections[section_y].biomes.get((x & 15) as usize, (y & 15) as usize, (z & 15) as usize);
        biome.unwrap_or(Biome(0))
    }
}

impl BiomeVolumeMut for Chunk {
    fn set_biome(&mut self, x: i32, y: i32, z: i32, biome: Biome) {
        let section_y = self.section_index(y);
        self.sections[section_y].biomes.set((x & 3) as usize, (y & 3) as usize, (z & 3) as usize, biome);
    }
}
