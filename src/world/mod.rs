use bevy::{app::MainScheduleOrder, prelude::*};

pub use self::types::*;

mod generation;
mod systems;
mod types;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(systems::GridUpdate);

        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_before(PostUpdate, systems::GridUpdate);

        app.add_systems(
            Update,
            generation::generate_world.run_if(resource_exists::<WorldParams>),
        )
        .add_systems(
            systems::GridUpdate,
            (systems::parent_grid_objects, systems::wrap_grid)
                .run_if(resource_exists::<WorldLayout>),
        );
    }
}
