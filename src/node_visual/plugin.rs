use bevy::prelude::*;

use crate::node_visual::EntityGraphMap;

pub fn plugin(app: &mut App) {
    app.init_resource::<EntityGraphMap>();
}
