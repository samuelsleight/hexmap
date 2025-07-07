use bevy::{
    asset::AssetMetaCheck, input::common_conditions::input_just_released, prelude::*,
    window::PresentMode,
};

#[cfg(feature = "remote")]
use bevy::remote::{RemotePlugin, http::RemoteHttpPlugin};

use bevy_asset_loader::prelude::*;

use camera::{CameraPlugin, CurrentOverlay, OverlayMode};
use input::InputPlugin;
use profiling::ProfilingPlugin;
use selection::SelectionPlugin;
use ui::UiPlugin;
use world::{WorldLayout, WorldOrigin, WorldParams, WorldPlugin, WorldTiles};

mod camera;
mod input;
mod profiling;
mod selection;
mod ui;
mod world;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Main,
}

fn setup_world(mut commands: Commands) {
    // Request the world generation
    commands.insert_resource(WorldParams {
        width: 170,
        height: 100,
        scale_factor: 1.2,
    });
}

fn regenerate_world(mut commands: Commands, grid: Single<Entity, With<WorldOrigin>>) {
    // Cleanup the previous world
    commands.remove_resource::<WorldLayout>();
    commands.remove_resource::<WorldTiles>();
    commands.entity(grid.into_inner()).despawn();

    // Reset the current overlay
    commands.insert_resource(CurrentOverlay::default());

    // Re-request a world
    setup_world(commands);
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

    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..Default::default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Hexmap".into(),
                    resolution: (1_000.0, 1_000.0).into(),
                    fit_canvas_to_parent: true,
                    present_mode: PresentMode::AutoNoVsync,

                    ..default()
                }),
                ..default()
            }),
    );

    #[cfg(feature = "remote")]
    app.add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default());

    app.init_state::<AppState>()
        .add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::Main));

    app.add_plugins((
        ProfilingPlugin,
        WorldPlugin,
        CameraPlugin,
        InputPlugin,
        SelectionPlugin,
        UiPlugin,
    ))
    .add_systems(OnEnter(AppState::Main), setup_world)
    .add_systems(Update, mode_toggle.run_if(resource_exists::<WorldLayout>))
    .add_systems(
        PostUpdate,
        regenerate_world.run_if(input_just_released(KeyCode::Space)),
    )
    .run();
}
