use bevy::{asset::uuid::Uuid, prelude::*};
use bimap::BiMap;

use crate::organism::{ActiveCell, BrainCell};

const NODE_LAYER: f32 = 1.;
const EDGE_LAYER: f32 = 0.;
pub const NODE_RADIUS: f32 = 20.;
const MIN_DISTANCE: f32 = 140.;

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

fn spawn_new_nodes(
    mut commands: Commands,
    cell: Single<&BrainCell, With<ActiveCell>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map: ResMut<EntityGraphMap>,
) {
    let brain = cell.network();

    let circle = meshes.add(Circle::new(NODE_RADIUS));

    let mut x = 0.;
    let mut y = -20.;

    for neuron in brain.neurons() {
        if map.get_entity(&neuron.id()).is_none() {
            let neuron_entity = commands
                .spawn((
                    GraphComponent,
                    Nid(neuron.id()),
                    Mesh2d(circle.clone()),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    Transform::from_xyz(x, y, NODE_LAYER),
                ))
                .id();

            map.insert(neuron_entity, neuron.id());

            commands.spawn((
                Text2d::new(neuron.name()),
                TextColor(Color::BLACK),
                ChildOf(neuron_entity),
            ));
            x += 12.;
            y *= -1.;
        }
    }

    for neuron in brain.neurons() {
        let neuron_e = *map.get_entity(&neuron.id()).unwrap();

        let mut new_edges = Vec::new();

        for dendrite in neuron.dendrites() {
            if map.get_entity(&dendrite.id()).is_none() {
                let connected_to = dendrite.connected_to();
                let Some(receives_from) = map.get_entity(&connected_to) else {
                    continue;
                };

                let edge = commands
                    .spawn((
                        GraphComponent,
                        Edge::new(dendrite.id(), *receives_from, neuron_e),
                        Mesh2d(meshes.add(Rectangle::new(LINE_MESH_X, LINE_MESH_Y))),
                        MeshMaterial2d(materials.add(Color::WHITE)),
                        Transform::from_xyz(0., 0., EDGE_LAYER),
                    ))
                    .id();

                map.insert(edge, dendrite.id());

                new_edges.push(edge);
            }
        }
    }

    //
}
