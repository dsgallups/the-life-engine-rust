mod cell_types;
pub use cell_types::*;

mod kind;
pub use kind::*;

mod genome;
pub use genome::*;

// mod inputs;
// pub use inputs::*;

// mod outputs;
// pub use outputs::*;

// mod template;
// pub use template::*;

use bevy::{color::palettes::tailwind::*, prelude::*};

// #[derive(Component, Reflect)]
// pub struct ActiveCell;

#[derive(Component, Reflect)]
#[relationship_target(relationship = CellOf)]
pub struct Cells(Vec<Entity>);
impl Cells {
    pub fn cells(&self) -> &[Entity] {
        &self.0
    }
}

#[derive(Component, Reflect)]
#[relationship(relationship_target = Cells)]
pub struct CellOf(pub Entity);

#[derive(Resource)]
pub struct CellAssets {
    pub cell: Handle<Mesh>,
    pub white: Handle<ColorMaterial>,
    pub pink: Handle<ColorMaterial>,
    pub red: Handle<ColorMaterial>,
    pub yellow: Handle<ColorMaterial>,
    pub sky: Handle<ColorMaterial>,
}

impl FromWorld for CellAssets {
    fn from_world(world: &mut World) -> Self {
        let cell = world
            .resource_mut::<Assets<Mesh>>()
            .add(Rectangle::default());
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();
        Self {
            cell,
            white: materials.add(Color::WHITE),
            pink: materials.add(Color::from(PINK_400)),
            red: materials.add(Color::from(RED_600)),
            yellow: materials.add(Color::from(YELLOW_400)),
            sky: materials.add(Color::from(SKY_300)),
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CellAssets>();
    app.add_plugins((cell_types::plugin));
}
