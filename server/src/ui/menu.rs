use crate::ui::console::ConsoleContainer;
use bevy::prelude::*;
use game_core::debug_state::DebugMode;

#[derive(Component)]
pub struct MenuText;

pub fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Text::new("Mode: Console | Appuyez sur [TAB] pour basculer"),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        MenuText,
    ));
}

pub fn handle_mode_switch(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_mode: Res<State<DebugMode>>,
    mut next_mode: ResMut<NextState<DebugMode>>,
    mut menu_text: Query<&mut Text, With<MenuText>>,
    mut console: Query<&mut Visibility, With<ConsoleContainer>>,
) {
    if keyboard.just_pressed(KeyCode::Tab) {
        let new_mode = match current_mode.get() {
            DebugMode::Console => DebugMode::Camera,
            DebugMode::Camera => DebugMode::Console,
        };

        next_mode.set(new_mode);

        if let Ok(mut text) = menu_text.single_mut() {
            text.0 = match new_mode {
                DebugMode::Console => "Mode: Console | Appuyez sur [TAB] pour basculer".to_string(),
                DebugMode::Camera => "Mode: Camera | Appuyez sur [TAB] pour basculer".to_string(),
            };
        }

        if let Ok(mut visibility) = console.single_mut() {
            *visibility = match new_mode {
                DebugMode::Console => Visibility::Visible,
                DebugMode::Camera => Visibility::Hidden,
            };
        }
    }
}
