use bevy::prelude::*;

use crate::gameplay::{cell::CellType, tick::GameTick};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Producer>();

    app.add_systems(Update, make_food);
    //todo
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(CellType = CellType::Producer)]
pub struct Producer {
    counter: u16,
}

const MAX: u16 = 5;

fn make_food(mut producers: Query<&mut Producer>) {
    for mut producer in &mut producers {
        producer.counter += 1;
        if producer.counter < MAX {
            continue;
        }
    }
}
