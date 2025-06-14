use bevy::{color::palettes::tailwind::BLUE_500, platform::collections::HashMap, prelude::*};

use crate::{asset_tracking::LoadResource, gameplay::environment::GlobalCoords};

mod armor;
pub use armor::*;

mod eye;
pub use eye::*;

mod food;
pub use food::*;

mod killer;
pub use killer::*;

mod mouth;
pub use mouth::*;

mod mover;
pub use mover::*;

mod producer;
pub use producer::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<OrganismCellType>()
        .register_type::<CellType>()
        .register_type::<CellMaterials>()
        .load_resource::<CellMaterials>()
        .register_type::<Cell>()
        .register_type::<Cells>();
    app.add_plugins((
        armor::plugin,
        eye::plugin,
        food::plugin,
        killer::plugin,
        mouth::plugin,
        mover::plugin,
        producer::plugin,
    ));

    app.add_observer(insert_visible_components);
    //todo
}

/// Simple link to the animation player of a model that is buried deep in the hierarchy.
#[derive(Component, Reflect, Clone, Deref)]
#[reflect(Component)]
#[relationship_target(relationship = Cell)]
pub(crate) struct Cells(Vec<Entity>);

#[derive(Component, Reflect, Deref)]
#[reflect(Component)]
#[allow(unreachable_code)]
#[require(GlobalCoords = panic_without_global_coords())]
#[relationship(relationship_target = Cells)]
pub(crate) struct Cell(pub(crate) Entity);

fn panic_without_global_coords() -> GlobalCoords {
    panic!("Cell must have global coordinates")
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum OrganismCellType {
    Armor,
    Eye,
    Killer,
    Mover,
    Producer,
    Mouth,
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, Reflect, Component)]
#[reflect(Component)]
pub enum CellType {
    Organism(OrganismCellType),
    Wall,
    Food,
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct CellMaterials {
    materials: HashMap<CellType, Color>,
}
impl CellMaterials {
    pub fn get_color(&self, cell_type: &CellType) -> Color {
        *self.materials.get(cell_type).unwrap()
    }
}

impl FromWorld for CellMaterials {
    fn from_world(_world: &mut World) -> Self {
        let mut map = HashMap::new();
        map.insert(
            CellType::Organism(OrganismCellType::Armor),
            Color::linear_rgb(0.5, 0.0, 0.5),
        );
        map.insert(
            CellType::Organism(OrganismCellType::Producer),
            Color::linear_rgb(0., 1., 0.),
        );
        map.insert(
            CellType::Organism(OrganismCellType::Mouth),
            Color::linear_rgb(1.0, 0.65, 0.),
        );
        map.insert(
            CellType::Organism(OrganismCellType::Mover),
            Color::linear_rgb(0.49, 1.0, 0.83),
        );
        map.insert(
            CellType::Organism(OrganismCellType::Killer),
            Color::linear_rgb(1.0, 0.0, 0.0),
        );
        map.insert(
            CellType::Organism(OrganismCellType::Eye),
            Color::linear_rgb(0.98, 0.5, 0.45),
        );
        map.insert(CellType::Food, BLUE_500.into());
        Self { materials: map }
    }
}

fn insert_visible_components(
    trigger: Trigger<OnAdd, CellType>,
    mut commands: Commands,
    materials: Res<CellMaterials>,
    cell_types: Query<&CellType>,
) {
    let Ok(cell_type) = cell_types.get(trigger.target()) else {
        return;
    };
    let color = materials.get_color(cell_type);

    commands
        .entity(trigger.target())
        .insert(Sprite { color, ..default() });
}
