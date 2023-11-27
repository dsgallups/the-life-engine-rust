use bevy::prelude::*;

use crate::life_engine::LEWorld;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LEWorld>()
            .add_systems(Startup, (spawn_camera, init_world));
    }
}

fn spawn_camera(mut commands: Commands, world: Res<LEWorld>) {
    let x = world.width() as f32 / 2.0;
    let y = world.height() as f32 / 2.0;
    let transform =
        Transform::from_scale(Vec3::new(0.04, 0.04, 1.)).with_translation(Vec3::new(x, y, 10.));
    let camera = Camera2dBundle {
        transform,
        ..default()
    };

    commands.spawn(camera);
}

fn init_world(mut commands: Commands, mut world: ResMut<LEWorld>) {
    world.refresh_map();

    world.draw(&mut commands);
}
