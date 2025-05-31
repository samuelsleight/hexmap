use bevy::prelude::*;

use hexx::{Hex, HexLayout, HexOrientation, OffsetHexMode};

mod generation;

#[derive(Component)]
#[require(InheritedVisibility, Transform)]
pub struct WorldOrigin;

#[derive(Component)]
#[require(InheritedVisibility, Transform)]
struct WorldColumn {
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
}

#[derive(Default, Resource)]
pub struct WorldTiles {
    pub tiles: Vec<Entity>,
}

pub struct WorldPlugin;

impl WorldLayout {
    fn hex(&self, x: i32, y: i32) -> Hex {
        Hex::from_offset_coordinates([x, y], OffsetHexMode::Even, HexOrientation::Flat)
    }

    fn hex_to_xy(&self, hex: Hex) -> [i32; 2] {
        hex.to_offset_coordinates(OffsetHexMode::Even, HexOrientation::Flat)
    }

    pub fn world_size(&self) -> Vec2 {
        self.layout
            .hex_to_world_pos(self.hex(self.width, self.height))
    }

    fn width(&self) -> f32 {
        self.world_size().x
    }

    pub fn world_pos_to_xy(&self, pos: Vec2) -> [i32; 2] {
        self.hex_to_xy(self.layout.world_pos_to_hex(pos))
    }

    fn xy_to_world_pos(&self, x: i32, y: i32) -> Vec2 {
        self.layout.hex_to_world_pos(self.hex(x, y))
    }

    pub fn pick_tile(&self, world_pos: Vec2, origin: Vec2) -> Hex {
        let mut hex = self.layout.world_pos_to_hex(world_pos - origin);
        hex.x -= 1;
        hex.y -= 1;

        hex
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

fn wrap_grid(
    world: Res<WorldLayout>,
    origin: Single<&mut Transform, (With<WorldOrigin>, Changed<Transform>)>,
    columns: Query<(&WorldColumn, &mut Transform), Without<WorldOrigin>>,
) {
    let mut origin = origin.into_inner();

    // Compute the width of the world in pixel coordinates
    let world_width = world.width();

    // Ensure the grid offset is between 0 and the width of the world
    if origin.translation.x < 0. {
        origin.translation.x += world_width
    }

    if origin.translation.x > world_width {
        origin.translation.x -= world_width
    }

    // Compute the distance in hexes between the grid offset and the right edge of the world
    let hex_offset = world.width - world.world_pos_to_xy(origin.translation.xy())[0];

    // Wrap any column past the right edge of the world over to the left
    for (column, mut transform) in columns {
        let wrapped_column = if column.column > hex_offset {
            column.column - world.width
        } else {
            column.column
        };

        transform.translation.x = world.xy_to_world_pos(wrapped_column, 0).x;
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            generation::generate_world.run_if(resource_exists::<WorldParams>),
        )
        .add_systems(PostUpdate, wrap_grid.run_if(resource_exists::<WorldLayout>));
    }
}
