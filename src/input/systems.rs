use bevy::{prelude::*, window::PrimaryWindow};

use super::MousePosition;

pub fn mouse_position(
    mut position: ResMut<MousePosition>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    let window = window.into_inner();
    let (camera, transform) = camera.into_inner();

    if let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        position.0 = cursor_position;
    }
}
