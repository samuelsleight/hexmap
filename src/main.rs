use bevy::{input::common_conditions::input_just_released, prelude::*, window::PresentMode};

#[cfg(feature = "remote")]
use bevy::remote::{RemotePlugin, http::RemoteHttpPlugin};

use camera::CameraPlugin;
use input::InputPlugin;
use profiling::ProfilingPlugin;
use selection::SelectionPlugin;
use world::{OnHex, WorldLayout, WorldOrigin, WorldParams, WorldPlugin, WorldTiles};

use crate::camera::{CurrentOverlay, OverlayMode};

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

fn regenerate_world(mut commands: Commands, grid: Single<Entity, With<WorldOrigin>>) {
    commands.remove_resource::<WorldLayout>();
    commands.remove_resource::<WorldTiles>();
    commands.entity(grid.into_inner()).despawn();
    setup_world(commands);
}

fn get_scale(projection: &Projection) -> f32 {
    if let Projection::Orthographic(ortho) = projection {
        return ortho.scale;
    }

    panic!("Unexpected projection")
}

fn setup_gizmos(mut config: ResMut<GizmoConfigStore>, camera: Single<&Projection, With<Camera>>) {
    let projection = camera.into_inner();
    let scale = get_scale(&projection);

    let (indicator_config, _) = config.config_mut::<IndicatorGizmos>();
    indicator_config.line.width = 1. / scale;
    indicator_config.line.joints = GizmoLineJoint::Miter;
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct IndicatorGizmos;

fn indicators(
    mut gizmos: Gizmos<IndicatorGizmos>,
    world: Res<WorldLayout>,
    indicators: Query<(&GlobalTransform, &InheritedVisibility), With<Indicator>>,
) {
    for (indicator, visibility) in indicators {
        if visibility.get() {
            let transform = indicator.translation().xy();

            gizmos.linestrip_2d(
                world
                    .edge_coordinates()
                    .into_iter()
                    .map(|[a, _]| a)
                    .chain(world.edge_coordinates().into_iter().take(2).map(|[a, _]| a))
                    .map(|v| v + transform),
                Color::srgb(0.5, 0.5, 0.5),
            );
        }
    }
}

fn mode_toggle(keyboard_input: Res<ButtonInput<KeyCode>>, mut mode: ResMut<CurrentOverlay>) {
    if keyboard_input.just_released(KeyCode::Digit1) {
        mode.0 = OverlayMode::None
    } else if keyboard_input.just_released(KeyCode::Digit2) {
        mode.0 = OverlayMode::Zone
    }
}

pub fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (1_000.0, 1_000.0).into(),
            fit_canvas_to_parent: true,
            present_mode: PresentMode::AutoNoVsync,

            ..default()
        }),
        ..default()
    }));

    #[cfg(feature = "remote")]
    app.add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default());

    app.add_plugins((
        ProfilingPlugin,
        WorldPlugin,
        CameraPlugin,
        InputPlugin,
        SelectionPlugin,
    ))
    .init_gizmo_group::<IndicatorGizmos>()
    .add_systems(Startup, setup_world)
    .add_systems(
        Update,
        (setup_gizmos, mode_toggle).run_if(resource_exists::<WorldLayout>),
    )
    .add_systems(
        PostUpdate,
        (
            regenerate_world.run_if(input_just_released(KeyCode::Space)),
            indicators.run_if(resource_exists::<WorldLayout>),
        ),
    )
    .run();
}
