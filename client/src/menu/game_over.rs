use crate::menu::{quit_to_title, Menu};
use crate::theme::widget;
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use game_core::client::{ClientCommand, ClientMessage};
use game_core::network::{ClientChannel, MessageSerialize};

pub(super) fn plugin(app: &mut App) {
    crate::menu::setup_menu(app, Menu::GameOver, spawn_game_over_menu);
}

fn spawn_game_over_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Game Over Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::GameOver),
        children![
            widget::header("GAME OVER"),
            Node {
                margin: UiRect::all(px(20)),
                ..default()
            },
            widget::button("Respawn", request_respawn),
            widget::button("Exit", quit_to_title),
        ],
    ));
}

fn request_respawn(
    _: On<Pointer<Click>>,
    mut client: ResMut<RenetClient>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    let message = ClientMessage::Command(ClientCommand::Respawn);
    client.send_message(ClientChannel::Input, message.to_bytes());

    info!("Respawn request sent to server");

    next_menu.set(Menu::None);
}
