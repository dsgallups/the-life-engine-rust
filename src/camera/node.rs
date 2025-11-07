use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::camera::RenderLayer;

#[derive(Component)]
#[require(RenderLayers = RenderLayers::from(RenderLayer::CELL_VISUAL))]
pub struct NodeCamera;

pub(super) fn plugin(app: &mut App) {
    //todo
}
