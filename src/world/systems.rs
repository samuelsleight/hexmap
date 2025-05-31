use bevy::prelude::*;

use super::{WorldColumn, WorldLayout, WorldOrigin};

pub fn wrap_grid(
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
