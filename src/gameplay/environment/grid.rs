use bevy::{platform::collections::HashMap, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<WorldGrid>();
}

#[derive(Resource, Default)]
pub struct WorldGrid {
    map: HashMap<IVec2, Entity>,
}

impl WorldGrid {
    pub fn get(&self, location: &IVec2) -> Option<&Entity> {
        self.map.get(location)
    }

    pub fn remove(&mut self, location: &IVec2) -> Option<Entity> {
        self.map.remove(location)
    }
    pub fn insert(&mut self, location: IVec2, entity: Entity) -> Option<Entity> {
        self.map.insert(location, entity)
    }
}
