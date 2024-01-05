use cellophanemc_core::biome::Biome;

pub trait BiomeGenerator {
    fn generate_biome(&self, x: i32, y: i32, z: i32) -> Biome;
}

pub struct SingleBiomeGenerator {
    pub(crate) biome: Biome,
}

impl BiomeGenerator for SingleBiomeGenerator {
    #[inline(always)]
    fn generate_biome(&self, x: i32, y: i32, z: i32) -> Biome {
        self.biome
    }
}
