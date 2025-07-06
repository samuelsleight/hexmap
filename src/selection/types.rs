use bevy::prelude::*;

use crate::{camera::RenderOrder, world::OnHex};

#[derive(Default, Component)]
#[require(OnHex, RenderOrder = RenderOrder::Selection)]
pub struct Indicator;

#[derive(Default, Component)]
#[require(Indicator)]
pub struct HoverIndicator;

#[derive(Default, Component)]
#[require(Indicator)]
pub struct SelectionIndicator;
