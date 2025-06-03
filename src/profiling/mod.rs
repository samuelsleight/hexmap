use bevy::prelude::*;

#[cfg(feature = "profiling")]
mod detail;

pub struct ProfilingPlugin;

impl Plugin for ProfilingPlugin {
    #[cfg(feature = "profiling")]
    fn build(&self, app: &mut App) {
        detail::build(app);
    }

    #[cfg(not(feature = "profiling"))]
    fn build(&self, _: &mut App) {}
}
