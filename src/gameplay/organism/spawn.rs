use bevy::prelude::*;

use crate::gameplay::{
    GameSystems,
    cell::{Cell, Cells},
    genome::{CellType, OrganismGenome},
    organism::Organism,
    world::GlobalCoords,
};

/// the original location to spawn an organism
#[derive(Component, Deref, DerefMut, Default)]
pub struct SpawnCoords(pub IVec2);

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnOrganism>();

    app.add_systems(Update, spawn_organisms.in_set(GameSystems::Update));

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
            continue;
        };

        for cell in genome.iter_cells() {
            let local_location = cell.location();

            let global_coords = (Cell(entity), GlobalCoords(root_coords + local_location));

            match cell.cell_type() {
                CellType::Armor => {
                    //todo
                    //lol
                }
                _ => todo!(),
            }
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
