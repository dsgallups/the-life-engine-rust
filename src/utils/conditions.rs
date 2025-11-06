#![expect(dead_code)]

use bevy::prelude::*;

fn not_in_state<S: States>(state: S) -> impl FnMut(Option<Res<State<S>>>) -> bool + Clone {
    move |current_state: Option<Res<State<S>>>| match current_state {
        Some(current_state) => *current_state != state,
        None => false,
    }
}
