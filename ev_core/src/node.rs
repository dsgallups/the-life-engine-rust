use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::RenderLayer;

#[derive(Component)]
#[require(RenderLayers = RenderLayers::from(RenderLayer::NODE_VISUAL))]
pub struct NodeCamera;

pub(super) fn plugin(app: &mut App) {
    //todo
}
