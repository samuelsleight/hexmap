use bevy::prelude::*;

use crate::{input::MousePosition, world::WorldLayout};

pub use self::types::*;

mod systems;
mod types;

#[cfg(feature = "debug_ui")]
mod debug;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::setup_indicators)
            .add_systems(
                Update,
                (
                    systems::mouse_hover.run_if(resource_changed::<MousePosition>),
                    systems::mouse_press,
                )
                    .run_if(resource_exists::<WorldLayout>),
            );

        #[cfg(feature = "debug_ui")]
        app.add_plugins(debug::DebugPlugin);
    }
}
