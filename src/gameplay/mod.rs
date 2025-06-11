//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::{Val::*, *},
};

use crate::{Pause, menus::Menu, screens::Screen};

mod cell;
mod genome;
mod level;
mod organism;
mod world;

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum GameSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,

    /// Things irrelevant to the state of the game that need to happen after input is recorded and timers are ticked
    Update,

    /// Movement
    Move,
    Produce,
    Eat,
    Attack,

    Spawn,
    Despawn,

    /// sync transforms last
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
    app.add_sub_state::<GameState>()
        .configure_sets(
            Update,
            (
                GameSet::TickTimers,
                GameSet::RecordInput,
                GameSet::Update,
                GameSet::SyncTransforms,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .configure_sets(
            Update,
            (
                GameSet::Eat,
                GameSet::Move,
                GameSet::Attack,
                GameSet::Produce,
                GameSet::Despawn,
                GameSet::Spawn,
                GameSet::SyncTransforms,
            )
                .chain(),
        );

    app.add_plugins((
        world::plugin,
        level::plugin,
        genome::plugin,
        organism::plugin,
        cell::plugin,
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
