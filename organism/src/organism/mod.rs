use bevy::prelude::*;

#[derive(Component, Reflect)]
#[relationship_target(relationship = CellOf)]
pub struct Cells(Vec<Entity>);

#[derive(Component, Reflect)]
#[relationship(relationship_target = Cells)]
pub struct CellOf(pub Entity);

pub(super) fn plugin(app: &mut App) {
    //todo
}
