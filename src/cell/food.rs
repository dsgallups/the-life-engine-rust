use bevy::prelude::*;

use crate::{occupied_locations::GlobalCellLocation, CELL_MULT};

use super::EnvironmentCellType;

#[derive(Component)]
pub struct Food;

#[derive(Bundle)]
pub struct FoodBundle {
    sprite: SpriteBundle,
    location: GlobalCellLocation,
    entity: Food,
}

impl FoodBundle {
    pub fn at(location: GlobalCellLocation) -> Self {
        Self {
            sprite: SpriteBundle {
                transform: Transform::from_translation(location.as_vec3()),
                sprite: Sprite {
                    color: EnvironmentCellType::Food.color(),
                    custom_size: Some(Vec2::new(CELL_MULT, CELL_MULT)),
                    ..Default::default()
                },
                ..Default::default()
            },
            location,
            entity: Food,
        }
    }
}
