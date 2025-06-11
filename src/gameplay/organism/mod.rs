use bevy::prelude::*;

use crate::gameplay::genome::OrganismGenome;

mod spawn;
pub use spawn::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Organism>();
    app.add_plugins(spawn::plugin);
    //todo
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(SpawnCoords)]
pub struct Organism(pub Handle<OrganismGenome>);

impl From<&Organism> for AssetId<OrganismGenome> {
    fn from(value: &Organism) -> Self {
        (&value.0).into()
    }
}
