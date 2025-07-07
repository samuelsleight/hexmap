use bevy::{app::MainScheduleOrder, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::csv::CsvAssetPlugin;

use crate::{
    AppState,
    world::names::{SettlementName, SettlementNameAssets},
};

pub use self::types::*;

mod generation;
mod names;
mod systems;
mod types;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(systems::GridUpdate);

        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_before(PostUpdate, systems::GridUpdate);

        app.add_plugins(CsvAssetPlugin::<SettlementName>::new(&["csv"]))
            .configure_loading_state(
                LoadingStateConfig::new(AppState::Loading)
                    .load_collection::<SettlementNameAssets>(),
            );

        app.add_systems(
            Update,
            generation::generate_world.run_if(resource_exists::<WorldParams>),
        )
        .add_systems(
            systems::GridUpdate,
            (systems::parent_grid_objects, systems::wrap_grid)
                .run_if(resource_exists::<WorldLayout>),
        );
    }
}
