use bevy::{asset::uuid::Uuid, prelude::*};
use bimap::BiMap;

const NODE_LAYER: f32 = 1.;
const EDGE_LAYER: f32 = 0.;

#[derive(Resource, Default)]
pub struct EntityGraphMap {
    entity_map: BiMap<Entity, Uuid>,
}
impl EntityGraphMap {
    pub fn insert(&mut self, entity: Entity, id: Uuid) {
        self.entity_map.insert(entity, id);
    }
    pub fn get_uuid(&self, entity: &Entity) -> Option<&Uuid> {
        self.entity_map.get_by_left(entity)
    }
    pub fn get_entity(&self, uuid: &Uuid) -> Option<&Entity> {
        self.entity_map.get_by_right(uuid)
    }
    fn remove(&mut self, entity: &Entity) {
        self.entity_map.remove_by_left(entity);
    }
    fn clear(&mut self) {
        self.entity_map.clear();
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<EntityGraphMap>();
}
