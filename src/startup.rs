use crate::{
    world_settings::WorldSettings, OrganBundle, OrganType, OrganismBundle, OrganismInfo,
    OrganismType, Reproduce,
};
use bevy::prelude::*;

use super::{FpsText, MousePosBox};

#[derive(Event)]
pub struct RemoveFood(pub Entity);

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldSettings>()
            .add_event::<Reproduce>()
            .add_event::<RemoveFood>()
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
        .spawn(OrganismBundle::new(
            OrganismType::Producer,
            (0, 0),
            OrganismInfo::new(3),
        ))
        .with_children(|parent| {
            parent.spawn(OrganBundle::new(OrganType::Producer, (1, 1)));
            parent.spawn(OrganBundle::new(OrganType::Mouth, (0, 0)));
            parent.spawn(OrganBundle::new(OrganType::Producer, (-1, -1)));
        });
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
