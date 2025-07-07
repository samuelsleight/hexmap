use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::csv::LoadedCsv;
use serde::Deserialize;

#[derive(AssetCollection, Resource)]
pub struct SettlementNameAssets {
    #[asset(path = "uk_bua.csv")]
    pub names: Handle<LoadedCsv<SettlementName>>,
}

#[derive(Deserialize, Asset, TypePath)]
pub struct SettlementName {
    pub name: String,
}
