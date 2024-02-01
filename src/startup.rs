use crate::{
    world_settings::WorldSettings, CantMove, Mouth, OrganBundle, OrganismBundle, OrganismEvent,
    Producer,
};
use bevy::prelude::*;

use super::{FpsText, MousePosBox};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldSettings>()
            .add_event::<OrganismEvent>()
            .add_systems(Startup, (spawn_camera, spawn_text, spawn_first_organism));
    }
}

fn spawn_camera(mut commands: Commands) {
    let transform =
        Transform::from_scale(Vec3::new(0.04, 0.04, 1.)).with_translation(Vec3::new(0., -2., 100.));

    let camera = Camera2dBundle {
        transform,
        ..default()
    };

    commands.spawn(camera);
}

fn spawn_first_organism(mut commands: Commands) {
    commands
        .spawn(OrganismBundle::new(CantMove, (0, 0), 3))
        .with_children(|parent| {
            parent.spawn(OrganBundle::new(Producer::new(), (1, 1)));
            parent.spawn(OrganBundle::new(Mouth, (0, 0)));
            parent.spawn(OrganBundle::new(Producer::new(), (-1, -1)));
        });

    /*let organism = Organism::simple_producer(I64Vec2::new(0, 0));

    let organ_sprites = organism
        .organs()
        .map(|organ| (SpriteBundle {
            sprite: Sprite {
                color: organ.color(),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                organ.relative_location.x as f32,
                organ.relative_location.y as f32,
                0.,
            )),
            ..default()
        })
        .collect::<Vec<_>>();

    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
            WorldLocation::new(0, 0),
            organism,
        ))
        .with_children(|parent| {
            for sprite in organ_sprites {
                parent.spawn(sprite);
            }
        });*/
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/fira.ttf");

    let text_style = TextStyle {
        font: font.clone(),
        color: Color::WHITE,
        font_size: 32.0,
    };

    commands.spawn((
        TextBundle::from_section("(0, 0)", text_style).with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        }),
        MousePosBox,
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font,
                    font_size: 50.0,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 50.0,
                color: Color::GOLD,
                ..default()
            }),
        ]),
        FpsText,
    ));
}
