use std::marker::PhantomData;

use crate::AppState;

use bevy::{
    ecs::system::{
        SystemParam,
        lifetimeless::{SQuery, SRes},
    },
    prelude::*,
};

use iyes_perf_ui::{PerfUiAppExt, entry::PerfUiEntry, prelude::PerfUiRoot};

use crate::{
    selection::{HoverIndicator, SelectionIndicator},
    world::{OnHex, WorldLayout},
};

pub struct DebugPlugin;

#[derive(Component, Default)]
struct DebugTile<T>(PhantomData<T>);

fn setup_debug(query: Single<Entity, With<PerfUiRoot>>, mut commands: Commands) {
    commands.entity(query.into_inner()).insert((
        DebugTile::<HoverIndicator>::default(),
        DebugTile::<SelectionIndicator>::default(),
    ));
}

trait DebugLabel {
    fn label() -> &'static str;
}

impl DebugLabel for HoverIndicator {
    fn label() -> &'static str {
        "Hovered Tile"
    }
}

impl DebugLabel for SelectionIndicator {
    fn label() -> &'static str {
        "Selected Tile"
    }
}

impl<T: Component + DebugLabel> PerfUiEntry for DebugTile<T> {
    type SystemParam = (SQuery<&'static OnHex, With<T>>, SRes<WorldLayout>);

    type Value = ((i32, i32), [i32; 2]);

    fn label(&self) -> &str {
        T::label()
    }

    fn sort_key(&self) -> i32 {
        -1
    }

    fn update_value(
        &self,
        (hex, world): &mut <Self::SystemParam as SystemParam>::Item<'_, '_>,
    ) -> Option<Self::Value> {
        hex.single()
            .ok()
            .and_then(|hex| hex.0)
            .map(|hex| ((hex.x, hex.y), world.hex_to_xy(hex)))
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_perf_ui_simple_entry::<DebugTile<HoverIndicator>>()
            .add_perf_ui_simple_entry::<DebugTile<SelectionIndicator>>()
            .add_systems(OnEnter(AppState::Main), setup_debug);
    }
}
