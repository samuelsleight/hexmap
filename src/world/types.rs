use bevy::prelude::*;

use hexx::{Hex, HexLayout, HexOrientation, OffsetHexMode};

use crate::camera::{OverlayMode, RenderOrder, VisibilityFlags};

#[derive(Default, Component)]
#[require(Visibility, RenderOrder = RenderOrder::Overlay, OverlayMode = OverlayMode::Zone)]
pub struct ZoneHighlight;

#[derive(Default, Component)]
#[require(VisibilityFlags)]
pub struct OnHex(pub Option<Hex>);

#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WorldOrigin;

#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WorldColumn {
    pub column: i32,
}

#[derive(Resource)]
pub struct WorldLayout {
    pub layout: HexLayout,
    pub width: i32,
    pub height: i32,
}

#[derive(Resource)]
pub struct WorldParams {
    pub width: i32,
    pub height: i32,
    pub scale_factor: f64,
}

#[derive(Default, Resource)]
pub struct WorldTiles {
    pub tiles: Vec<Entity>,
}

impl WorldLayout {
    pub fn hex(&self, x: i32, y: i32) -> Hex {
        Hex::from_offset_coordinates([x, y], OffsetHexMode::Even, HexOrientation::Flat)
    }

    pub fn hex_to_xy(&self, hex: Hex) -> [i32; 2] {
        hex.to_offset_coordinates(OffsetHexMode::Even, HexOrientation::Flat)
    }

    pub fn wrap(&self, hex: Hex) -> Hex {
        let [mut x, y] = self.hex_to_xy(hex);

        if x <= 0 {
            x += self.width;
        } else if x > self.width {
            x -= self.width;
        }

        self.hex(x, y)
    }

    pub fn world_size(&self) -> Vec2 {
        self.layout
            .hex_to_world_pos(self.hex(self.width, self.height))
    }

    pub fn width(&self) -> f32 {
        self.world_size().x
    }

    pub fn world_pos_to_xy(&self, pos: Vec2) -> [i32; 2] {
        self.hex_to_xy(self.layout.world_pos_to_hex(pos))
    }

    pub fn xy_to_world_pos(&self, x: i32, y: i32) -> Vec2 {
        self.layout.hex_to_world_pos(self.hex(x, y))
    }

    pub fn pick_tile(&self, world_pos: Vec2, origin: Vec2) -> Hex {
        let mut hex = self.layout.world_pos_to_hex(world_pos - origin);
        hex.x -= 1;
        hex.y -= 1;

        hex
    }

    pub fn edge_coordinates(&self) -> [[Vec2; 2]; 6] {
        self.layout.all_edge_coordinates(self.hex(0, 0))
    }
}

impl WorldTiles {
    pub fn get(&self, hex: Hex, world: &WorldLayout) -> Option<Entity> {
        let [mut x, y] = world.hex_to_xy(hex);

        if x < 0 {
            x += world.width;
        } else if x >= world.width {
            x -= world.width;
        }

        let index = (x * world.height + y) as usize;

        if index >= self.tiles.len() {
            None
        } else {
            Some(self.tiles[index])
        }
    }
}
