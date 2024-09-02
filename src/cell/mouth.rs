use bevy::prelude::*;

use crate::{
    environment::Ticker,
    game::GameState,
    occupied_locations::{GlobalCellLocation, OccupiedLocations},
    organism::{genome::CellLocation, Organism},
};

use super::{CellType, EnvironmentCellType};

#[derive(Component)]
pub struct MouthCell;

pub struct MouthPlugin;

impl Plugin for MouthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, consume_food.run_if(in_state(GameState::Playing)));
    }
}

fn consume_food(
    mut commands: Commands,
    timer: Res<Ticker>,
    mut occupied_locations: ResMut<OccupiedLocations>,
    mouths: Query<(&CellLocation, &Parent), With<MouthCell>>,
    mut organisms: Query<(&mut Organism, &GlobalCellLocation)>,
) {
    if !timer.just_finished() {
        return;
    }
    for (local_mouth_location, parent) in &mouths {
        let Ok((mut organism, organism_location)) = organisms.get_mut(parent.get()) else {
            panic!("Orphan cell found for mouth!");
        };

        let mouth_location = *organism_location + *local_mouth_location;

        for location in mouth_location.around() {
            if let Some(CellType::Environment(EnvironmentCellType::Food)) =
                occupied_locations.cell_type_at(&location)
            {
                //eat food
                let Some((entity, _)) = occupied_locations.remove(&location) else {
                    panic!("something weird just happened")
                };
                commands.entity(entity).despawn_recursive();
                organism.ate_food();
                break;
            }
        }
    }
}
