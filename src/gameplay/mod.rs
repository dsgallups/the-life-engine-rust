//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::{Val::*, *},
};

use crate::{Pause, gameplay::tick::GameTick, menus::Menu, screens::Screen};

mod cell;
mod environment;
mod genome;
mod level;
mod organism;
mod tick;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum GameSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Things irrelevant to the state of the game that need to happen after input is recorded and timers are ticked
    Update,
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum CellSet {
    /// Movement
    Move,
    Produce,
    Eat,
    Attack,
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum EnvironmentSet {
    /// PrevCoords is the location of something before the update passes start
    SetPrevCoords,
    FirstGridUpdate,
    SecondGridUpdate,
    SyncTransforms,
}

#[derive(SubStates, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
#[allow(dead_code)]
#[source(Screen = Screen::Gameplay)]
enum GameState {
    #[default]
    Playing,
    Paused,
}

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<GameState>().configure_sets(
        Update,
        (GameSet::TickTimers, GameSet::RecordInput, GameSet::Update)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );

    app.configure_sets(
        Update,
        (
            EnvironmentSet::SetPrevCoords,
            EnvironmentSet::FirstGridUpdate,
            EnvironmentSet::SecondGridUpdate,
            EnvironmentSet::SyncTransforms,
        )
            .chain(),
    );

    app.configure_sets(
        Update,
        (
            CellSet::Move,
            (CellSet::Eat, CellSet::Attack),
            CellSet::Produce,
        )
            .chain(),
    );

    app.configure_sets(
        Update,
        (EnvironmentSet::SetPrevCoords, CellSet::Move).chain(),
    )
    .configure_sets(
        Update,
        (
            CellSet::Move,
            EnvironmentSet::FirstGridUpdate,
            CellSet::Eat,
            EnvironmentSet::SecondGridUpdate,
            CellSet::Produce,
        )
            .chain(),
    );

    //.configure_sets(Update, (OrganSet::Move, GameSet::SyncTransforms).chain());
    // .configure_sets(
    //     Update,
    //     (
    //         GameSet::TickTimers,
    //         (
    //             OrganSet::Move,
    //             (OrganSet::Eat, OrganSet::Attack),
    //             OrganSet::Produce,
    //         )
    //             .run_if(game_ticked)
    //             .chain(),
    //         GameSet::Despawn,
    //         GameSet::Spawn,
    //         GameSet::SyncTransforms,
    //     )
    //         .chain(),
    // );

    app.add_plugins((
        environment::plugin,
        level::plugin,
        genome::plugin,
        organism::plugin,
        cell::plugin,
        tick::plugin,
    ));
    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

pub fn game_ticked(tick: Res<GameTick>, mut prev: Local<usize>) -> bool {
    if tick.current_tick() != *prev {
        *prev = tick.current_tick();
        true
    } else {
        false
    }
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: Percent(100.0),
            height: Percent(100.0),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        StateScoped(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
