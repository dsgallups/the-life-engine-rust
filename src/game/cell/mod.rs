use bevy::prelude::*;
mod neurons;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(neurons::plugin);
    //todo
}
