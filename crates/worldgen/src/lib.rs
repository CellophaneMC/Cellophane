mod perlin;
mod random;
mod noise;
mod density_function;
mod octave_perlin;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::perlin::PerlinNoiseSampler;
    use crate::random::{Random, SimpleRandom};

    #[test]
    fn it_works() {
        // println!("{}", (42i64 ^ 25214903917i64) & 281474976710655i64)


        let mut random = SimpleRandom::new(42);
        //
        let perlin = PerlinNoiseSampler::gen(&mut random);
        //
        println!("{}", perlin.sample(random.next_f64(), random.next_f64(), random.next_f64()));
    }
}
