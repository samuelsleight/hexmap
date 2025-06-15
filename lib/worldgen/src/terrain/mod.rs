use hexx::HexLayout;
use rand::{Rng, rng};

use crate::cylinder::CylindricalHexMapSampler;

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
    let sampler = CylindricalHexMapSampler::new(width, height, scale_factor, layout.clone());

    let noise = noise::get_noise_fn(NoiseParameters::new(
        rng().random(),
        sampler.height_extent(),
        sampler.y_step() * 15.,
        0.6,
    ));

    GeneratedTerrain::new(
        width,
        height,
        layout,
        sampler.generate(noise).map(get_terrain).collect(),
    )
}
