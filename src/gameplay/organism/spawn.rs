use bevy::prelude::*;

use crate::gameplay::{GameSystems, genome::OrganismGenome, organism::Organism};

/// the original location to spawn an organism
#[derive(Component, Deref, DerefMut, Default)]
pub struct SpawnCoords(pub IVec2);

/// the inner count is going to be attempts to spawn this organism
#[derive(Component, Default)]
pub struct SpawnAttempts(u32);

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnOrganism>();

    app.add_systems(Update, spawn_organisms.in_set(GameSystems::Update));

    //todo
}
fn spawn_organisms(
    mut new_organisms: Query<(Entity, &SpawnCoords, &Organism, &mut SpawnAttempts)>,
    genomes: Res<Assets<OrganismGenome>>,
    mut commands: Commands,
) {
    //TODO: we will perform a query on the world to see if the thing conflicts, and if it does, the organism will be marked for deletion.
    for (entity, coords, organism, mut spawn_attempts) in &mut new_organisms {
        let Some(genome) = genomes.get(organism) else {
            continue;
        };

        for cell in genome.iter_cells() {
            //todo
        }

        //todo
    }
    //todo
}

#[derive(Event)]
pub struct SpawnOrganism {
    genome: Handle<OrganismGenome>,
    location: IVec2,
}
impl SpawnOrganism {
    pub fn new(genome: Handle<OrganismGenome>, location: IVec2) -> Self {
        Self { genome, location }
    }
}
