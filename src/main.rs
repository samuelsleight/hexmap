use bevy::prelude::*;

use rand::{Rng, distr::Uniform};

use camera::CameraPlugin;
use input::InputPlugin;
use profiling::ProfilingPlugin;
use selection::SelectionPlugin;
use world::{OnHex, WorldLayout, WorldParams, WorldPlugin};

mod camera;
mod input;
mod profiling;
mod selection;
mod world;

#[derive(Default, Component)]
#[require(OnHex, Transform = Transform::from_xyz(0., 0., 1.))]
struct Indicator;

fn setup_world(mut commands: Commands) {
    // Request the world generation
    commands.insert_resource(WorldParams {
        width: 170,
        height: 100,
        scale_factor: 1.6,
    });
}

fn spawn_some_squares(
    mut commands: Commands,
    world: Res<WorldLayout>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(6., 6.));

    let colours = [
        materials.add(ColorMaterial::from_color(Color::srgb(1., 0., 0.))),
        materials.add(ColorMaterial::from_color(Color::srgb(1., 1., 0.))),
        materials.add(ColorMaterial::from_color(Color::srgb(1., 0., 1.))),
        materials.add(ColorMaterial::from_color(Color::srgb(0., 1., 1.))),
    ];

    let mut rng = rand::rng();
    let x_dist = Uniform::new(0, 170).unwrap();
    let y_dist = Uniform::new(0, 100).unwrap();

    for i in 0..25 {
        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(colours[i % colours.len()].clone()),
            OnHex(Some(world.hex(rng.sample(x_dist), rng.sample(y_dist)))),
        ));
    }
}

fn indicators(
    mut gizmos: Gizmos,
    world: Res<WorldLayout>,
    indicators: Query<(&GlobalTransform, &InheritedVisibility), With<Indicator>>,
) {
    for (indicator, visibility) in indicators {
        if visibility.get() {
            let transform = indicator.translation().xy();

            for [a, b] in world.edge_coordinates() {
                gizmos.line_2d(transform + a, transform + b, Color::srgb(0.5, 0.5, 0.5));
            }
        }
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_000.0, 1_000.0).into(),

                #[cfg(not(target_arch = "wasm32"))]
                present_mode: bevy::window::PresentMode::Immediate,

                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            ProfilingPlugin,
            WorldPlugin,
            CameraPlugin,
            InputPlugin,
            SelectionPlugin,
        ))
        .add_systems(Startup, setup_world)
        .add_systems(
            Update,
            (
                indicators.run_if(resource_exists::<WorldLayout>),
                spawn_some_squares.run_if(resource_added::<WorldLayout>),
            ),
        )
        .run();
}
