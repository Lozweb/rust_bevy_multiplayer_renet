use crate::resource::ServerLobby;
use bevy::prelude::{info, MessageReader, ResMut};
use bevy_renet::renet::RenetServer;
use game_core::event::game_event::GameEvent;
use game_core::network::serialize_server_message;
use game_core::server::{ServerChannel, ServerMessages};

pub fn on_server_event(
    mut server: ResMut<RenetServer>,
    mut lobby: ResMut<ServerLobby>,
    mut game_event_reader: MessageReader<GameEvent>,
) {
    for event in game_event_reader.read() {
        match event {
            GameEvent::PlayerCreated {
                client_id,
                entity,
                position,
            } => {
                info!(
                    "PlayerCreated {:?} {:?} at position : {:?}",
                    client_id, entity, position
                );
                lobby.add_player(client_id, *entity);

                send_server_message_to_client(
                    client_id,
                    &ServerMessages::PlayerCreate {
                        client_id: *client_id,
                        position: *position,
                        entity: *entity,
                    },
                    &mut server,
                );

                broadcast_server_message(
                    &mut server,
                    &ServerMessages::PlayerCreate {
                        client_id: *client_id,
                        position: *position,
                        entity: *entity,
                    },
                );
            }
            GameEvent::PlayerRemoved { client_id } => {
                info!("PlayerRemoved {:?}", client_id);
                lobby.remove_player(client_id);

                broadcast_server_message(
                    &mut server,
                    &ServerMessages::PlayerRemove {
                        client_id: *client_id,
                    },
                );
            }
        }
    }
}

fn broadcast_server_message(server: &mut ResMut<RenetServer>, server_message: &ServerMessages) {
    let message = serialize_server_message(server_message);
    server.broadcast_message(ServerChannel::ServerMessages, message);
}

fn send_server_message_to_client(
    client_id: &u64,
    server_message: &ServerMessages,
    server: &mut ResMut<RenetServer>,
) {
    let message = serialize_server_message(server_message);
    server.send_message(*client_id, ServerChannel::ServerMessages, message);
}
