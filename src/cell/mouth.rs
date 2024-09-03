use bevy::prelude::*;

use crate::{game::GameState, neighbor::VecExt as _, organism::Organism, CellTree};

use super::Food;

#[derive(Component)]
pub struct MouthCell;

pub struct MouthPlugin;

impl Plugin for MouthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, consume_food.run_if(in_state(GameState::Playing)));
    }
}

fn consume_food(
    mut commands: Commands,
    locations: Res<CellTree>,
    mouths: Query<(&GlobalTransform, &Parent), With<MouthCell>>,
    mut organisms: Query<&mut Organism>,
    food: Query<&Food>,
) {
    for (mouth, mouth_parent) in &mouths {
        let translation = mouth.translation();

        let mut food_eaten = 0;

        for entity in translation.get_surrounding_entities(&locations) {
            if food.get(entity).is_ok() {
                //eat the food. may be gone by the time we get here
                // if we get the entity and it despawns, just gotta hand it to the organism for winning
                // the race condition
                if let Some(mut entity) = commands.get_entity(entity) {
                    entity.despawn();
                    food_eaten += 1;
                }
            }
        }

        if food_eaten > 0 {
            organisms
                .get_mut(mouth_parent.get())
                .unwrap()
                .ate_food(food_eaten);
        }
    }
}
