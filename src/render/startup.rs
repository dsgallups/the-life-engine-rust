use bevy::{prelude::*, sprite::Anchor};

use crate::LEWorld;

use super::MousePosBox;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LEWorld>()
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>, _world: Res<LEWorld>) {
    let transform =
        Transform::from_scale(Vec3::new(0.04, 0.04, 1.)).with_translation(Vec3::new(0., -2., 100.));

    let camera = Camera2dBundle {
        transform,
        ..default()
    };

    let font = asset_server.load("fonts/fira.ttf");

    let text_style = TextStyle {
        font,
        color: Color::WHITE,
        font_size: 32.0,
        p,
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
        Text2dBundle {
            text: Text {
                sections: vec![TextSection::new("(0, 0)", text_style)],
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 1.))
                .with_scale(Vec3::new(0.2, 0.2, 1.)),
            text_anchor: Anchor::TopLeft,
            ..Default::default()
        },
        MousePosBox,
    ));

    commands.spawn(camera);
}
