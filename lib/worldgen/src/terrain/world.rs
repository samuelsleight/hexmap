use hexx::{Hex, HexLayout, HexOrientation, OffsetHexMode};

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
pub struct GeneratedTerrain {
    width: i32,
    height: i32,
    layout: HexLayout,
    tiles: Vec<TerrainType>,
}

impl TerrainType {
    pub fn is_habitable(&self) -> bool {
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

impl GeneratedTerrain {
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
}
