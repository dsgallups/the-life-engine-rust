use bevy::prelude::*;

use crate::gameplay::genome::OrganismGenome;

mod spawn;
pub use spawn::*;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum CellSet {
    /// Movement
    Move,
    Produce,
    Eat,
    Attack,
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Organism>().configure_sets(
        Update,
        (
            CellSet::Move,
            (CellSet::Eat, CellSet::Attack),
            CellSet::Produce,
        )
            .chain(),
    );
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
