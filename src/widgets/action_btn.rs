use bevy::{
    ecs::system::IntoObserverSystem,
    feathers::{
        controls::{ButtonProps, ButtonVariant, button},
        theme::ThemedText,
    },
    prelude::*,
};

use crate::widgets::ClickableEvent;

pub fn action_btn_shared<E, B, M, I>(
    title: impl Into<String>,
    normal: bool,
    action: I,
) -> impl Bundle
where
    E: ClickableEvent,
    for<'a> <E as Event>::Trigger<'a>: Default,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let variant = if normal {
        ButtonVariant::Normal
    } else {
        ButtonVariant::Primary
    };
    let title: String = title.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node {
            display: Display::Flex,
            flex_grow: 1.,
            ..default()
        },
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn(button(
                    ButtonProps {
                        variant,
                        ..default()
                    },
                    Pickable::default(),
                    Spawn((Text::new(title), ThemedText)),
                ))
                .observe(action)
                .observe(|btn: On<Pointer<Click>>, mut commands: Commands| {
                    commands.trigger(E::new(btn.original_event_target()));
                });
        })),
    )
}

pub fn action_btn<E: EntityEvent, B: Bundle, M, I: IntoObserverSystem<E, B, M>>(
    title: impl Into<String>,
    normal: bool,
    action: I,
) -> impl Bundle {
    let variant = if normal {
        ButtonVariant::Normal
    } else {
        ButtonVariant::Primary
    };
    let title: String = title.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node {
            display: Display::Flex,
            ..default()
        },
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn(button(
                    ButtonProps {
                        variant,
                        ..default()
                    },
                    Pickable::default(),
                    Spawn((Text::new(title), ThemedText)),
                ))
                .observe(action);
        })),
    )
}

pub fn action_btn_component<
    B: Bundle,
    E: EntityEvent,
    B2: Bundle,
    M,
    I: IntoObserverSystem<E, B2, M>,
>(
    title: impl Into<String>,
    normal: bool,
    action: I,
    b: B,
) -> impl Bundle {
    let variant = if normal {
        ButtonVariant::Normal
    } else {
        ButtonVariant::Primary
    };
    let title: String = title.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node {
            display: Display::Flex,
            ..default()
        },
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn(button(
                    ButtonProps {
                        variant,
                        ..default()
                    },
                    (b, Pickable::default()),
                    Spawn((Text::new(title), ThemedText)),
                ))
                .observe(action);
        })),
    )
}
