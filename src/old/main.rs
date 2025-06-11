use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use life_engine_rs::GamePlugin;

fn main() -> AppExit {
    App::new()
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "The Life Engine".to_string(),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: Level::ERROR,
                    ..Default::default()
                }),
        )
        .add_plugins(GamePlugin)
        .run()
}
