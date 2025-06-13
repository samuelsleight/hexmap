use std::f64::consts::PI;

use hexx::{HexLayout, HexOrientation, OffsetHexMode, shapes::flat_rectangle};
use noise::{Add, Fbm, MultiFractal, NoiseFn, Perlin, RidgedMulti, ScaleBias, Seedable};
use rand::{Rng, rng};

pub use world::{GeneratedWorld, TerrainType};

mod world;

#[derive(Debug, Clone, Copy)]
pub struct WorldParams {
    pub width: i32,
    pub height: i32,
    pub scale_factor: f64,
}

struct ContinentWrapper<Source: NoiseFn<f64, 3>> {
    overall_extent: f64,
    bounds_extent: f64,
    erosion_power: f64,

    source: Source,
}

impl WorldParams {
    fn get_terrain(&self, value: f64) -> TerrainType {
        let to_range = |value| (value / 1.67) - 0.4;

        let value = to_range(value + 1.);

        let ocean_percentage = 0.44;
        let mountain_percentage = 0.32;

        let land_threshold = to_range(2. * ocean_percentage);
        let mountain_threshold = to_range(2. * (1. - mountain_percentage));

        if value <= land_threshold {
            let percentage = value / land_threshold;

            if percentage < 0.3 {
                TerrainType::DeepOcean
            } else if percentage < 0.8 {
                TerrainType::ShallowOcean
            } else {
                TerrainType::Coast
            }
        } else if value <= mountain_threshold {
            let percentage = (value - land_threshold) / (mountain_threshold - land_threshold);

            if percentage < 0.085 {
                TerrainType::Beach
            } else if percentage < 0.6 {
                TerrainType::Plains
            } else {
                TerrainType::Hills
            }
        } else {
            let percentage = (value - mountain_threshold) / (0.6 - mountain_threshold);

            if percentage < 0.25 {
                TerrainType::LowMountains
            } else if percentage < 0.6 {
                TerrainType::HighMountains
            } else {
                TerrainType::Peaks
            }
        }
    }
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

fn get_noise_fn(
    height_extent: f64,
    continent_bounds: f64,
    erosion_power: f64,
) -> impl NoiseFn<f64, 3> {
    let seed = rng().random();

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

pub fn generate_world(params: &WorldParams) -> GeneratedWorld {
    let layout = HexLayout::flat().with_hex_size(2.);
    let hex_rect = layout.rect_size();

    let scale = 1. / params.scale_factor;

    let angle_extent = 360.0;
    let height_extent = (2. * PI)
        * scale
        * (params.height as f64 / params.width as f64)
        * (hex_rect.x as f64 / hex_rect.y as f64);

    let x_step = angle_extent / params.width as f64;
    let y_step = height_extent / params.height as f64;

    let noise = get_noise_fn(height_extent, y_step * 15., 0.6);

    let vec = flat_rectangle([1, params.width, 1, params.height])
        .map(|hex| {
            let [x, y] = hex.to_offset_coordinates(OffsetHexMode::Even, HexOrientation::Flat);

            let mut current_height = y_step * y as f64;
            let current_angle = x_step * x as f64;

            if x % 2 == 0 {
                current_height += y_step * 0.5;
            }

            let point_x = current_angle.to_radians().cos() * scale;
            let point_z = current_angle.to_radians().sin() * scale;

            let value = noise.get([point_x, current_height, point_z]);
            params.get_terrain(value)
        })
        .collect();

    GeneratedWorld::new(params.width, params.height, layout, vec)
}
