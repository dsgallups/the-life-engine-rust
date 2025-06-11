use bevy::prelude::*;

use crate::gameplay::genome::Genome;

mod spawn;
pub use spawn::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Organism>();
    //todo
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Organism(pub Handle<Genome>);
