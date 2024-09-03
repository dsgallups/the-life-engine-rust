use bevy::prelude::*;

use crate::{neighbor::VecExt as _, organism::Organism, CellTree, GameState};

use super::{CellType, FoodBundle};

#[derive(Component)]
pub struct KillerCell;

pub struct KillerPlugin;

impl Plugin for KillerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, kill_organisms.run_if(in_state(GameState::Playing)));
    }
}

fn kill_organisms(
    mut commands: Commands,
    tree: Res<CellTree>,
    killers: Query<(&Parent, &GlobalTransform), With<KillerCell>>,
    cells: Query<(&Parent, &CellType)>,
    organisms: Query<&Children, With<Organism>>,
    locations: Query<&GlobalTransform>,
) {
    for (killer_parent, killer_transform) in &killers {
        let translation = killer_transform.translation();

        for touched_entity in translation.get_surrounding_entities(&tree) {
            if let Ok((cell_parent, cell_type)) = cells.get(touched_entity) {
                if cell_type == &CellType::armor() || killer_parent.get() == cell_parent.get() {
                    continue;
                }
                info!("Kill command incoming!");
                let organism_to_kill = cell_parent.get();

                // kill the parent and replace it with food
                if let Ok(organism_children) = organisms.get(organism_to_kill) {
                    info!("replacing organism with food");
                    for organism_child in organism_children.iter() {
                        let Ok(location) = locations.get(*organism_child) else {
                            continue;
                        };
                        commands.spawn(FoodBundle::at(location.translation()));
                    }
                } else {
                    info!("couldn't kill organism: 1")
                }

                if let Some(organism_entity) = commands.get_entity(organism_to_kill) {
                    info!("despawning organism");
                    organism_entity.despawn_recursive()
                }
            }
        }
    }
}
