#![allow(unused_variables)]

#[cfg(feature = "dev")]
mod dev_tools;

pub mod cell;

pub mod cpu_net;
pub mod genome;

pub mod organism;

pub mod camera;
pub mod game;
pub mod node_visual;
pub mod settings;
pub mod utils;
pub mod widgets;

use bevy::{prelude::*, window::WindowResolution};
use ev_core::CorePlugin;

pub fn plugin(app: &mut App) {
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

    app.add_plugins(CorePlugin);

    app.add_plugins((
        camera::plugin,
        cell::plugin,
        organism::plugin,
        node_visual::plugin,
        settings::plugin,
        utils::plugin,
        game::plugin,
    ));

    #[cfg(feature = "dev")]
    app.add_plugins(dev_tools::plugin);
}
