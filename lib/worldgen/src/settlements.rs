use std::collections::HashMap;

use fast_poisson::Poisson2D;
use hexx::{Hex, HexOrientation, OffsetHexMode};
use noise::{Fbm, MultiFractal, Perlin, ScaleBias, Seedable};

use crate::{cylinder::CylindricalHexMapSampler, terrain::GeneratedTerrain};

#[derive(Debug, Clone, Copy)]
pub struct SettlementParams {
    seed: u32,
}

impl SettlementParams {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }
}

pub fn generate(
    terrain: &GeneratedTerrain,
    SettlementParams { seed }: SettlementParams,
) -> impl Iterator<Item = Hex> {
    let noise =
        ScaleBias::new(Fbm::<Perlin>::default().set_seed(seed).set_frequency(2.)).set_bias(0.5);

    let width = terrain.width();
    let height = terrain.height();

    let hex_fn = move |x: i32, y: i32| {
        let x = x.clamp(0, width) + 1;
        let mut y = y.clamp(0, height) + 1;

        if x % 2 == 1 {
            y += 1
        }

        Hex::from_offset_coordinates([x, y], OffsetHexMode::Even, HexOrientation::Flat)
    };

    let habitability = terrain
        .tiles()
        .map(|(hex, tile)| (hex, tile.is_habitable()))
        .collect::<HashMap<_, _>>();

    let sampler = CylindricalHexMapSampler::new(width, height, 4., terrain.layout().clone());

    Poisson2D::new()
        .with_dimensions([width as f64, height as f64], move |[x, y]: [f64; 2]| {
            let min_radius = 2.;
            let max_radius = 10.;

            let hex = hex_fn(x as i32, y as i32);
            let [x, y] = hex.to_offset_coordinates(OffsetHexMode::Even, HexOrientation::Flat);

            if habitability.get(&hex).is_some_and(|pair| *pair) {
                let value = sampler.sample_xy(x, y, &noise).clamp(0., 1.);
                Some((value * (max_radius - min_radius)) + min_radius)
            } else {
                None
            }
        })
        .with_wrapping([true, false])
        .with_seed(seed as u64 + 1)
        .into_iter()
        .map(move |[x, y]| hex_fn(x as i32, y as i32))
}
