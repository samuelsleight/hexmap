use noise::{Add, Fbm, MultiFractal, NoiseFn, Perlin, RidgedMulti, ScaleBias, Seedable};

pub struct NoiseParameters {
    seed: u32,
    height_extent: f64,
    continent_bounds: f64,
    erosion_power: f64,
}

impl NoiseParameters {
    pub fn new(seed: u32, height_extent: f64, continent_bounds: f64, erosion_power: f64) -> Self {
        Self {
            seed,
            height_extent,
            continent_bounds,
            erosion_power,
        }
    }
}

pub fn get_noise_fn(
    NoiseParameters {
        seed,
        height_extent,
        continent_bounds,
        erosion_power,
    }: NoiseParameters,
) -> impl NoiseFn<f64, 3> {
    let base = Fbm::<Perlin>::default()
        .set_seed(seed)
        .set_lacunarity(1.91010101)
        .set_persistence(0.40)
        .set_octaves(12);

    let ridged = RidgedMulti::<Perlin>::default()
        .set_seed(seed + 1)
        .set_frequency(0.9)
        .set_lacunarity(2.11010101)
        .set_persistence(0.60)
        .set_octaves(5);

    let added = Add::new(
        ScaleBias::new(base).set_scale(0.7),
        ScaleBias::new(ridged).set_scale(0.3),
    );

    ContinentWrapper {
        overall_extent: height_extent,
        bounds_extent: continent_bounds,
        erosion_power,
        source: added,
    }
}

struct ContinentWrapper<Source: NoiseFn<f64, 3>> {
    overall_extent: f64,
    bounds_extent: f64,
    erosion_power: f64,

    source: Source,
}

impl<T: NoiseFn<f64, 3>> NoiseFn<f64, 3> for ContinentWrapper<T> {
    fn get(&self, point: [f64; 3]) -> f64 {
        let inner = self.source.get(point);

        let y = point[1];
        let mut erosion = 0.;

        if y < self.bounds_extent {
            let depth = 1. - (y / self.bounds_extent);
            erosion = depth * self.erosion_power;
        } else if y > (self.overall_extent - self.bounds_extent) {
            let threshold = self.overall_extent - self.bounds_extent;
            let y = y - threshold;
            let depth = y / self.bounds_extent;
            erosion = depth * self.erosion_power;
        }

        inner - erosion
    }
}
