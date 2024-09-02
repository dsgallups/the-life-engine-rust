use bevy::prelude::*;

use crate::environment::location::GlobalCellLocation;

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
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..Default::default()
                },
                ..Default::default()
            },
            location,
            entity: Food,
        }
    }
}
