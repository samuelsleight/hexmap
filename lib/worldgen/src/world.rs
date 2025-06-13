use std::collections::HashMap;

use fast_poisson::Poisson2D;
use hexx::{Hex, HexLayout, HexOrientation, OffsetHexMode};
use noise::{Fbm, MultiFractal, NoiseFn, Perlin, ScaleBias, Seedable};
use rand::{Rng, rng};

#[derive(Debug, Clone, Copy)]
pub enum TerrainType {
    DeepOcean,
    ShallowOcean,
    Coast,
    Beach,
    Plains,
    Hills,
    LowMountains,
    HighMountains,
    Peaks,
}

#[derive(Debug, Clone)]
pub struct GeneratedWorld {
    width: i32,
    height: i32,
    layout: HexLayout,
    tiles: Vec<TerrainType>,
}

impl TerrainType {
    fn is_habitable(&self) -> bool {
        match self {
            TerrainType::DeepOcean => false,
            TerrainType::ShallowOcean => false,
            TerrainType::Coast => false,
            TerrainType::Beach => false,
            TerrainType::Plains => true,
            TerrainType::Hills => true,
            TerrainType::LowMountains => false,
            TerrainType::HighMountains => false,
            TerrainType::Peaks => false,
        }
    }
}

impl GeneratedWorld {
    pub fn new(width: i32, height: i32, layout: HexLayout, tiles: Vec<TerrainType>) -> Self {
        Self {
            width,
            height,
            layout,
            tiles,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn layout(&self) -> &HexLayout {
        &self.layout
    }

    fn index_to_hex(height: i32, index: usize) -> Hex {
        let x = (index as i32 / height) + 1;
        let mut y = (index as i32 % height) + 1;

        if x % 2 == 1 {
            y += 1
        }

        Hex::from_offset_coordinates([x, y], OffsetHexMode::Even, HexOrientation::Flat)
    }

    pub fn tiles(&self) -> impl Iterator<Item = (Hex, TerrainType)> {
        let height = self.height;

        self.tiles
            .iter()
            .enumerate()
            .map(move |(index, tile)| (Self::index_to_hex(height, index), *tile))
    }

    pub fn generate_settlements(&self) -> impl Iterator<Item = Hex> {
        let noise = ScaleBias::new(
            Fbm::<Perlin>::default()
                .set_seed(rng().random())
                .set_persistence(0.50)
                .set_frequency(3.0)
                .set_octaves(8),
        )
        .set_bias(0.5);

        let width = self.width;
        let height = self.height;

        let hex_fn = move |x: i32, y: i32| {
            let x = x.clamp(0, width) + 1;
            let mut y = y.clamp(0, height) + 1;

            if x % 2 == 1 {
                y += 1
            }

            Hex::from_offset_coordinates([x, y], OffsetHexMode::Even, HexOrientation::Flat)
        };

        let habitability = self
            .tiles()
            .map(|(hex, tile)| (hex, tile.is_habitable()))
            .collect::<HashMap<_, _>>();

        Poisson2D::new()
            .with_dimensions([width as f64, height as f64], move |[x, y]: [f64; 2]| {
                let min_radius = 3.5;
                let max_radius = 10.;

                let hex = hex_fn(x as i32, y as i32) + Hex::new(1, 1);

                let x = x / width as f64;
                let y = y / height as f64;

                if habitability.get(&hex).is_some_and(|pair| *pair) {
                    let value = noise.get([x, y]).clamp(0., 1.);
                    Some((value * (max_radius - min_radius)) + min_radius)
                } else {
                    None
                }
            })
            .with_wrapping([true, false])
            .with_seed(rng().random())
            .into_iter()
            .map(move |[x, y]| hex_fn(x as i32, y as i32))
    }
}
