use std::iter;

use crate::widgets::{self, ClickableEvent};
use bevy::{color::palettes::css::NAVY, feathers::palette::WHITE, prelude::*};
use bevy_ui_text_input::{TextInputBuffer, TextInputMode, TextInputNode, TextInputPrompt};
use cosmic_text::{Cursor, Edit};

pub fn text<C: Bundle>(text: impl Into<String>, fs: f32, ident: C) -> impl Bundle {
    (Text::new(text), TextFont::from_font_size(fs), ident)
}

#[derive(Component)]
pub struct TextInputContainer;

pub fn text_input<C: Bundle>(header_txt: impl Into<String>, ident: C) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        TextInputContainer,
        children![widgets::nav::section_header(header_txt), title_inner(ident)],
    )
}

fn title_inner<C: Bundle>(ident: C) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            //todo
            parent.spawn(input_node(ident)).observe(show_action_buttons);
        })),
    )
}
fn def_input_node() -> TextInputNode {
    TextInputNode {
        mode: TextInputMode::SingleLine,
        clear_on_submit: true,
        unfocus_on_submit: true,
        ..default()
    }
}

fn input_node<C: Bundle>(ident: C) -> impl Bundle {
    (
        def_input_node(),
        TextInputPrompt::default(),
        ident,
        Node {
            flex_basis: px(300.),
            height: px(25.),
            ..default()
        },
        TextFont::from_font_size(20.),
        TextColor(WHITE),
        BackgroundColor(NAVY.into()),
    )
}

#[derive(Component)]
#[relationship_target(relationship = InputActionOf)]
pub struct InputActions(Vec<Entity>);

#[derive(Component)]
#[relationship(relationship_target = InputActions)]
pub struct InputActionOf(pub Entity);

fn show_action_buttons(
    ev: On<Pointer<Press>>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    containers: Query<Option<&InputActions>, With<TextInputContainer>>,
) {
    info!("INPUT PRESSED");
    let ev = ev.event_target();
    let Some(parent) = parents.iter_ancestors(ev).find(|parent| {
        let Ok(actions) = containers.get(*parent) else {
            //doesn't have text_input_container
            return false;
        };
        // is a container without any actions
        actions.is_none_or(|actions| actions.is_empty())
    }) else {
        return;
    };
    //commands.add_observer(remove_cancel_button);

    let cancel_btn = widgets::action_btn_shared("Cancel", true, remove_cancel_button);
    commands.spawn((cancel_btn, InputActionOf(parent), ChildOf(parent)));
}

#[derive(EntityEvent)]
pub struct RemoveCancelButton {
    pub entity: Entity,
}
impl ClickableEvent for RemoveCancelButton {
    fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

pub fn remove_cancel_button(
    btn: On<RemoveCancelButton>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    children: Query<&Children>,
    containers: Query<(), With<TextInputContainer>>,
    mut input_node: Query<&mut TextInputBuffer, With<TextInputNode>>,
) {
    let target = btn.event_target();
    let Some(parent) = iter::once(target)
        .chain(parents.iter_ancestors(target))
        .find(|parent| containers.get(*parent).is_ok())
    else {
        return;
    };
    commands.entity(parent).despawn_related::<InputActions>();

    let Some(input_node_e) = children
        .iter_descendants(parent)
        .find(|child| input_node.get(*child).is_ok())
    else {
        return;
    };
    let Ok(mut input_node) = input_node.get_mut(input_node_e) else {
        error!("Input node doesn't have input node? wat");
        return;
    };
    let s_c = Cursor::new(0, 0);
    let s_e = input_node.editor.cursor();

    input_node.editor.delete_range(s_c, s_e);
    commands.entity(input_node_e).insert(def_input_node());
}
