use std::fmt;
use std::hash::Hash;

use glam::{IVec3, ivec3};

use crate::biome::Biome;
use crate::block_state::BlockStateId;
use crate::packed_array::PackedArray;
use crate::palette::{EmptyPalette, HashPalette, LinerPalette, Palette, PaletteMut, SinglePalette};

const CHUNK_BLOCK_SIZE: usize = 16;
pub(super) const SECTION_BLOCK_COUNT: usize = CHUNK_BLOCK_SIZE * CHUNK_BLOCK_SIZE * CHUNK_BLOCK_SIZE;
pub(super) const SECTION_BIOME_COUNT: usize = 4 * 4 * 4;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct ChunkPos {
    pub x: i32,
    pub z: i32,
}

impl ChunkPos {
    #[inline(always)]
    pub const fn new(x: i32, z: i32) -> Self {
        ChunkPos {
            x,
            z,
        }
    }
}

impl From<IVec3> for ChunkPos {
    #[inline]
    fn from(value: IVec3) -> Self {
        ChunkPos {
            x: value.x.div_euclid(CHUNK_BLOCK_SIZE as i32),
            z: value.x.div_euclid(CHUNK_BLOCK_SIZE as i32),
        }
    }
}

impl From<ChunkPos> for IVec3 {
    #[inline]
    fn from(value: ChunkPos) -> Self {
        ivec3(value.x * CHUNK_BLOCK_SIZE as i32, 0, value.z * CHUNK_BLOCK_SIZE as i32)
    }
}

impl fmt::Display for ChunkPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&(self.x, self.z), f)
    }
}

impl Into<u64> for ChunkPos {
    fn into(self) -> u64 {
        ((self.x as u64) & 0xFFFFFFFF) | (((self.z as u64) & 0xFFFFFFFF) << 32)
    }
}

impl From<u64> for ChunkPos {
    fn from(value: u64) -> Self {
        ChunkPos {
            x: (value & 0xFFFFFFFF) as i32,
            z: ((value >> 32) & 0xFFFFFFFF) as i32,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChunkSection {
    pub block_states: PaletteContainer<BlockStateId>,
    pub biomes: PaletteContainer<Biome>,
}

impl ChunkSection {
    pub fn count_non_air_blocks(&self) -> usize {
        let mut count = 0;
        match &self.block_states.palette {
            DynPalette::Empty(_) => {}
            DynPalette::Single(x) => {
                if x.0 != Default::default() {
                    count += SECTION_BIOME_COUNT
                }
            }
            DynPalette::Liner(x) => {
                for i in 0..SECTION_BLOCK_COUNT {
                    if self.block_states.storage.get(i).unwrap_or(0) != 0 {
                        count += 1;
                    }
                }
            }
            DynPalette::Hash(x) => {
                for i in 0..SECTION_BLOCK_COUNT {
                    if self.block_states.storage.get(i).unwrap_or(0) != 0 {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaletteContainer<T: Copy + Eq + PartialEq + Hash> {
    pub strategy: Strategy,
    pub palette: DynPalette<T>,
    pub storage: PackedArray,
}

impl<T: Copy + Eq + Hash + std::fmt::Debug> PaletteContainer<T> {
    pub fn new(strategy: Strategy, default: T) -> Self {
        let max_entries = strategy.max_entries();
        let min_bits_per_entry = strategy.min_bits_per_entry();
        Self {
            strategy,
            palette: DynPalette::Single(
                SinglePalette(default)
            ),
            storage: PackedArray::new(max_entries, min_bits_per_entry),
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<T> {
        let index = self.strategy.index(x, y, z);
        if let Some(id) = self.storage.get(index) {
            self.palette.get(id as usize)
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, val: T) -> usize {
        let id = self.palette.insert(val);
        if self.palette.bits_per_value() > self.storage.bits_per_value() {
            let mut new_storage = PackedArray::new(
                self.strategy.max_entries(),
                self.palette.bits_per_value(),
            );
            for i in 0..self.strategy.max_entries() {
                if let Some(val) = self.storage.get(i) {
                    new_storage.set(i, val);
                }
            }
            self.storage = new_storage;
        }
        let pos_index = self.strategy.index(x, y, z);
        self.storage.set(pos_index, id as u64);
        id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DynPalette<T: Copy + Eq + Hash> {
    Empty(EmptyPalette<T>),
    Single(SinglePalette<T>),
    Liner(LinerPalette<T>),
    Hash(HashPalette<T>),
}

impl<T: Copy + Eq + Hash> DynPalette<T> {
    pub fn new() -> Self <> {
        Default::default()
    }

    fn bits_per_value(&self) -> usize {
        match self {
            DynPalette::Empty(_) => 0,
            DynPalette::Single(_) => 1,
            _ => {
                (usize::BITS - (self.len() - 1).leading_zeros()).try_into().unwrap()
            }
        }
    }
}

impl<T: Copy + Eq + Hash> Default for DynPalette<T> {
    fn default() -> Self {
        Self::Empty(EmptyPalette::new())
    }
}

impl<T: Copy + Eq + Hash> Palette<T> for DynPalette<T> {
    fn len(&self) -> usize {
        match self {
            DynPalette::Empty(p) => p.len(),
            DynPalette::Single(p) => p.len(),
            DynPalette::Liner(p) => p.len(),
            DynPalette::Hash(p) => p.len(),
        }
    }

    fn get(&self, idx: usize) -> Option<T> {
        match self {
            DynPalette::Empty(p) => p.get(idx),
            DynPalette::Single(p) => p.get(idx),
            DynPalette::Liner(p) => p.get(idx),
            DynPalette::Hash(p) => p.get(idx),
        }
    }

    fn index(&self, val: T) -> Option<usize> {
        match self {
            DynPalette::Empty(p) => p.index(val),
            DynPalette::Single(p) => p.index(val),
            DynPalette::Liner(p) => p.index(val),
            DynPalette::Hash(p) => p.index(val),
        }
    }
}

impl<T: Copy + Eq + Hash> PaletteMut<T> for DynPalette<T> {
    fn insert(&mut self, val: T) -> usize {
        match self {
            DynPalette::Empty(_) => {
                *self = DynPalette::Single(SinglePalette::new(val));
                0
            }
            DynPalette::Single(_) => {
                *self = DynPalette::Liner(
                    LinerPalette::from(vec![self.get(0).unwrap(), val])
                );
                1
            }
            DynPalette::Liner(l) => {
                if l.len() < 16 {
                    l.insert(val)
                } else {
                    let mut new = HashPalette::from(l.0.clone());
                    let result = new.insert(val);
                    *self = DynPalette::Hash(new);
                    result
                }
            }
            DynPalette::Hash(h) => h.insert(val),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Strategy {
    Chunk,
    Biome,
}

impl Strategy {
    const fn min_bits_per_entry(&self) -> usize {
        match self {
            Self::Biome => 1,
            Self::Chunk => 4
        }
    }

    const fn max_bits_per_entry(&self) -> usize {
        match self {
            Self::Biome => 3,
            Self::Chunk => 8
        }
    }

    const fn max_entries(&self) -> usize {
        match self {
            Self::Biome => 64,
            Self::Chunk => 4096
        }
    }

    const fn size_bits(&self) -> usize {
        match self {
            Self::Biome => 2,
            Self::Chunk => 4
        }
    }

    const fn index(&self, x: usize, y: usize, z: usize) -> usize {
        let size_bits = self.size_bits();
        (y << size_bits | z) << size_bits | x
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};

    use super::*;

    #[test]
    fn foo() {
        let mut container = PaletteContainer::new(Strategy::Chunk, 0);
        container.set(1, 0, 1, 1);
        container.set(2, 2, 2, 2);
        container.set(2, 3, 3, 3);
        container.set(4, 4, 0, 4);
        container.set(0, 5, 5, 5);
        container.set(6, 6, 6, 6);
        assert_eq!(container.get(1, 0, 1), Some(1));
        assert_eq!(container.get(2, 2, 2), Some(2));
        assert_eq!(container.get(2, 3, 3), Some(3));
        assert_eq!(container.get(4, 4, 0), Some(4));
        assert_eq!(container.get(0, 5, 5), Some(5));
        assert_eq!(container.get(6, 6, 6), Some(6));
        assert_eq!(container.get(7, 7, 7), Some(0));
        let a = container.get(7, 7, 7);
        println!("{:?}", a)
    }

    #[test]
    fn add_blocks() {
        let mut palette = DynPalette::new();

        for i in 0..100 {
            let index = palette.insert(i);
            assert_eq!(index, i as usize);
            assert_eq!(palette.get(index).unwrap(), i);
        }

        assert_eq!(palette.len(), 100);
    }

    #[test]
    fn a() {
        let mut section = ChunkSection {
            block_states: PaletteContainer::new(Strategy::Chunk, BlockStateId(0)),
            biomes: PaletteContainer::new(Strategy::Biome, Biome(0)),
        };

        let mut rng = thread_rng();
        for x in 0..15 {
            for z in 0..15 {
                for y in 0..15 {
                    if y > 0 {
                        // section.block_states.set(x, y, z, 0);
                    } else {
                        let v: bool = rng.gen();
                        if v {
                            section.block_states.set(x, z, z, BlockStateId(1));
                        } else {
                            section.block_states.set(x, z, z, BlockStateId(2));
                        }
                    }
                }
            }
        }

        for x in 0..15 {
            for z in 0..15 {
                for y in 0..15 {
                    println!("block: {:?}", section.block_states.get(x, y, z))
                }
            }
        }

        for x in &section.block_states.storage.bits {
            let a = format!("{x:064b}");

            let mut split_string = String::new();

            for (i, ch) in a.chars().enumerate() {
                // Append the character to the split_string
                split_string.push(ch);
                // Add a colon after every 4 characters except for the last one
                if (i + 1) % 4 == 0 && i != a.len() - 1 {
                    split_string.push(':');
                }
            }
            println!("entries: {split_string} | {x} | {x:016X}")
        }

        println!("{:?}", section);
    }
}
