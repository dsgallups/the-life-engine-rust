use std::ops::Neg as _;

use bevy::prelude::*;
use bevy_spatial::SpatialAccess;

use crate::{
    environment::Dir,
    game::GameState,
    neighbor::VecExt as _,
    organism::{BrainType, Organism},
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
    tree: Res<CellTree>,
    mut organisms: Query<(Entity, &Children, &mut Transform, &Organism)>,
    cells: Query<&Parent, With<CellType>>,
    transforms: Query<&GlobalTransform, Without<Organism>>,
    food: Query<&Food>,
    //killers: Query<&KillerCell>,
) {
    'movement: for (organism_entity, organism_children, mut organism_transform, organism) in
        &mut organisms
    {
        if !organism.can_move() {
            continue;
        }
        let og_trns = organism_transform.translation.as_vec2();
        let direction_to_move = match organism.brain() {
            Some(brain_type) => {
                let cell_count = organism.size();
                // assumes that this organism's cells are always closest to the origin.
                // but this is not true.
                let nearest_neighbors = tree.k_nearest_neighbour(og_trns, cell_count + 2);
                //todo: some logic that grabs the parent of the neighbor and stuff, like filters out

                let mut movement_direction = Dir::rand(&mut rand::thread_rng()).delta();

                for (neighbor_loc, neighbor) in nearest_neighbors {
                    let neighbor = neighbor.unwrap();
                    if food.get(neighbor).is_ok() {
                        let diff_x = neighbor_loc.x - og_trns.x;
                        let diff_y = neighbor_loc.y - og_trns.y;
                        movement_direction = if diff_x < diff_y {
                            Vec2::new(diff_x / diff_x.abs(), 0.)
                        } else {
                            Vec2::new(0., diff_y / diff_y.abs())
                        };
                        break;
                    } else if let Ok(neighbor_parent) = cells.get(neighbor) {
                        if neighbor_parent.get() != organism_entity {
                            let diff_x = neighbor_loc.x - og_trns.x;
                            let diff_y = neighbor_loc.y - og_trns.y;
                            movement_direction = if diff_x < diff_y {
                                Vec2::new(diff_x / diff_x.abs(), 0.)
                            } else {
                                Vec2::new(0., diff_y / diff_y.abs())
                            };
                            if brain_type == BrainType::Prey {
                                movement_direction.x = movement_direction.x.neg();
                                movement_direction.y = movement_direction.y.neg();
                            }
                            break;
                        }
                    }
                }
                movement_direction
            }
            None => Dir::rand(&mut rand::thread_rng()).delta(),
        };

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
                    (Err(_), Err(_)) => {
                        // this happens in some cases where, from finding something at this location
                        // up to now, it has been despawned. Best to just not move.
                        continue 'movement;
                    }
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
