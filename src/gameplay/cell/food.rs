use bevy::prelude::*;

use crate::gameplay::{
    cell::{CellMaterials, CellType, panic_without_global_coords},
    environment::GlobalCoords,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Food>();

    app.add_observer(insert_visible_food);
}

#[derive(Component, Reflect)]
#[require(GlobalCoords = panic_without_global_coords())]
#[reflect(Component)]
pub struct Food;

fn insert_visible_food(
    trigger: Trigger<OnAdd, Food>,
    mut commands: Commands,
    materials: Res<CellMaterials>,
) {
    commands.entity(trigger.target()).insert(Sprite {
        color: materials.get_color(&CellType::Food),
        ..default()
    });
}
