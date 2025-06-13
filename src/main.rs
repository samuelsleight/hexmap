use bevy::prelude::*;

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
        //scale_factor: 1.6,
        scale_factor: 1.2,
    });
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
        .add_systems(Update, indicators.run_if(resource_exists::<WorldLayout>))
        .run();
}
