#![allow(unused_variables)]

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

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The Brain Engine".to_string(),
                resolution: WindowResolution::new(1920, 1080),
                ..default()
            }),
            ..default()
        }),
        MeshPickingPlugin,
    ));

    app.insert_resource(MeshPickingSettings {
        require_markers: true,
        ..default()
    });

    app.insert_resource(UiPickingSettings {
        require_markers: true,
    });

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
