use std::marker::PhantomData;

use bevy::{ecs::system::SystemParam, prelude::*};
use rand::seq::IndexedRandom;

use super::grid::WorldGrid;
use crate::gameplay::environment::{DIRECTIONS, GlobalCoords};

pub(super) fn plugin(app: &mut App) {
    //todo
}

#[derive(SystemParam)]
pub struct EnvironmentQuery<'w, 's> {
    //todo
    grid: Res<'w, WorldGrid>,
    _phantom: PhantomData<&'s ()>,
}

impl<'w, 's> EnvironmentQuery<'w, 's> {
    /// Returns a free space if found. NOTE: does not guarantee that the caller will get this free space.
    pub fn get_free_space(&self, coords: &GlobalCoords) -> Option<GlobalCoords> {
        for direction in DIRECTIONS.choose_multiple(&mut rand::rng(), 4) {
            let location = coords.directional_move(*direction);
            if self.grid.get(&location.0).is_none() {
                return Some(location);
            };
        }
        None
    }
}
