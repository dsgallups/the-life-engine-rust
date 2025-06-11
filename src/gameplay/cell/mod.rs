use bevy::prelude::*;

use crate::gameplay::world::GlobalCoords;

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
    app.add_plugins((
        armor::plugin,
        eye::plugin,
        food::plugin,
        killer::plugin,
        mouth::plugin,
        mover::plugin,
        producer::plugin,
    ));

    app.register_type::<Cell>().register_type::<Cells>();
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
