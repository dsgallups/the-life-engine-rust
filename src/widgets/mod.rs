mod legacy;
pub use legacy::*;

mod game_mode_btn;
pub use game_mode_btn::*;

mod text;
pub use text::*;

pub mod nav;

mod action_btn;
pub use action_btn::*;

use bevy::prelude::*;

pub trait ClickableEvent
where
    Self: EntityEvent + Event,
{
    fn new(entity: Entity) -> Self
    where
        Self: Sized;
}

// fn on_click<'s, 'w, 't, E, B, M, I>(
//     action: I,
// ) -> (
//     impl IntoObserverSystem<E, B, M>,
//     impl Fn(On<'w, 't, Pointer<Click>>, Commands<'w, 's>),
// )
// where
//     E: ClickableEvent,
//     for<'a> <E as Event>::Trigger<'a>: Default,
//     B: Bundle,
//     I: IntoObserverSystem<E, B, M>,
// {
//     let res = |btn: On<Pointer<Click>>, mut commands: Commands| {
//         commands.trigger(E::new(btn.original_event_target()));
//     };
//     (action, res)
// }
