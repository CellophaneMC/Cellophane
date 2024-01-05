use std::cmp::{max, min};

use cellophanemc_core::chunk_pos::ChunkPos;

pub trait ChunkFilter {
    fn contains(&self, pos: &ChunkPos, include_edge: bool) -> bool;

    fn for_each_chunk_within_distance<F>(&self, f: F)
        where
            F: Fn(&ChunkPos);
}

fn is_within_distance(pos1: &ChunkPos, pos2: &ChunkPos, distance: usize, include_edge: bool) -> bool {
    let i = max(0, (pos1.x - pos2.x).abs() - 1);
    let j = max(0, (pos1.z - pos2.z).abs() - 1);
    let l = max(0, max(i, j) - if include_edge { 1 } else { 0 }) as i64;
    let m = min(i, j) as i64;
    let n = m * m + l * l;
    let k = distance as i64 * distance as i64;
    n < k
}

pub struct CylindricalChunkFilter {
    center: ChunkPos,
    view_distance: usize,
}

impl CylindricalChunkFilter {
    pub fn new(center: ChunkPos, view_distance: usize) -> Self {
        Self {
            center,
            view_distance,
        }
    }

    pub fn max_x(&self) -> i32 {
        self.center.x + self.view_distance as i32
    }

    pub fn min_x(&self) -> i32 {
        self.center.x - self.view_distance as i32
    }

    pub fn max_z(&self) -> i32 {
        self.center.z + self.view_distance as i32
    }

    pub fn min_z(&self) -> i32 {
        self.center.z - self.view_distance as i32
    }

    pub fn overlaps(&self, other: &CylindricalChunkFilter) -> bool {
        self.max_x() >= other.min_x() && self.min_x() <= other.max_x() && self.max_z() >= other.min_z() && self.min_z() <= other.max_z()
    }
}

impl ChunkFilter for CylindricalChunkFilter {
    fn contains(&self, pos: &ChunkPos, include_edge: bool) -> bool {
        is_within_distance(&self.center, pos, self.view_distance, include_edge)
    }

    fn for_each_chunk_within_distance<F>(&self, f: F) where F: Fn(&ChunkPos) {
        for x in self.min_x()..=self.max_x() {
            for z in self.min_z()..=self.max_z() {
                let pos = ChunkPos::new(x, z);
                if self.contains(&pos, true) {
                    f(&pos);
                }
            }
        }
    }
}
