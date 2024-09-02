use bevy::prelude::*;
use bevy_spatial::SpatialAccess;

use crate::{
    environment::{Dir, Ticker},
    game::GameState,
    neighbor::VecExt as _,
    organism::Organism,
    CellTree,
};

use super::{CellType, Food};

#[derive(Component)]
pub struct MoverCell;

pub struct MoverPlugin;

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_organism.run_if(in_state(GameState::Playing)));
    }
}

pub fn move_organism(
    mut commands: Commands,
    timer: Res<Ticker>,
    tree: Res<CellTree>,
    mut organisms: Query<(Entity, &Children, &mut Transform, &Organism)>,
    cells: Query<&Parent, With<CellType>>,
    transforms: Query<&GlobalTransform, Without<Organism>>,
    food: Query<&Food>,
) {
    if !timer.just_finished() {
        return;
    }

    'movement: for (organism_entity, organism_children, mut organism_transform, organism) in
        &mut organisms
    {
        if !organism.can_move() {
            continue;
        }
        let direction_to_move = Dir::rand(&mut rand::thread_rng()).delta();

        let mut food_to_despawn = Vec::new();
        for child in organism_children {
            let new_child_transform = (transforms.get(*child).unwrap().translation().as_vec2()
                + direction_to_move)
                .round();
            let (closest, closest_entity) = tree.nearest_neighbour(new_child_transform).unwrap();
            let closest_entity = closest_entity.unwrap();

            //something is here already. It could be a cell that is part of this component or food or another cell
            if new_child_transform == closest {
                match (food.get(closest_entity), cells.get(closest_entity)) {
                    (Err(_), Ok(cell_parent)) => {
                        if cell_parent.get() != organism_entity {
                            //someone, but not us, is in the way
                            continue 'movement;
                        }
                    }
                    (Ok(_), _) => {
                        //despawn this food. This could be bad though
                        // if a mouth is right next to some food and the mouthplugin hasn't run yet. hmmm
                        // maybe we run this after feed_organism
                        food_to_despawn.push(closest_entity)
                    }
                    (Err(_), Err(e)) => panic!("Not sure how we got here: {}", e),
                }
            }
        }

        // at this point, we are free to move. despawn the food
        for food_entity in food_to_despawn {
            if let Some(mut ec) = commands.get_entity(food_entity) {
                ec.despawn();
            }
        }

        organism_transform.translation =
            (organism_transform.translation + direction_to_move.as_vec3()).round()
    }
}
