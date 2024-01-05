use core::default::Default;
use std::collections::BTreeMap;

use cellophanemc_core::biome::Biome;
use cellophanemc_core::block::Block;
use cellophanemc_core::block_state::BlockStateId;
use cellophanemc_core::volume::{BiomeVolumeMut, BlockVolumeMut};

use crate::biome_generator::{BiomeGenerator, SingleBiomeGenerator};
use crate::chunk::Chunk;

#[derive(Debug, Clone)]
pub struct FlatGeneratorSettings {
    biome: String,
    features: bool,
    lakes: bool,
    layers: Vec<FlatGeneratorLayer>,
    structure_overrides: Vec<String>,
}

impl Default for FlatGeneratorSettings {
    fn default() -> Self {
        Self {
            biome: "minecraft:plains".to_string(),
            features: false,
            lakes: false,
            layers: Default::default(),
            structure_overrides: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
struct FlatGeneratorLayer {
    block: String,
    height: usize,
}

impl Default for FlatGeneratorLayer {
    fn default() -> Self {
        Self {
            block: "minecraft:air".to_string(),
            height: 16,
        }
    }
}

pub struct FlatWorldGenerator {
    layers: Vec<BlockStateId>,
    biome_generator: SingleBiomeGenerator,
    void_gen: bool,
}

impl FlatWorldGenerator {
    pub fn new(block_registry: &BTreeMap<String, Block>, settings: FlatGeneratorSettings) -> FlatWorldGenerator {
        let mut layers = vec![];
        for x in settings.layers {
            let block = block_registry.get(&x.block).expect("Block not found in block registry").default_block_state().expect("Default block state not defined");
            for _ in 0..x.height {
                layers.push(block.id);
            }
        }
        let void_gen = layers.iter().all(|&x| {
            x.0 == 0
        });

        Self {
            layers,
            biome_generator: SingleBiomeGenerator { biome: Biome(0) },
            void_gen,
        }
    }

    pub fn fill_blocks(&self, chunk: &mut Chunk) {
        for (y, block) in self.layers.iter().enumerate() {
            for x in 0..16 {
                for z in 0..16 {
                    chunk.set_block_at(x, y as i32, z, *block)
                }
            }
        }
    }

    pub fn fill_biomes(&self, chunk: &mut Chunk) {
        for y in 0..4 {
            for x in 0..4 {
                for z in 0..4 {
                    let biome = self.biome_generator.generate_biome(x * 4, y * 4, z * 4);
                    chunk.set_biome(x * 4, y * 4, z * 4, biome)
                }
            }
        }
    }
}
