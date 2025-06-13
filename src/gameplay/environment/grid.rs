use bevy::{platform::collections::HashMap, prelude::*};

use crate::gameplay::environment::GlobalCoords;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WorldGrid>();
}

#[derive(Resource, Default)]
pub struct WorldGrid {
    map: HashMap<GlobalCoords, Entity>,
}

impl WorldGrid {
    pub fn get(&self, location: &GlobalCoords) -> Option<&Entity> {
        self.map.get(location)
    }
}
