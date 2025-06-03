use bevy::prelude::*;

use iyes_perf_ui::prelude::*;

fn setup_profiling(mut commands: Commands) {
    commands.spawn(PerfUiDefaultEntries::default());
}

pub fn build(app: &mut App) {
    app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup_profiling);
}
