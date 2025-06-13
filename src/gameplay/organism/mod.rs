use bevy::prelude::*;

use crate::{
    gameplay::{GameSet, GameState, genome::OrganismGenome},
    screens::Screen,
};

mod spawn;
pub use spawn::*;

#[derive(SubStates, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
#[source(Screen = Screen::Gameplay)]
enum OrganismState {
    #[default]
    Move,
    Produce,
    Eat,
    Attack,
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum OrganismSet {
    Move,
    Produce,
    Eat,
    Attack,
}

impl OrganismState {
    pub fn next(&self) -> OrganismState {
        match self {
            OrganismState::Eat => OrganismState::Produce,
            OrganismState::Produce => OrganismState::Move,
            OrganismState::Move => OrganismState::Attack,
            OrganismState::Attack => OrganismState::Eat,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Organism>()
        .add_sub_state::<OrganismState>();

    app.configure_sets(
        Update,
        OrganismSet::Move
            .run_if(in_state(OrganismState::Move))
            .in_set(GameSet::Update),
    )
    .configure_sets(
        Update,
        OrganismSet::Produce
            .run_if(in_state(OrganismState::Produce))
            .in_set(GameSet::Update),
    )
    .configure_sets(
        Update,
        OrganismSet::Attack
            .run_if(in_state(OrganismState::Attack))
            .in_set(GameSet::Update),
    )
    .configure_sets(
        Update,
        OrganismSet::Eat
            .run_if(in_state(OrganismState::Eat))
            .in_set(GameSet::Update),
    );

    app.add_plugins(spawn::plugin);

    app.add_systems(
        PreUpdate,
        update_organism_state.run_if(in_state(GameState::Playing)),
    );
    //todo
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(SpawnCoords)]
pub struct Organism(pub Handle<OrganismGenome>);

impl From<&Organism> for AssetId<OrganismGenome> {
    fn from(value: &Organism) -> Self {
        (&value.0).into()
    }
}

fn update_organism_state(
    state: Res<State<OrganismState>>,
    mut next_state: ResMut<NextState<OrganismState>>,
) {
    next_state.set(state.get().next());
}
