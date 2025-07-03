use bevy::prelude::*;

use crate::world::WorldLayout;

mod systems;
mod types;

pub use types::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_camera)
            .add_systems(
                Update,
                (
                    systems::centre_camera,
                    systems::zoom_viewport,
                    systems::scroll_grid,
                    systems::render_order,
                    systems::handle_overlay_visibility,
                )
                    .run_if(resource_exists::<WorldLayout>),
            )
            .add_systems(PostUpdate, systems::handle_visibility_flags);
    }
}
