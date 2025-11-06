#![allow(unused)]

use std::time::Duration;

use crate::piano::MainPiano;
use bevy::{prelude::*, time::common_conditions::once_after_delay, ui_widgets::Activate};

use crate::{
    Screen,
    game::{
        GameMode,
        map_maker::ui::{
            actions::{self, track::load::test_inner},
            left_nav::TrackInfoContainer,
        },
    },
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Splash), skip_to_load);

    app.add_systems(OnEnter(Screen::Game), skip_to_game_mode);

    // app.add_systems(
    //     Update,
    //     load_test_track.run_if(
    //         in_state(GameMode::MapMaker).and(once_after_delay(Duration::from_millis(1000))),
    //     ),
    // );
}

fn skip_to_load(mut state: ResMut<NextState<Screen>>) {
    state.set(Screen::Loading);
}

fn skip_to_game_mode(mut gs: ResMut<NextState<GameMode>>) {
    gs.set(GameMode::Dev);
}

fn load_test_track(
    commands: Commands,
    piano: Query<Entity, With<MainPiano>>,
    assets: Res<AssetServer>,
    view: Single<Entity, With<TrackInfoContainer>>,
) {
    test_inner(commands, piano, assets, view);
}

#[derive(EntityEvent)]
struct None {
    entity: Entity,
}
