use bevy::prelude::*;

use crate::gameplay::{
    cell::Producer,
    environment::{GlobalCoords, GridSet, grid::WorldGrid},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, produce_foods.in_set(GridSet::RectifyGrid));
    //todo
}

#[derive(Event)]
pub struct ProduceFood {
    location: GlobalCoords,
    producer: Entity,
}

pub fn produce_foods(
    mut event_reader: EventReader<ProduceFood>,
    grid: ResMut<WorldGrid>,
    producers: Query<&Producer>,
) {
    for event in event_reader.read() {
        if grid.get(&event.location.0).is_some() {
            continue;
        }

        todo!()
    }
}
