use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct MousePosition(pub Vec2);

pub struct InputPlugin;

mod systems;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MousePosition>()
            .add_systems(PreUpdate, systems::mouse_position);
    }
}
