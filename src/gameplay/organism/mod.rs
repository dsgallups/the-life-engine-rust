use bevy::prelude::*;

use crate::gameplay::genome::OrganismGenome;

mod spawn;
pub use spawn::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Organism>();
    //todo
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(SpawnCoords)]
#[require(SpawnAttempts)]
pub struct Organism(pub Handle<OrganismGenome>);

impl From<&Organism> for AssetId<OrganismGenome> {
    fn from(value: &Organism) -> Self {
        (&value.0).into()
    }
}
