use std::f64::consts::PI;

use ::noise::NoiseFn;
use hexx::{HexLayout, HexOrientation, OffsetHexMode, shapes::flat_rectangle};
use rand::{Rng, rng};

use self::noise::NoiseParameters;

pub use self::world::{GeneratedTerrain, TerrainType};

mod noise;
mod world;

#[derive(Debug, Clone, Copy)]
pub struct TerrainParams {
    width: i32,
    height: i32,
    scale_factor: f64,
}

impl TerrainParams {
    pub fn new(width: i32, height: i32, scale_factor: f64) -> Self {
        Self {
            width,
            height,
            scale_factor,
        }
    }
}

fn get_terrain(value: f64) -> TerrainType {
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

pub fn generate(
    TerrainParams {
        width,
        height,
        scale_factor,
    }: TerrainParams,
) -> GeneratedTerrain {
    let layout = HexLayout::flat().with_hex_size(2.);
    let hex_rect = layout.rect_size();

    let scale = 1. / scale_factor;

    let angle_extent = 360.0;
    let height_extent = (2. * PI)
        * scale
        * (height as f64 / width as f64)
        * (hex_rect.x as f64 / hex_rect.y as f64);

    let x_step = angle_extent / width as f64;
    let y_step = height_extent / height as f64;

    let noise = noise::get_noise_fn(NoiseParameters::new(
        rng().random(),
        height_extent,
        y_step * 15.,
        0.6,
    ));

    let vec = flat_rectangle([1, width, 1, height])
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
            get_terrain(value)
        })
        .collect();

    GeneratedTerrain::new(width, height, layout, vec)
}
