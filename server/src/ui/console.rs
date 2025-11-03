use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use game_core::debug_state::{Log, MessageDirection};
use log::info;

#[derive(Component)]
pub struct ConsoleContainer;

#[derive(Component)]
pub struct ConsoleContent;

#[derive(Resource, Default)]
pub struct ConsoleScrollState {
    pub auto_scroll: bool,
}

pub fn setup_console_ui(mut commands: Commands) {
    commands.insert_resource(ConsoleScrollState { auto_scroll: true });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(200.0),
                overflow: Overflow::scroll_y(),
                border: UiRect::all(Val::Px(2.0)),
                padding: UiRect::all(Val::Px(5.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.05)),
            BorderColor::all(Color::srgb(0.05, 0.05, 0.05)),
            Visibility::Visible,
            Interaction::default(),
            Pickable::IGNORE,
            ConsoleContainer,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ConsoleContent,
            ));
        });
}

pub fn update_console_display(log: Res<Log>, mut console: Query<&mut Text, With<ConsoleContent>>) {
    if !log.is_changed() {
        return;
    }

    if let Ok(mut text) = console.single_mut() {
        let lines: Vec<String> = log
            .entries
            .iter()
            .map(|entry| {
                let dir_symbol = match entry.direction {
                    MessageDirection::Sent => "->",
                    MessageDirection::Received => "<-",
                };

                format!(
                    "[{}] {} [{}] {}",
                    entry.formatted_timestamp(),
                    dir_symbol,
                    entry.channel,
                    entry.content
                )
            })
            .collect();

        **text = lines.join("\n");
    }
}

pub fn auto_scroll_console(
    scroll_state: Res<ConsoleScrollState>,
    mut container: Query<(&mut ScrollPosition, &Node, &ComputedNode), With<ConsoleContainer>>,
) {
    if !scroll_state.auto_scroll {
        return;
    }

    if let Ok((mut scroll_position, _node, computed)) = container.single_mut() {
        let max_scroll =
            (computed.content_size().y - computed.size().y) * computed.inverse_scale_factor();
        scroll_position.y = max_scroll.max(0.0);
    }
}

pub fn handle_console_input(keyboard: Res<ButtonInput<KeyCode>>, mut log: ResMut<Log>) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        log.clear();
        info!("Console cleared");
    }
}

#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
pub struct Scroll {
    entity: Entity,
    delta: Vec2,
}

const LINE_HEIGHT: f32 = 21.;

pub fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    console_query: Query<(Entity, &Interaction), With<ConsoleContainer>>,
    mut commands: Commands,
) {
    let Ok((console_entity, interaction)) = console_query.single() else {
        return;
    };

    if *interaction != Interaction::Hovered {
        return;
    }

    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);

        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= LINE_HEIGHT;
        }

        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }

        commands.trigger(Scroll {
            entity: console_entity,
            delta,
        });
    }
}

const SCROLL_BOTTOM_THRESHOLD: f32 = 5.0;

pub fn on_scroll_handler(
    mut scroll: On<Scroll>,
    mut scroll_state: ResMut<ConsoleScrollState>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode), With<ConsoleContainer>>,
) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();

    let delta = &mut scroll.delta;

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
        scroll_state.auto_scroll = false;

        let max = if delta.y > 0. {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.
        };

        if !max {
            scroll_position.y += delta.y;
            delta.y = 0.;
        }

        if (scroll_position.y - max_offset.y).abs() < SCROLL_BOTTOM_THRESHOLD {
            scroll_state.auto_scroll = true;
        }
    }

    if *delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}
