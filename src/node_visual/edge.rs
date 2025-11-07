use bevy::{color::palettes::tailwind::RED_400, platform::collections::HashMap, prelude::*};
use uuid::Uuid;

#[derive(Component, Reflect)]
pub struct Edge {
    id: Uuid,
    sender: Entity,
    receiver: Entity,
}
pub const LINE_MESH_X: f32 = 1.;
pub const LINE_MESH_Y: f32 = 2.;

impl Edge {
    pub fn new(id: Uuid, sender: Entity, receiver: Entity) -> Self {
        Self {
            sender,
            id,
            receiver,
        }
    }
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn sender(&self) -> Entity {
        self.sender
    }
    pub fn receiver(&self) -> Entity {
        self.receiver
    }
}

#[derive(Message)]
pub struct EdgeUpdates {
    map: HashMap<Uuid, i32>,
}
impl EdgeUpdates {
    pub fn empty() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn set(values: impl IntoIterator<Item = (Uuid, i32)>) -> Self {
        Self {
            map: values.into_iter().collect(),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (update_edge_transforms, update_edge_colors));
    app.add_message::<EdgeUpdates>();
}

fn update_edge_transforms(
    edges: Query<(&mut Transform, &Edge)>,
    nodes: Query<&Transform, Without<Edge>>,
) {
    for (mut transform, edge) in edges {
        if let Ok(sender_trns) = nodes.get(edge.sender())
            && let Ok(recv_trns) = nodes.get(edge.receiver())
        {
            let val = (recv_trns.translation.xy() - sender_trns.translation.xy());
            let length = val.length();
            if length > 0. {
                transform.scale.x = length;
            }
            transform.translation = sender_trns.translation + (Vec3::new(val.x, val.y, 0.) * 0.5);

            let angle = val.y.atan2(val.x);

            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

fn update_edge_colors(
    mut reader: MessageReader<EdgeUpdates>,
    edges: Query<(&Edge, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for update in reader.read() {
        info!("In update edge colors");
        for (edge, material_handle) in edges {
            let Some(material) = materials.get_mut(&material_handle.0) else {
                continue;
            };
            let val = update.map.get(&edge.id).copied().unwrap_or_default();

            if val > 0 {
                info!("COLOR IS RED");
                material.color = RED_400.into();
            } else {
                info!("COLOR IS WHITE");
                material.color = Color::WHITE;
            }
        }
        //todo
    }
}
