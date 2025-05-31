use bevy::prelude::*;

use crate::world::WorldLayout;

mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_camera).add_systems(
            Update,
            (
                systems::centre_camera,
                systems::zoom_viewport,
                systems::scroll_grid,
            )
                .run_if(resource_exists::<WorldLayout>),
        );
    }
}
