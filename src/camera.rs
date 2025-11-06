use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    //todo
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
    ));
}
