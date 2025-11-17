use bevy::prelude::*;

use crate::organism::Wave;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui);
    app.add_systems(Update, update_wave_text);
}

#[derive(Component)]
pub struct UiRoot;

pub fn spawn_ui(mut commands: Commands) {
    let root = commands
        .spawn((
            UiRoot,
            Node {
                width: percent(100.),
                height: percent(100.),
                ..default()
            },
        ))
        .id();

    commands.spawn((
        Node::default(),
        WaveText,
        Text::new("YOWEUIRFWOEFJI"),
        ChildOf(root),
    ));
}

#[derive(Component)]
pub struct WaveText;

fn update_wave_text(wave: Res<Wave>, mut text: Single<&mut Text, With<WaveText>>) {
    text.0 = format!("Wave {}", wave.0);
}
