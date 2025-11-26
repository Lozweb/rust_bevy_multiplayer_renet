use crate::resource::server_lobby::ServerLobby;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::mesh::Mesh;
use bevy::prelude::{
    info, ColorMaterial, Commands, Entity, MessageReader, Query, ResMut, Transform, With,
};
use bevy_renet::renet::{RenetServer, ServerEvent};
use game_core::enemy::Enemy;
use game_core::player::{spawn_player, AimDirection, MovementController, PlayerInfo};
use game_core::server::ServerMessages;

pub fn on_server_event(
    players: Query<(Entity, &PlayerInfo, &Transform)>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut server: ResMut<RenetServer>,
    mut lobby: ResMut<ServerLobby>,
    mut meshes: Option<ResMut<Assets<Mesh>>>,
    mut materials: Option<ResMut<Assets<ColorMaterial>>>,
    mut server_event_reader: MessageReader<ServerEvent>,
    mut commands: Commands,
) {
    for event in server_event_reader.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                ServerMessages::client_logon(client_id);

                let quadrant = fastrand::u8(0..4);
                let position = match quadrant {
                    0 => Vec3::new(
                        fastrand::f32() * 200.0 + 200.0,
                        fastrand::f32() * 200.0 + 200.0,
                        0.0,
                    ),
                    1 => Vec3::new(
                        fastrand::f32() * 200.0 - 400.0,
                        fastrand::f32() * 200.0 + 200.0,
                        0.0,
                    ),
                    2 => Vec3::new(
                        fastrand::f32() * 200.0 + 200.0,
                        fastrand::f32() * 200.0 - 400.0,
                        0.0,
                    ),
                    _ => Vec3::new(
                        fastrand::f32() * 200.0 - 400.0,
                        fastrand::f32() * 200.0 - 400.0,
                        0.0,
                    ),
                };

                let entity = spawn_player(
                    client_id,
                    position,
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                );

                commands
                    .entity(entity)
                    .insert(MovementController::default())
                    .insert(AimDirection::default());

                lobby.add_player(client_id, entity);

                info!(
                    "PlayerCreated {:?} {:?} at position : {:?}",
                    client_id, entity, position
                );

                for (entity, player_info, transform) in players.iter() {
                    ServerMessages::send(
                        client_id,
                        &ServerMessages::PlayerCreate {
                            client_id: player_info.id,
                            position: transform.translation,
                            entity,
                        },
                        &mut server,
                    );
                }

                for (entity, transform) in enemies.iter() {
                    ServerMessages::send(
                        client_id,
                        &ServerMessages::EnemySpawned {
                            server_entity: entity,
                            position: transform.translation,
                        },
                        &mut server,
                    );
                }

                ServerMessages::broadcast(
                    &ServerMessages::PlayerCreate {
                        client_id: *client_id,
                        position,
                        entity,
                    },
                    &mut server,
                );
            }
            ServerEvent::ClientDisconnected { client_id, .. } => {
                ServerMessages::client_logoff(client_id);

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
                );
            }
        }
    }
}
