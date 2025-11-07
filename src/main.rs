#[cfg(feature = "dev")]
mod dev_tools;

mod camera;
mod game;
mod settings;
mod utils;
mod widgets;

use bevy::{prelude::*, window::WindowResolution};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(1920, 1080),
            ..default()
        }),
        ..default()
    }));

    app.add_plugins((
        camera::plugin,
        settings::plugin,
        utils::plugin,
        game::plugin,
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(dev_tools::plugin);

    app.run();
}
