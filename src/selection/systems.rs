use bevy::prelude::*;

use crate::{
    input::MousePosition,
    selection::{HoverIndicator, SelectionIndicator},
    world::{OnHex, WorldLayout, WorldOrigin},
};

pub fn setup_indicators(mut commands: Commands) {
    commands.spawn(HoverIndicator);
    commands.spawn(SelectionIndicator);
}

pub fn mouse_hover(
    position: Res<MousePosition>,
    world: Res<WorldLayout>,
    origin: Single<&Transform, With<WorldOrigin>>,
    indicator: Single<&mut OnHex, With<HoverIndicator>>,
) {
    let origin = origin.into_inner();
    indicator.into_inner().0 = Some(world.pick_tile(position.0, origin.translation.xy()));
}

pub fn mouse_press(
    mouse: Res<ButtonInput<MouseButton>>,
    hover: Single<&OnHex, With<HoverIndicator>>,
    select: Single<&mut OnHex, (With<SelectionIndicator>, Without<HoverIndicator>)>,
) {
    if mouse.just_released(MouseButton::Left) {
        let hovered = hover.into_inner();
        let mut current = select.into_inner();

        if current.0 != hovered.0 {
            current.0 = hovered.0;
        } else {
            current.0 = None;
        }
    }
}
