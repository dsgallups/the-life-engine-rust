use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
pub struct ChangeState(pub GameState);

#[derive(Component)]
pub struct OpenLink(pub &'static str);
