use bevy::prelude::*;

use crate::{environment::EnvironmentSettings, neighbor::VecExt, CellTree, GameState};

use super::FoodBundle;

#[derive(Component, Default)]
pub struct ProducerCell {
    counter: u16,
}

pub struct ProducerPlugin;

impl Plugin for ProducerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tick_producers.run_if(in_state(GameState::Playing)),
                produce_food.run_if(in_state(GameState::Playing)),
            ),
        );
    }
}

#[derive(Component)]
pub struct MakeFood;

pub fn tick_producers(
    par_commands: ParallelCommands,
    settings: Res<EnvironmentSettings>,
    mut producers: Query<(Entity, &mut ProducerCell)>,
) {
    producers.par_iter_mut().for_each(|(pe, mut producer)| {
        producer.counter += 1;
        if producer.counter >= settings.producer_threshold {
            //(&commands).entity(pe).insert(MakeFood);
            par_commands.command_scope(|mut c| {
                c.entity(pe).try_insert(MakeFood);
            });
            producer.counter = 0;
        }
    });
}

pub fn produce_food(
    par_commands: ParallelCommands,
    locations: Res<CellTree>,
    producers: Query<(Entity, &GlobalTransform), With<MakeFood>>,
) {
    producers.par_iter().for_each(|(pe, producer_location)| {
        let translation = producer_location.translation();
        if let Some(free_space) = translation.get_free_space(&locations) {
            par_commands.command_scope(|mut commands| {
                commands.spawn(FoodBundle::at(free_space));
                commands.entity(pe).remove::<MakeFood>();
            });
        }
    });
}
