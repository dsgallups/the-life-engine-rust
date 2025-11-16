mod edge;
use std::collections::HashMap;

pub use edge::*;

mod node;
use ev_core::RenderLayer;
pub use node::*;

use bevy::{asset::uuid::Uuid, camera::visibility::RenderLayers, prelude::*};
use bimap::BiMap;
use organism::{
    ActiveOrganism, Cells,
    cpu_net::{Cell, CpuNeuron},
};

const NODE_LAYER: f32 = 1.;
const EDGE_LAYER: f32 = 0.;
pub const NODE_RADIUS: f32 = 20.;
const MIN_DISTANCE: f32 = 140.;

#[derive(Resource, Default)]
pub struct EntityGraphMap {
    entity_map: BiMap<Entity, Uuid>,
    /// contains the (sender, receiver), edge id
    connections: HashMap<(Uuid, Uuid), Uuid>, //connection_map
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
    pub fn get(&self, sender: Uuid, receiver: Uuid) -> Option<Uuid> {
        self.connections.get(&(sender, receiver)).copied()
    }
    pub fn insert_conn(&mut self, sender: Uuid, receiver: Uuid) -> Uuid {
        let id = Uuid::new_v4();
        self.connections.insert((sender, receiver), id);
        id
    }
}

#[derive(Component)]
pub struct GraphComponent;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((node::plugin, edge::plugin));
    app.init_resource::<EntityGraphMap>();

    app.add_systems(PreUpdate, spawn_new_nodes);
}

fn spawn_new_nodes(
    mut commands: Commands,
    organism: Single<&Cells, With<ActiveOrganism>>,
    cells: Query<&Cell>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map: ResMut<EntityGraphMap>,
) {
    let circle = meshes.add(Circle::new(NODE_RADIUS));

    let mut x = 0.;
    let mut y = -20.;

    for cell in organism.cells() {
        let Ok(cell) = cells.get(*cell) else {
            warn!("CELL MISSING CELL");
            continue;
        };
        for (i, neuron) in cell.output_neurons().iter().enumerate() {
            let name = format!("{:?} Output {i}", cell.kind());
            neuron_spawner(
                commands.reborrow(),
                &circle,
                materials.as_mut(),
                &mut x,
                &mut y,
                map.as_mut(),
                neuron,
                name,
            );
        }
    }

    for cell in organism.cells() {
        let Ok(cell) = cells.get(*cell) else {
            continue;
        };

        for neuron in cell.output_neurons() {
            edge_spawner(
                commands.reborrow(),
                meshes.as_mut(),
                materials.as_mut(),
                map.as_mut(),
                neuron,
            );
        }
    }
}

fn neuron_spawner(
    mut commands: Commands,
    circle: &Handle<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    x: &mut f32,
    y: &mut f32,
    map: &mut EntityGraphMap,
    neuron: &CpuNeuron,
    name: String,
) {
    let id = neuron.id();
    if map.get_entity(&id).is_none() {
        let neuron_entity = commands
            .spawn((
                GraphComponent,
                RenderLayers::from(RenderLayer::NODE_VISUAL),
                Nid(id),
                Mesh2d(circle.clone()),
                MeshMaterial2d(materials.add(Color::WHITE)),
                Transform::from_xyz(*x, *y, NODE_LAYER),
            ))
            .id();

        map.insert(neuron_entity, id);

        commands.spawn((
            Text2d::new(name),
            RenderLayers::from(RenderLayer::NODE_VISUAL),
            TextColor(Color::BLACK),
            ChildOf(neuron_entity),
        ));
        *x += 12.;
        *y *= -1.;
    }

    neuron.on_inputs(|input_neuron| {
        neuron_spawner(
            commands.reborrow(),
            circle,
            materials,
            x,
            y,
            map,
            input_neuron,
            "Node".to_string(),
        );
    });
}

fn edge_spawner(
    mut commands: Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    map: &mut EntityGraphMap,
    neuron: &CpuNeuron,
) {
    let neuron_id = neuron.id();
    let neuron_e = *map.get_entity(&neuron.id()).unwrap();

    let mut new_edges = Vec::new();
    neuron.on_inputs(|input_neuron| {
        let receives_from_id = input_neuron.id();
        if map.get(receives_from_id, neuron_id).is_none() {
            let Some(receives_from) = map.get_entity(&receives_from_id).copied() else {
                return;
            };
            let connection_id = map.insert_conn(receives_from_id, neuron_id);

            let edge = commands
                .spawn((
                    GraphComponent,
                    RenderLayers::from(RenderLayer::NODE_VISUAL),
                    Edge::new(connection_id, receives_from, neuron_e),
                    Mesh2d(meshes.add(Rectangle::new(LINE_MESH_X, LINE_MESH_Y))),
                    MeshMaterial2d(materials.add(Color::WHITE)),
                    Transform::from_xyz(0., 0., EDGE_LAYER),
                ))
                .id();

            map.insert(edge, connection_id);

            new_edges.push(edge);
        }

        edge_spawner(commands.reborrow(), meshes, materials, map, input_neuron);
    });
}
