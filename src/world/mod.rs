use bevy::prelude::*;

pub use self::types::*;

mod generation;
mod systems;
mod types;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            generation::generate_world.run_if(resource_exists::<WorldParams>),
        )
        .add_systems(
            PostUpdate,
            systems::wrap_grid.run_if(resource_exists::<WorldLayout>),
        );
    }
}
