use bevy::prelude::*;

use crate::game::GameState;

use super::{
    actions::{ChangeState, OpenLink},
    button::{ButtonColors, Mute},
    Menu,
};

/// This works because the buttons have a ButtonBundle, which includes Interactions I presume
pub fn click_button(
    // this will allow you to set the state of game state based on the change state of the menu menu button bundle
    mut next_state: ResMut<NextState<GameState>>,
    mut audio_query: Query<&mut AudioSink, With<Menu>>,
    mut button_interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
            Option<&mut Mute>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, change_state, open_link, mute) in
        &mut button_interaction_query
    {
        match *interaction {
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
            Interaction::Pressed => {
                // the play button was pressed
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open lin {error:?}");
                    }
                } else if let Some(mut mute) = mute {
                    let new_volume = if mute.0 { 0. } else { 0.2 };
                    for playback in &mut audio_query {
                        playback.set_volume(new_volume)
                    }
                    mute.0 = !mute.0;
                }
            }
        }
    }
}
