// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod audio;
mod camera;
#[cfg(feature = "dev")]
mod dev;
mod gameplay;
mod loading;
mod menus;
mod screens;
mod settings;
mod splash;
mod theme;
mod title;

//mod old;

use bevy::prelude::*;

fn main() -> AppExit {
    let mut app = App::new();
    // Add Bevy plugins.
    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Window {
                title: "The Life Engine".to_string(),
                fit_canvas_to_parent: true,
                ..default()
            }
            .into(),
            ..default()
        }),
    );

    app.add_plugins((loading::plugin, splash::plugin, title::plugin));

    // Add other plugins.
    app.add_plugins((
        asset_tracking::plugin,
        audio::plugin,
        camera::plugin,
        gameplay::plugin,
        menus::plugin,
        screens::plugin,
        settings::plugin,
        theme::plugin,
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);

    // Order new `AppSystems` variants by adding them here:
    app.configure_sets(
        Update,
        (
            AppSystems::TickTimers,
            AppSystems::ChangeUi,
            AppSystems::RecordInput,
            AppSystems::Update,
        )
            .chain(),
    );

    // Set up the `Pause` state.
    app.init_state::<Pause>();
    app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

    app.run()
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    ChangeUi,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;
