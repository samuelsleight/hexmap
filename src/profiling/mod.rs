use bevy::prelude::*;

#[cfg(feature = "debug_ui")]
mod detail;

pub struct ProfilingPlugin;

impl Plugin for ProfilingPlugin {
    #[cfg(feature = "debug_ui")]
    fn build(&self, app: &mut App) {
        detail::build(app);
    }

    #[cfg(not(feature = "debug_ui"))]
    fn build(&self, _: &mut App) {}
}
