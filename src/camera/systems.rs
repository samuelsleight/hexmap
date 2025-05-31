use bevy::prelude::*;

use crate::world::{WorldLayout, WorldOrigin};

fn get_scale(projection: &mut Projection) -> &mut f32 {
    if let Projection::Orthographic(ortho) = projection {
        return &mut ortho.scale;
    }

    panic!("Unexpected projection")
}

pub fn centre_camera(world: Res<WorldLayout>, camera: Single<&mut Transform, With<Camera>>) {
    let bounds = world.world_size();
    let mut transform = camera.into_inner();
    transform.translation = (bounds / 2.).extend(0.);
}

pub fn zoom_viewport(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera: Single<&mut Projection, With<Camera>>,
) {
    let mut projection = camera.into_inner();
    let scale = get_scale(&mut projection);

    let speed = 2. * time.delta_secs();

    let (min_scale, max_scale) = (0.1, 1.5);

    if keyboard_input.pressed(KeyCode::KeyQ) {
        *scale = f32::max(min_scale, *scale - speed);
    }

    if keyboard_input.pressed(KeyCode::KeyZ) {
        *scale = f32::min(max_scale, *scale + speed);
    }
}

pub fn scroll_grid(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    origin: Single<&mut Transform, With<WorldOrigin>>,
) {
    let mut transform = origin.into_inner();

    let speed = 200. * time.delta_secs();

    if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        transform.translation.y -= speed;
    }

    if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        transform.translation.y += speed;
    }

    if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        transform.translation.x -= speed;
    }

    if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        transform.translation.x += speed;
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
