use bevy::{prelude::*, render::pipelined_rendering::PipelinedRenderingPlugin};

use hexx::Hex;

use camera::CameraPlugin;
use input::{InputPlugin, MousePosition};
use world::{WorldLayout, WorldOrigin, WorldParams, WorldPlugin, WorldTiles};

mod camera;
mod input;
mod world;

#[derive(Default, Resource)]
struct HoveredHex(Option<Hex>);

#[derive(Component)]
#[require(Transform = Transform::from_xyz(0., 0., 1.))]
struct HoverIndicator;

fn setup_world(mut commands: Commands) {
    commands.insert_resource(WorldParams {
        width: 150,
        height: 90,
    });
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

fn place_hover_indicator(
    mut commands: Commands,
    hovered: Res<HoveredHex>,
    world: Res<WorldLayout>,
    tiles: Res<WorldTiles>,
    mut indicator: Query<Entity, With<HoverIndicator>>,
) {
    if let Some(entity) = hovered.0.and_then(|hex| tiles.get(hex, &world)) {
        if let Ok(indicator) = indicator.single_mut() {
            commands.entity(entity).add_child(indicator);
        } else {
            commands.entity(entity).with_child(HoverIndicator);
        }
    } else if let Ok(indicator) = indicator.single_mut() {
        commands.entity(indicator).despawn();
    }
}

fn hover_indicator(
    mut gizmos: Gizmos,
    world: Res<WorldLayout>,
    indicator: Single<&GlobalTransform, With<HoverIndicator>>,
) {
    let transform = indicator.into_inner().translation().xy();

    for [a, b] in world.edge_coordinates() {
        gizmos.line_2d(transform + a, transform + b, Color::srgb(0.5, 0.5, 0.5));
    }
}

pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1_000.0, 1_000.0).into(),
                        ..default()
                    }),
                    ..default()
                })
                .build()
                .disable::<PipelinedRenderingPlugin>(),
        )
        .add_plugins(WorldPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(InputPlugin)
        .init_resource::<HoveredHex>()
        .add_systems(Startup, setup_world)
        .add_systems(
            Update,
            ((
                (
                    mouse_hover.run_if(resource_changed::<MousePosition>),
                    place_hover_indicator.run_if(resource_changed::<HoveredHex>),
                )
                    .chain(),
                hover_indicator,
            )
                .run_if(resource_exists::<WorldLayout>),),
        )
        .run();
}
