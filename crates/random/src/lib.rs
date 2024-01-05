// pub trait Random {
//     fn next_bits(&mut self, bits: usize) -> u32;
//
//     fn next_u32(&mut self) -> u32;
//
//     fn next_u64(&mut self) -> u64;
//
//     #[inline]
//     fn next_i64(&mut self) -> i64 {
//         self.next_u64() as i64
//     }
//
//     #[inline]
//     fn next_i32(&mut self) -> i32 {
//         self.next_u32() as i32
//     }
//
//     fn next_i32_bound(&mut self, bound: i32) -> i32;
//
//     fn next_f64(&mut self) -> f64;
//
//     fn set_seed(&mut self, seed: i64);
// }
//
// pub struct LegacyRandom {
//     seed: i64,
// }
//
// impl Random for LegacyRandom {
//     fn next_bits(&mut self, bits: usize) -> u32 {
//         let l = self.seed;
//         let m = l.wrapping_mul(25214903917).wrapping_add(11) & 0xFFFFFFFFFFFF;
//         self.seed = m;
//         (m >> 48 - bits) as u32
//     }
//
//     fn next_u32(&mut self) -> u32 {
//         self.next_bits(32)
//     }
//
//     fn next_u64(&mut self) -> u64 {
//         let i = self.next_bits(32) as i32;
//         let j = self.next_bits(32) as i32;
//         let l = (i as i64) << 32;
//         (l + (j as i64)) as u64
//     }
//
//     fn next_i32_bound(&mut self, bound: i32) -> i32 {
//         let mut j: i32;
//         let mut i: i32;
//
//         if (bound & (bound - 1)) == 0 {
//             // println!("bbb");
//             return (bound as i64 * self.next_bits(31) as i64 >> 31) as i32;
//         }
//
//         loop {
//             i = self.next_bits(31) as i32;
//             j = i % bound;
//
//             if i - j + (bound - 1) >= 0 {
//                 // println!("aaa");
//                 break j;
//             }
//         }
//     }
//
//     fn next_f64(&mut self) -> f64 {
//         let i = self.next_bits(26) as i32;
//         let j = self.next_bits(27) as i32;
//         let l = ((i as i64) << 27) + (j as i64);
//         (l as f64) * DOUBLE_MULTIPLIER
//     }
//
//     fn set_seed(&mut self, seed: i64) {
//         self.seed = (seed ^ 0x5DEECE66D) & 0xFFFFFFFFFFFF;
//     }
// }
//
// impl LegacyRandom {
//     pub fn new(seed: i64) -> Self {
//         let mut random = Self { seed };
//         random.set_seed(seed);
//         random
//     }
// }
//
// pub struct ChunkRandom {
//     base: LegacyRandom,
// }
//
// const DOUBLE_MULTIPLIER: f64 = 1.110223E-16f32 as f64;
//
// impl Random for ChunkRandom {
//     fn next_bits(&mut self, bits: usize) -> u32 {
//         self.base.next_bits(bits)
//     }
//
//     fn next_u32(&mut self) -> u32 {
//         self.base.next_u32()
//     }
//
//     fn next_u64(&mut self) -> u64 {
//         self.base.next_u64()
//     }
//
//     fn next_i32_bound(&mut self, bound: i32) -> i32 {
//         self.base.next_i32_bound(bound)
//     }
//
//     fn next_f64(&mut self) -> f64 {
//         self.base.next_f64()
//     }
//
//     fn set_seed(&mut self, seed: i64) {
//         self.base.set_seed(seed)
//     }
// }
//
// impl ChunkRandom {
//     fn set_population_seed(&mut self, world_seed: i64, block_x: i32, block_z: i32) -> i64 {
//         self.set_seed(world_seed);
//         let l = self.next_i64() | 1;
//         let m = self.next_i64() | 1;
//         let n = (block_x as i64) * l + (block_z as i64) * m ^ world_seed;
//         self.set_seed(n);
//         n
//     }
//
//     fn set_decorator_seed(&mut self, population_seed: i64, index: usize, step: usize) {
//         let l = population_seed + (index as i64) + ((10000 * step) as i64);
//         self.set_seed(l)
//     }
//
//     fn get_slime_random(chunk_x: i32, chunk_z: i32, world_seed: i64, scrambler: i64) -> LegacyRandom {
//         let seed = world_seed
//             + (chunk_x as i64 * chunk_x as i64 * 4987142)
//             + (chunk_x as i64 * 5947611)
//             + (chunk_z as i64 * chunk_z as i64 * 4392871)
//             + (chunk_z as i64 * 389711)
//             ^ scrambler;
//         LegacyRandom::new(seed)
//     }
// }
