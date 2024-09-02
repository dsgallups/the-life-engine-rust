use bevy::prelude::*;

use crate::ORGANISM_LAYER;

use super::{CellType, EnvironmentCellType};

#[derive(Component)]
pub struct Food;

#[derive(Bundle)]
pub struct FoodBundle {
    sprite: SpriteBundle,
    entity: Food,
    cell_type: CellType,
}

impl FoodBundle {
    pub fn at(location: Vec2) -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    location.x,
                    location.y,
                    ORGANISM_LAYER,
                )),
                sprite: Sprite {
                    color: EnvironmentCellType::Food.color(),
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            entity: Food,
            cell_type: CellType::food(),
        }
    }
}
