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

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum GameSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
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
        (
            GameSystems::TickTimers,
            GameSystems::RecordInput,
            GameSystems::Update,
        )
            .chain()
            .run_if(in_state(GameState::Playing)),
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
