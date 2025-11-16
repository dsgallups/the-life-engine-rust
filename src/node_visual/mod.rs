mod edge;
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

#[derive(Component)]
pub struct GraphComponent;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((node::plugin, edge::plugin));
    app.init_resource::<EntityGraphMap>();

    app.add_systems(PreUpdate, spawn_new_nodes);

    app.add_systems(PostUpdate, (despawn_dead_nodes, despawn_dead_edges).chain());
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

    // let spawner = |neuron: &CpuNeuron, name| {
    //     let id = neuron.id();
    //     if map.get_entity(&id).is_none() {
    //         let neuron_entity = commands
    //             .spawn((
    //                 GraphComponent,
    //                 RenderLayers::from(RenderLayer::NODE_VISUAL),
    //                 Nid(id),
    //                 Mesh2d(circle.clone()),
    //                 MeshMaterial2d(materials.add(Color::WHITE)),
    //                 Transform::from_xyz(x, y, NODE_LAYER),
    //             ))
    //             .id();

    //         map.insert(neuron_entity, id);

    //         commands.spawn((
    //             Text2d::new(name),
    //             RenderLayers::from(RenderLayer::NODE_VISUAL),
    //             TextColor(Color::BLACK),
    //             ChildOf(neuron_entity),
    //         ));
    //         x += 12.;
    //         y *= -1.;
    //     }
    //     neuron.on_inputs(|input_neuron| {
    //         spawner(input_neuron, "Node".to_string());
    //     });
    // };

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

    // for neuron in brain.neurons() {
    //     if map.get_entity(&neuron.id()).is_none() {
    //         let neuron_entity = commands
    //             .spawn((
    //                 GraphComponent,
    //                 RenderLayers::from(RenderLayer::NODE_VISUAL),
    //                 Nid(neuron.id()),
    //                 Mesh2d(circle.clone()),
    //                 MeshMaterial2d(materials.add(Color::WHITE)),
    //                 Transform::from_xyz(x, y, NODE_LAYER),
    //             ))
    //             .id();

    //         map.insert(neuron_entity, neuron.id());

    //         commands.spawn((
    //             Text2d::new(neuron.name()),
    //             RenderLayers::from(RenderLayer::NODE_VISUAL),
    //             TextColor(Color::BLACK),
    //             ChildOf(neuron_entity),
    //         ));
    //         x += 12.;
    //         y *= -1.;
    //     }
    // }

    for neuron in brain.neurons() {
        let neuron_e = *map.get_entity(&neuron.id()).unwrap();

        let mut new_edges = Vec::new();

        let inner = neuron.read();

        let Some(inputs) = inner.inputs() else {
            continue;
        };

        for input in inputs {
            if map.get_entity(&input.id()).is_none() {
                let connected_to = input.node().id();
                let Some(receives_from) = map.get_entity(&connected_to) else {
                    continue;
                };

                let edge = commands
                    .spawn((
                        GraphComponent,
                        RenderLayers::from(RenderLayer::NODE_VISUAL),
                        Edge::new(input.id(), *receives_from, neuron_e),
                        Mesh2d(meshes.add(Rectangle::new(LINE_MESH_X, LINE_MESH_Y))),
                        MeshMaterial2d(materials.add(Color::WHITE)),
                        Transform::from_xyz(0., 0., EDGE_LAYER),
                    ))
                    .id();

                map.insert(edge, input.id());

                new_edges.push(edge);
            }
        }
    }

    //
}

fn despawn_dead_nodes(
    mut commands: Commands,
    cell: Single<&BrainCell, With<ActiveCell>>,
    nodes: Query<(Entity, &Nid)>,
    mut map: ResMut<EntityGraphMap>,
) {
    let brain = cell.network();

    for (node, id) in nodes {
        if brain.get_neuron(id.0).is_none() {
            map.remove(&node);
            commands.entity(node).despawn();
        }
    }
}

fn despawn_dead_edges(
    mut commands: Commands,
    edges: Query<(Entity, &Edge)>,
    cell: Single<&BrainCell, With<ActiveCell>>,
    mut map: ResMut<EntityGraphMap>,
) {
    let brain = cell.network();
    for (entity, edge) in edges {
        if !brain.has_input(edge.id()) {
            map.remove(&entity);
            commands.entity(entity).despawn();
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
