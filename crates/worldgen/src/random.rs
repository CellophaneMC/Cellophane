pub trait Random {
    fn next_bits(&mut self, bits: usize) -> i32;

    fn next_i32(&mut self) -> i32 {
        self.next_bits(32)
    }

    fn next_f64(&mut self) -> f64 {
        let i = self.next_bits(26);
        let j = self.next_bits(27);
        let l = ((i as i64) << 27) + (j as i64);
        (l as f64) * DOUBLE_MULTIPLIER
    }

    fn next_bounded(&mut self, bound: usize) -> i32 {
        let mut i = self.next_bits(31);
        let bound = bound as i32;
        let m = bound - 1;
        if (bound & m) == 0 {
            i = ((bound as i64) * (i as i64) >> 31) as i32;
        } else {
            let mut j = i;
            i = j % bound;
            while j - i + m < 0 {
                j = self.next_bits(31);
                i = j % bound;
            }
        }
        i
    }

    fn set_seed(&mut self, seed: i64);
}

pub struct SimpleRandom {
    seed: i64,
}

const INT_BITS: usize = 48;
const SEED_MASK: i64 = 281474976710655i64;
const MULTIPLIER: i64 = 25214903917i64;
const INCREMENT: i64 = 11i64;
const DOUBLE_MULTIPLIER: f64 = 1.1102230246251565E-16;

impl Random for SimpleRandom {
    fn next_bits(&mut self, bits: usize) -> i32 {
        let l = self.seed.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT) & SEED_MASK;
        self.seed = l;
        (l >> INT_BITS - bits) as i32
    }

    fn set_seed(&mut self, seed: i64) {
        self.seed = (seed ^ MULTIPLIER) & SEED_MASK;
    }
}

impl SimpleRandom {
    pub fn new(seed: i64) -> Self {
        let mut random = Self { seed: 0 };
        random.set_seed(seed);
        random
    }
}
