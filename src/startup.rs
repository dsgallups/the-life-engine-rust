use bevy::{prelude::*, utils::Uuid};

use crate::{
    organism::Organism,
    world::{ItemType, LEWorld},
};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LEWorld>()
            .add_systems(Startup, (spawn_camera, init_world));
    }
}

fn spawn_camera(mut commands: Commands) {
    let transform = Transform::from_scale(Vec3::new(0.04, 0.04, 1.));
    let camera = Camera2dBundle {
        transform,
        ..default()
    };

    commands.spawn(camera);
}

fn init_world(mut commands: Commands, world: Res<LEWorld>) {
    let width = world.width();
    let height = world.height();

    //camera spawns at 0, 0.
    //if the width is 10, then we start at -5

    let mut cursor_x = ((width / 2) as i64) - (width as i64);

    let mut cursor_y = ((height / 2) as i64) - (height as i64);

    for _ in 0..width {
        for _ in 0..height {
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.75),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    cursor_x as f32,
                    cursor_y as f32,
                    0.,
                )),
                ..default()
            });
            cursor_y += 1;
        }
        cursor_y = ((height / 2) as i64) - (height as i64);
        cursor_x += 1;
    }

    for organism in world.organisms() {
        let sprites = organism.draw();

        for sprite in sprites {
            commands.spawn((sprite, ItemType::Organism(Uuid::new_v4())));
        }
    }
}
