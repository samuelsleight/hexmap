use bevy::prelude::*;

use iyes_perf_ui::prelude::*;

#[cfg(feature = "profiling")]
fn setup_profiling(mut commands: Commands) {
    commands.spawn(PerfUiDefaultEntries::default());
}

#[cfg(not(feature = "profiling"))]
fn setup_profiling(mut commands: Commands) {
    commands.spawn(PerfUiRoot::default());
}

pub fn build(app: &mut App) {
    #[cfg(feature = "profiling")]
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin);

    app.add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup_profiling);
}
