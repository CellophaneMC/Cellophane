use crate::random::Random;

pub struct PerlinNoiseSampler {
    pub(crate) origin_x: f64,
    pub(crate) origin_y: f64,
    pub(crate) origin_z: f64,
    pub(crate) permutation: [u8; 256],
}

impl PerlinNoiseSampler {
    pub fn gen(random: &mut impl Random) -> PerlinNoiseSampler {
        let origin_x = random.next_f64() * 256.0;
        let origin_y = random.next_f64() * 256.0;
        let origin_z = random.next_f64() * 256.0;
        let mut permutation = [0u8; 256];

        for i in 0..256 {
            permutation[i] = i as u8;
        }

        for i in 0..256 {
            let i = i as usize;
            let j = random.next_bounded(256 - i) as usize;
            permutation.swap(i, i + j);
        }

        PerlinNoiseSampler {
            origin_x,
            origin_y,
            origin_z,
            permutation,
        }
    }

    pub fn sample(
        &self,
        x: f64,
        y: f64,
        z: f64,
    ) -> f64 {
        let d = x + self.origin_x;
        let e = y + self.origin_y;
        let f = z + self.origin_z;
        let section_x = d.floor();
        let section_y = e.floor();
        let section_z = f.floor();
        let local_x = d - section_x;
        let local_y = e - section_y;
        let local_z = f - section_z;
        let section_x = section_x as usize;
        let section_y = section_y as usize;
        let section_z = section_z as usize;

        let var0 = section_x & 0xFF;
        let var1 = (section_x + 1) & 0xFF;
        let var2 = self.permutation[var0] as usize;
        let var3 = self.permutation[var1] as usize;
        let var4 = (var2 + section_y) & 0xFF;
        let var5 = (var3 + section_y) & 0xFF;
        let var6 = (var2 + section_y + 1) & 0xFF;
        let var7 = (var3 + section_y + 1) & 0xFF;
        let var8 = self.permutation[var4] as usize;
        let var9 = self.permutation[var5] as usize;
        let var10 = self.permutation[var6] as usize;
        let var11 = self.permutation[var7] as usize;

        let var12 = (var8 + section_z) & 0xFF;
        let var13 = (var9 + section_z) & 0xFF;
        let var14 = (var10 + section_z) & 0xFF;
        let var15 = (var11 + section_z) & 0xFF;
        let var16 = (var8 + section_z + 1) & 0xFF;
        let var17 = (var9 + section_z + 1) & 0xFF;
        let var18 = (var10 + section_z + 1) & 0xFF;
        let var19 = (var11 + section_z + 1) & 0xFF;
        let var20 = ((self.permutation[var12] & 15) << 2) as usize;
        let var21 = ((self.permutation[var13] & 15) << 2) as usize;
        let var22 = ((self.permutation[var14] & 15) << 2) as usize;
        let var23 = ((self.permutation[var15] & 15) << 2) as usize;
        let var24 = ((self.permutation[var16] & 15) << 2) as usize;
        let var25 = ((self.permutation[var17] & 15) << 2) as usize;
        let var26 = ((self.permutation[var18] & 15) << 2) as usize;
        let var27 = ((self.permutation[var19] & 15) << 2) as usize;
        let var60 = local_x - 1.0;
        let var61 = local_y - 1.0;
        let var62 = local_z - 1.0;
        let var87 = FLAT_SIMPLEX_GRAD[(var20) | 0] * local_x + FLAT_SIMPLEX_GRAD[(var20) | 1] * local_y + FLAT_SIMPLEX_GRAD[(var20) | 2] * local_z;
        let var88 = FLAT_SIMPLEX_GRAD[(var21) | 0] * var60 + FLAT_SIMPLEX_GRAD[(var21) | 1] * local_y + FLAT_SIMPLEX_GRAD[(var21) | 2] * local_z;
        let var89 = FLAT_SIMPLEX_GRAD[(var22) | 0] * local_x + FLAT_SIMPLEX_GRAD[(var22) | 1] * var61 + FLAT_SIMPLEX_GRAD[(var22) | 2] * local_z;
        let var90 = FLAT_SIMPLEX_GRAD[(var23) | 0] * var60 + FLAT_SIMPLEX_GRAD[(var23) | 1] * var61 + FLAT_SIMPLEX_GRAD[(var23) | 2] * local_z;
        let var91 = FLAT_SIMPLEX_GRAD[(var24) | 0] * local_x + FLAT_SIMPLEX_GRAD[(var24) | 1] * local_y + FLAT_SIMPLEX_GRAD[(var24) | 2] * var62;
        let var92 = FLAT_SIMPLEX_GRAD[(var25) | 0] * var60 + FLAT_SIMPLEX_GRAD[(var25) | 1] * local_y + FLAT_SIMPLEX_GRAD[(var25) | 2] * var62;
        let var93 = FLAT_SIMPLEX_GRAD[(var26) | 0] * local_x + FLAT_SIMPLEX_GRAD[(var26) | 1] * var61 + FLAT_SIMPLEX_GRAD[(var26) | 2] * var62;
        let var94 = FLAT_SIMPLEX_GRAD[(var27) | 0] * var60 + FLAT_SIMPLEX_GRAD[(var27) | 1] * var61 + FLAT_SIMPLEX_GRAD[(var27) | 2] * var62;

        let var95 = local_x * 6.0 - 15.0;
        let var96 = local_y * 6.0 - 15.0;
        let var97 = local_z * 6.0 - 15.0;
        let var98 = local_x * var95 + 10.0;
        let var99 = local_y * var96 + 10.0;
        let var100 = local_z * var97 + 10.0;
        let var101 = local_x * local_x * local_x * var98;
        let var102 = local_y * local_y * local_y * var99;
        let var103 = local_z * local_z * local_z * var100;

        let var113 = var87 + var101 * (var88 - var87);
        let var114 = var93 + var101 * (var94 - var93);
        let var115 = var91 + var101 * (var92 - var91);
        let var116 = var89 + var101 * (var90 - var89);
        let var117 = var114 - var115;
        let var118 = var102 * (var116 - var113);
        let var119 = var102 * var117;
        let var120 = var113 + var118;
        let var121 = var115 + var119;
        var120 + (var103 * (var121 - var120))
    }
}

const FLAT_SIMPLEX_GRAD: [f64; 64] = [
    1., 1., 0., 0.,
    -1., 1., 0., 0.,
    1., -1., 0., 0.,
    -1., -1., 0., 0.,
    1., 0., 1., 0.,
    -1., 0., 1., 0.,
    1., 0., -1., 0.,
    -1., 0., -1., 0.,
    0., 1., 1., 0.,
    0., -1., 1., 0.,
    0., 1., -1., 0.,
    0., -1., -1., 0.,
    1., 1., 0., 0.,
    0., -1., 1., 0.,
    -1., 1., 0., 0.,
    0., -1., -1., 0.,
];

#[cfg(test)]
mod tests {
    use crate::perlin::PerlinNoiseSampler;
    use crate::random::{Random, SimpleRandom};

    use super::*;

    #[test]
    fn it_works() {
        let mut random = SimpleRandom::new(42);
        let perlin = PerlinNoiseSampler::gen(&mut random);
        assert_eq!(-0.4810989641932585, perlin.sample(random.next_f64(), random.next_f64(), random.next_f64()));
    }
}
