use bevy::{prelude::*, window::PrimaryWindow};

use hexx::Hex;
use world::{WorldLayout, WorldOrigin, WorldParams, WorldPlugin, WorldTiles};

mod world;

#[derive(Default, Resource)]
struct MousePosition(Vec2);

#[derive(Default, Resource)]
struct HoveredHex(Option<Hex>);

#[derive(Component)]
#[require(Transform = Transform::from_xyz(0., 0., 1.))]
struct HoverIndicator;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_world(mut commands: Commands) {
    commands.insert_resource(WorldParams {
        width: 150,
        height: 90,
    });
}

fn get_scale(projection: &mut Projection) -> &mut f32 {
    if let Projection::Orthographic(ortho) = projection {
        return &mut ortho.scale;
    }

    panic!("Unexpected projection")
}

fn centre_camera(world: Res<WorldLayout>, camera: Single<&mut Transform, With<Camera>>) {
    let bounds = world.world_size();
    let mut transform = camera.into_inner();
    transform.translation = (bounds / 2.).extend(0.);
}

fn zoom_viewport(
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

fn scroll_grid(
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

fn mouse_position(
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

fn mouse_hover(
    position: Res<MousePosition>,
    world: Res<WorldLayout>,
    mut hovered: ResMut<HoveredHex>,
    origin: Single<&Transform, With<WorldOrigin>>,
) {
    let origin = origin.into_inner();
    hovered.0 = Some(world.pick_tile(position.0, origin.translation.xy()));
}

fn hover_indicator(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    hovered: Res<HoveredHex>,
    world: Res<WorldLayout>,
    tiles: Res<WorldTiles>,
    mut indicator: Query<Entity, With<HoverIndicator>>,
) {
    if let Some(entity) = hovered.0.and_then(|hex| tiles.get(hex, &world)) {
        if let Ok(indicator) = indicator.single_mut() {
            commands.entity(entity).add_child(indicator);
        } else {
            commands.entity(entity).with_child((
                HoverIndicator,
                Mesh2d(meshes.add(Circle::new(5.))),
                MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))),
            ));
        }
    } else if let Ok(indicator) = indicator.single_mut() {
        commands.entity(indicator).despawn();
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldPlugin)
        .init_resource::<MousePosition>()
        .init_resource::<HoveredHex>()
        .add_systems(Startup, (setup_camera, setup_world))
        .add_systems(
            Update,
            (
                mouse_position.before(mouse_hover),
                (
                    centre_camera,
                    zoom_viewport,
                    scroll_grid,
                    (
                        mouse_hover.run_if(resource_changed::<MousePosition>),
                        hover_indicator.run_if(resource_changed::<HoveredHex>),
                    )
                        .chain(),
                )
                    .run_if(resource_exists::<WorldLayout>),
            ),
        )
        .run();
}
