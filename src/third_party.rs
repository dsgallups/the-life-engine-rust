use avian2d::{PhysicsPlugins, prelude::Gravity};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
    app.insert_resource(Gravity::ZERO);
}
