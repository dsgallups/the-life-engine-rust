use bevy::prelude::*;

use crate::LEWorld;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LEWorld>()
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, _world: Res<LEWorld>) {
    let transform =
        Transform::from_scale(Vec3::new(0.04, 0.04, 1.)).with_translation(Vec3::new(0., -2., 100.));

    let camera = Camera2dBundle {
        transform,
        ..default()
    };

    commands.spawn(camera);
}
