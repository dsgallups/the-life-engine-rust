use bevy::prelude::*;

use super::Organism;

/// Marker to find the container entity so we can show/hide the FPS counter
#[derive(Component)]
pub struct OrgCountRoot;

/// Marker to find the text entity so we can update it
#[derive(Component)]
pub struct OrgCountText;

pub fn setup_organism_counter(mut commands: Commands) {
    // create our UI root node
    // this is the wrapper/container for the text
    let root = commands
        .spawn((
            OrgCountRoot,
            NodeBundle {
                // give it a dark background for readability
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.5)),
                // make it "always on top" by setting the Z index to maximum
                // we want it to be displayed over all other UI
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    position_type: PositionType::Absolute,
                    // position it at the top-right corner
                    // 1% away from the top window edge
                    right: Val::Percent(1.),
                    top: Val::Auto,
                    // set bottom/left to Auto, so it can be
                    // automatically sized depending on the text
                    bottom: Val::Percent(1.),
                    left: Val::Auto,
                    // give it some padding for readability
                    padding: UiRect::all(Val::Px(4.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();
    // create our text
    let text_fps = commands
        .spawn((
            OrgCountText,
            TextBundle {
                // use two sections, so it is easy to update just the number
                text: Text::from_sections([
                    TextSection {
                        value: "# Organisms: ".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                    TextSection {
                        value: " N/A".into(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            // if you want to use your game's font asset,
                            // uncomment this and provide the handle:
                            // font: my_font_handle
                            ..default()
                        },
                    },
                ]),
                ..Default::default()
            },
        ))
        .id();
    commands.entity(root).push_children(&[text_fps]);
}

pub fn organism_text_update_system(
    organisms: Query<&Organism>,
    mut query: Query<&mut Text, With<OrgCountText>>,
) {
    for mut text in &mut query {
        // try to get a "smoothed" FPS value from Bevy

        let count = organisms.iter().count();

        // Format the number as to leave space for 4 digits, just in case,
        // right-aligned and rounded. This helps readability when the
        // number changes rapidly.
        text.sections[1].value = format!("{count:>8.0}");
    }
}
