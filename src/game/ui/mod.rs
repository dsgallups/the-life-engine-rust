use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui);
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

    commands.spawn((Node::default(), Text::new("YOWEUIRFWOEFJI"), ChildOf(root)));
}
