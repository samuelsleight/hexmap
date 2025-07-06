use bevy::prelude::*;

#[derive(Clone, Copy, Component)]
#[require(Transform)]
pub enum RenderOrder {
    Terrain,
    InHex,
    Overlay,
    Selection,
    Border,
}

#[derive(Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord)]
#[require(VisibilityFlags)]
pub enum OverlayMode {
    #[default]
    None,
    Zone,
}

#[derive(Clone, Copy, Default, Resource)]
pub struct CurrentOverlay(pub OverlayMode);

#[derive(Clone, Copy, Component)]
#[require(Visibility)]
pub struct VisibilityFlags {
    pub hex_visibility: bool,
    pub overlay_visibility: bool,
}

impl VisibilityFlags {
    pub fn all(&self) -> bool {
        self.hex_visibility && self.overlay_visibility
    }
}

impl Default for VisibilityFlags {
    fn default() -> Self {
        Self {
            hex_visibility: true,
            overlay_visibility: true,
        }
    }
}
