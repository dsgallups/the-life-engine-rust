mod spawn;
use rand::Rng;
pub use spawn::*;

mod ui;

use crate::{
    cpu_net::Cell,
    food::{FoodEaten, Health},
    genome::Genome,
    utils::Random, //old_genome::Genome,
};
use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrganismSet {
    ProcessInput,
    ProcessOutput,
}

#[derive(Component, Reflect)]
pub struct ActiveOrganism;

#[derive(Component)]
#[require(FoodEaten)]
#[require(Health)]
#[require(TicksAlive)]
pub struct Organism {
    genome: Genome,
}

#[derive(Component, Reflect, Default)]
pub struct TicksAlive(u32);

#[derive(Component)]
pub struct Dead;

impl Organism {
    pub fn new(genome: Genome) -> Self {
        Self { genome }
    }
}

pub fn plugin(app: &mut App) {
    app.configure_sets(
        Update,
        (OrganismSet::ProcessInput, OrganismSet::ProcessOutput).chain(),
    );

    app.add_systems(PreUpdate, kill_unhealthy_organisms);
    app.add_systems(Update, update_ticks_alive);
    app.add_systems(PostUpdate, produce_new_wave);

    app.add_plugins((spawn::plugin, ui::plugin));
    app.add_systems(PostUpdate, reset_cells);
}

fn update_ticks_alive(organisms: Query<&mut TicksAlive, Without<Dead>>) {
    for mut tick in organisms {
        tick.0 += 1;
    }
}

fn kill_unhealthy_organisms(
    mut commands: Commands,
    organisms: Query<(Entity, &Health), Without<Dead>>,
) {
    for (organism, health) in organisms {
        if health.0 != 0 {
            continue;
        }
        commands.entity(organism).insert(Dead).despawn_children();
    }
}

fn produce_new_wave(
    mut commands: Commands,
    alive_organisms: Query<&Organism, Without<Dead>>,
    dead_organisms: Query<(Entity, &Organism, &TicksAlive), With<Dead>>,
    mut rand: ResMut<Random>,
    mut msgs: MessageWriter<SpawnOrganism>,
) {
    if !alive_organisms.is_empty() {
        return;
    }

    let mut sorted = Vec::new();

    for (entity, dead_organism, ticks_alive) in dead_organisms {
        sorted.push((dead_organism, ticks_alive.0));

        commands.entity(entity).despawn();
    }
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let mut messages = Vec::with_capacity(30);

    for (parents, _) in sorted.into_iter().take(5) {
        for _ in 0..5 {
            let mut child = parents.genome.deep_clone();
            child.scramble(&mut rand.0);
            let x = rand.0.random_range(-85_f32..=85_f32);
            let y = rand.0.random_range(-85_f32..=85_f32);

            messages.push(SpawnOrganism::new(child, Vec2::new(x, y)));
        }
    }

    msgs.write_batch(messages);
}

fn reset_cells(cells: Query<&Cell>) {
    cells.par_iter().for_each(|cell| {
        cell.reset();
    });
}
