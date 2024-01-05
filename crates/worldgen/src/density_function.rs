trait DensityFunction {
    fn min(&self) -> f64;
    fn max(&self) -> f64;

    fn compute(&self, x: f64, y: f64, z: f64) -> f64;
}

struct Noise {
    noise: crate::noise::NoiseParameters,
    xz_scale: f64,
    y_scale: f64,
}

impl DensityFunction for Noise {
    fn min() -> f64 {
        todo!()
    }

    fn max() -> f64 {
        todo!()
    }

    fn compute(&self, x: f64, y: f64, z: f64) -> f64 {
        todo!()
    }
}
