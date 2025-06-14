use bevy::prelude::*;

use crate::gameplay::{
    cell::{CellType, OrganismCellType},
    environment::{EnvironmentQuery, GlobalCoords},
    tick::GameTick,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Producer>();

    app.add_systems(Update, make_food);
    //todo
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(CellType = CellType::Organism(OrganismCellType::Producer))]
pub struct Producer {
    counter: u16,
}

const MAX: u16 = 5;

fn make_food(
    mut producers: Query<(&mut Producer, &GlobalCoords)>,
    spatial_query: EnvironmentQuery,
) {
    for (mut producer, coords) in &mut producers {
        producer.counter += 1;
        if producer.counter < MAX {
            continue;
        }
        let Some(coordinate) = spatial_query.get_free_space(coords) else {
            continue;
        };
    }
}
