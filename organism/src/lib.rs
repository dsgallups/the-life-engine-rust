mod cell;
pub use cell::*;

mod genome;
pub use genome::*;

mod organism;
pub use organism::*;

use bevy::prelude::*;

pub struct OrganismPlugin;

impl Plugin for OrganismPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((cell::plugin, organism::plugin));
    }
}

#[derive(Component, Reflect)]
#[relationship_target(relationship = CellOf)]
pub struct Cells(Vec<Entity>);

#[derive(Component, Reflect)]
#[relationship(relationship_target = Cells)]
pub struct CellOf(pub Entity);
