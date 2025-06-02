use hexx::{Hex, HexLayout, HexOrientation, OffsetHexMode};

#[derive(Debug, Clone)]
pub struct GeneratedWorld<T> {
    width: i32,
    height: i32,
    layout: HexLayout,
    tiles: Vec<T>,
}

impl<T> GeneratedWorld<T> {
    pub fn new(width: i32, height: i32, layout: HexLayout, tiles: Vec<T>) -> Self {
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

    pub fn tiles(&self) -> impl Iterator<Item = (Hex, &T)> {
        let height = self.height;

        let index_to_hex = move |index| {
            let x = (index as i32 / height) + 1;
            let mut y = (index as i32 % height) + 1;

            if x % 2 == 1 {
                y += 1
            }

            Hex::from_offset_coordinates([x, y], OffsetHexMode::Even, HexOrientation::Flat)
        };

        self.tiles
            .iter()
            .enumerate()
            .map(move |(index, tile)| (index_to_hex(index), tile))
    }
}
