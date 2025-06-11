use bevy::prelude::*;

use crate::gameplay::{
    GameSet, cell::*, genome::OrganismGenome, organism::Organism, world::GlobalCoords,
};

/// the original location to spawn an organism
#[derive(Component, Deref, DerefMut, Default)]
pub struct SpawnCoords(pub IVec2);

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnOrganism>();

    app.add_systems(Update, spawn_organisms.in_set(GameSet::Update));

    //todo
}

fn spawn_organisms(
    new_organisms: Query<(Entity, &SpawnCoords, &Organism), Without<Cells>>,
    genomes: Res<Assets<OrganismGenome>>,
    mut commands: Commands,
) {
    //TODO: we will perform a query on the world to see if the thing conflicts, and if it does, the organism will be marked for deletion.
    for (entity, coords, organism) in &new_organisms {
        let SpawnCoords(root_coords) = coords;
        let Some(genome) = genomes.get(organism) else {
            info!("Genome not yet loaded for unspawned organism");
            continue;
        };

        info!("Spawning organism");

        for cell in genome.iter_cells() {
            let local_location = cell.location();

            let default_components = (Cell(entity), GlobalCoords(root_coords + local_location));

            match cell.cell_type() {
                CellType::Armor => {
                    commands.spawn((Armor, default_components));
                }
                CellType::Eye => {
                    commands.spawn((Eye, default_components));
                }
                CellType::Mouth => {
                    commands.spawn((Mouth, default_components));
                }
                CellType::Mover => {
                    commands.spawn((Mover, default_components));
                }
                CellType::Killer => {
                    commands.spawn((Killer, default_components));
                }
                CellType::Producer => {
                    commands.spawn((Producer::default(), default_components));
                }
            }
        }
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
