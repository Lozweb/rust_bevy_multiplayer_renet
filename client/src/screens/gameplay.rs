use crate::client::message_routing::on_client_event;
use crate::client::position_sync::*;
use crate::client::Connected;
use crate::game::level::spawn_level;
use crate::menu::Menu;
use crate::resource::{ClientLobby, CurrentClientId};
use crate::screens::Screen;
use crate::Pause;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_renet::client_connected;
use bevy_renet::netcode::NetcodeClientTransport;
use bevy_renet::renet::RenetClient;
use game_core::network::connection_config;
use game_core::transport::setup_client_transport;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (connect_to_server, spawn_level).chain(),
    );

    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(
        OnExit(Screen::Gameplay),
        (close_menu, unpause, disconnect_from_server),
    );
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(Update, on_client_event.in_set(Connected));

    app.add_systems(
        Update,
        (
            receive_position_updates,
            interpolate_networked_players,
            interpolate_networked_enemies,
        )
            .chain()
            .run_if(client_connected),
    );
    app.configure_sets(Update, Connected.run_if(client_connected));
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DespawnOnExit(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn connect_to_server(mut commands: Commands) {
    info!("Connecting to server...");

    let client = RenetClient::new(connection_config());
    let transport = setup_client_transport("127.0.0.1", 5000);

    commands.insert_resource(CurrentClientId(transport.client_id()));
    commands.insert_resource(client);
    commands.insert_resource(transport);

    info!("Connection initiated");
}

fn disconnect_from_server(mut commands: Commands, mut lobby: ResMut<ClientLobby>) {
    info!("Disconnecting from server...");

    lobby.players.clear();
    commands.remove_resource::<RenetClient>();
    commands.remove_resource::<NetcodeClientTransport>();
    commands.remove_resource::<CurrentClientId>();

    info!("Disconnected from server");
}
