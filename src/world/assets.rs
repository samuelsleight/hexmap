use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::csv::LoadedCsv;
use serde::Deserialize;

#[derive(AssetCollection, Resource)]
pub struct WorldGenerationAssets {
    #[asset(path = "settlement_names.csv")]
    names: Handle<LoadedCsv<SettlementName>>,
}

#[derive(Deserialize, Asset, TypePath, Clone)]
pub struct SettlementName {
    pub name: String,
}

#[derive(Resource, Clone)]
pub struct SettlementNames(pub Vec<SettlementName>);

impl FromWorld for SettlementNames {
    fn from_world(world: &mut World) -> Self {
        let names = world
            .resource::<Assets<LoadedCsv<SettlementName>>>()
            .get(world.resource::<WorldGenerationAssets>().names.id())
            .unwrap()
            .rows
            .clone();

        SettlementNames(names)
    }
}
