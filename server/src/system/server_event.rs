use crate::resource::server_lobby::ServerLobby;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::mesh::Mesh;
use bevy::prelude::{
    info, ColorMaterial, Commands, Entity, MessageReader, Query, ResMut, Transform,
};
use bevy_renet::renet::{RenetServer, ServerEvent};
use game_core::debug_state::MessageLog;
use game_core::player::{spawn_player, PlayerInfo};
use game_core::server::ServerMessages;

#[allow(dead_code, clippy::too_many_arguments)]
pub fn on_server_event(
    mut players: Query<(Entity, &PlayerInfo, &Transform)>,
    mut server: ResMut<RenetServer>,
    mut lobby: ResMut<ServerLobby>,
    mut log: ResMut<MessageLog>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut server_event_reader: MessageReader<ServerEvent>,
    mut commands: Commands,
) {
    for event in server_event_reader.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                ServerMessages::client_logon(client_id, &mut log);

                let position = Vec3::new(fastrand::f32() * 800.0 - 400.0, 0.0, 0.0);

                let entity = spawn_player(
                    client_id,
                    position,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );

                lobby.add_player(client_id, entity);

                info!(
                    "PlayerCreated {:?} {:?} at position : {:?}",
                    client_id, entity, position
                );

                for (entity, player_info, transform) in players.iter_mut() {
                    ServerMessages::send(
                        client_id,
                        &ServerMessages::PlayerCreate {
                            client_id: player_info.id,
                            position: transform.translation,
                            entity,
                        },
                        &mut server,
                        &mut log,
                    );
                }
                ServerMessages::broadcast(
                    &ServerMessages::PlayerCreate {
                        client_id: *client_id,
                        position,
                        entity,
                    },
                    &mut server,
                    &mut log,
                );
            }
            ServerEvent::ClientDisconnected { client_id, .. } => {
                ServerMessages::client_logoff(client_id, &mut log);

                if let Some(entity) = lobby.get_player(client_id) {
                    commands.entity(*entity).despawn();
                    lobby.remove_player(client_id);
                    info!("PlayerRemoved {:?}", client_id);
                }
                ServerMessages::broadcast(
                    &ServerMessages::PlayerRemove {
                        client_id: *client_id,
                    },
                    &mut server,
                    &mut log,
                );
            }
        }
    }
}
