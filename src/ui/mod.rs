use bevy::prelude::*;

pub use settlement::SettlementUi;

mod settlement;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        settlement::register(app);
    }
}
