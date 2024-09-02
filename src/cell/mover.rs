use bevy::prelude::*;

use crate::{
    environment::{
        location::{GlobalCellLocation, OccupiedLocations},
        Dir, Ticker,
    },
    game::GameState,
    organism::Organism,
};

use super::CellType;

#[derive(Component)]
pub struct MoverCell;

pub struct MoverPlugin;

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_organism.run_if(in_state(GameState::Playing)));
    }
}

pub fn move_organism(
    mut commands: Commands,
    timer: Res<Ticker>,
    mut occupied_locations: ResMut<OccupiedLocations>,
    mut organisms: Query<(Entity, &mut Transform, &mut GlobalCellLocation, &Organism)>,
) {
    if !timer.just_finished() {
        return;
    }

    'mover: for (organism_entity, mut organism_transform, mut global_location, organism) in
        &mut organisms
    {
        if !organism.can_move() {
            continue;
        }

        //move in a random direction
        let direction_to_move = Dir::rand(&mut rand::thread_rng()).delta();
        let new_parent_location = *global_location + direction_to_move;

        info!(
            "Direction to move: {:?}\nnew_parent_location: {:?}",
            direction_to_move, new_parent_location
        );
        for location in organism.occupying_locations() {
            if occupied_locations
                .get(&(new_parent_location + location))
                .is_some_and(|(e, t)| e != organism_entity && t != CellType::food())
            {
                continue 'mover;
            }
        }
        /*
            the direction is free to be moved into.
            the following things need to happen:
            1. the occupied_locations need to be updated
            2. the organism's transform and global cell location need to be updated

            the children do not have a global cell location to care about.
        */

        // update occupied_locations.
        // this needs to happen in two loops since
        // we could potentially be deleting locations in the previous
        // iteration of the loop
        for location in organism.occupying_locations() {
            let current_location = *global_location + location;
            if let Some((_e, _cell_type)) = occupied_locations.remove(&current_location) {
                //they sometimes don't match up
            }
        }
        for cell in organism.cells() {
            let new_location = new_parent_location + cell.location();

            if let Some((e, cell)) =
                occupied_locations.insert(new_location, organism_entity, cell.cell_type())
            {
                if cell == CellType::food() {
                    if let Some(mut e) = commands.get_entity(e) {
                        e.remove_parent();
                        e.despawn_recursive();
                    }
                }
            }
        }

        //now update the organism's components
        *global_location = new_parent_location;
        organism_transform.translation = new_parent_location.as_vec3();
    }
}
