use bevy::prelude::*;

use super::super::Indicator;

#[derive(Default, Component)]
#[require(Indicator)]
pub struct HoverIndicator;

#[derive(Default, Component)]
#[require(Indicator)]
pub struct SelectionIndicator;
