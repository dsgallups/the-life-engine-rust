use bevy::{
    color::palettes::tailwind::GREEN_400,
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::{Val::*, *},
};

use crate::utils::InteractionPalette;

pub fn game_mode<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text: String = text.into();

    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Game Mode Button"),
        Node::default(),
        BorderRadius::all(Px(8.)),
        BackgroundColor(LinearRgba::new(0., 0., 0., 0.4).into()),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Game Mode Button Inner"),
                    Button,
                    Node {
                        flex_grow: 1.,
                        flex_shrink: 0.,
                        flex_basis: Percent(30.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Px(50.)),
                        ..default()
                    },
                    Pickable::default(),
                    InteractionPalette::new(LinearRgba::new(0., 0., 0., 0.2)),
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont::from_font_size(40.0),
                        TextColor(GREEN_400.into()),
                    )],
                ))
                .observe(action);
        })),
    )
}
