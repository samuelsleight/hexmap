use bevy::prelude::*;

use camera::CameraPlugin;
use input::{InputPlugin, MousePosition};
use world::{OnHex, WorldLayout, WorldOrigin, WorldParams, WorldPlugin};

mod camera;
mod input;
mod world;

#[derive(Default, Component)]
#[require(OnHex, Transform = Transform::from_xyz(0., 0., 1.))]
struct Indicator;

#[derive(Default, Component)]
#[require(Indicator)]
struct HoverIndicator;

#[derive(Default, Component)]
#[require(Indicator)]
struct SelectionIndicator;

fn setup_world(mut commands: Commands) {
    // Request the world generation
    commands.insert_resource(WorldParams {
        width: 150,
        height: 90,
    });

    // Spawn some global indicators
    commands.spawn(HoverIndicator);
    commands.spawn(SelectionIndicator);
}

fn mouse_hover(
    position: Res<MousePosition>,
    world: Res<WorldLayout>,
    origin: Single<&Transform, With<WorldOrigin>>,
    indicator: Single<&mut OnHex, With<HoverIndicator>>,
) {
    let origin = origin.into_inner();
    indicator.into_inner().0 = Some(world.pick_tile(position.0, origin.translation.xy()));
}

fn mouse_press(
    mouse: Res<ButtonInput<MouseButton>>,
    hover: Single<&OnHex, With<HoverIndicator>>,
    select: Single<&mut OnHex, (With<SelectionIndicator>, Without<HoverIndicator>)>,
) {
    if mouse.just_released(MouseButton::Left) {
        let hovered = hover.into_inner();
        let mut current = select.into_inner();

        if current.0 != hovered.0 {
            current.0 = hovered.0;
        } else {
            current.0 = None;
        }
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
        .add_plugins(WorldPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(InputPlugin)
        .add_systems(Startup, setup_world)
        .add_systems(
            Update,
            ((
                mouse_hover.run_if(resource_changed::<MousePosition>),
                mouse_press,
                indicators,
            )
                .run_if(resource_exists::<WorldLayout>),),
        )
        .run();
}
