use crate::handler::enemy_event::sending_existing_enemies;
use crate::network::{broadcast_player_event, send_player_event};
use crate::resource::server_lobby::ServerLobby;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::mesh::Mesh;
use bevy::prelude::{ColorMaterial, Commands, Entity, Query, ResMut, Transform};
use bevy_renet::renet::{ClientId, RenetServer};
use game_core::enemy::{Enemy, EnemyServerEntity};
use game_core::player::{spawn_player, AimDirection, MovementController, PlayerHealth, PlayerInfo};
use game_core::server::PlayerMessages;
use tracing::info;

pub fn client_connected(
    client_id: ClientId,
    commands: &mut Commands,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
    server: &mut ResMut<RenetServer>,
    lobby: &mut ServerLobby,
    players: Query<(Entity, &PlayerInfo, &Transform)>,
    enemies: Query<(&Transform, &Enemy, &EnemyServerEntity)>,
) {
    let position = rand_position();
    let server_entity = spawn_player(&client_id, position, commands, meshes, materials);

    commands
        .entity(server_entity)
        .insert(MovementController::default())
        .insert(AimDirection::default())
        .insert(PlayerHealth::default());

    lobby.add_player(&client_id, server_entity);

    sending_existing_players(client_id, &players, server);
    sending_existing_enemies(client_id, &enemies, server);

    info!(
        "New Client connected : PlayerCreated {:?} {:?} at position : {:?}",
        client_id, server_entity, position
    );

    broadcast_player_event(
        server,
        PlayerMessages::PlayerCreate {
            server_entity,
            client_id,
            position,
        },
    );
}

pub fn client_disconnected(
    client_id: ClientId,
    lobby: &mut ServerLobby,
    commands: &mut Commands,
    server: &mut ResMut<RenetServer>,
) {
    if let Some(entity) = lobby.get_player(&client_id) {
        commands.entity(*entity).despawn();
        lobby.remove_player(&client_id);
        info!("PlayerRemoved {:?}", client_id);
    }

    broadcast_player_event(server, PlayerMessages::PlayerRemove { client_id });
}

fn sending_existing_players(
    client_id: ClientId,
    players: &Query<(Entity, &PlayerInfo, &Transform)>,
    server: &mut ResMut<RenetServer>,
) {
    for (server_entity, player_info, transform) in players.iter() {
        info!(
            "Sending existing player to client {:?}: {:?}",
            client_id, player_info.id
        );

        send_player_event(
            client_id,
            server,
            PlayerMessages::PlayerCreate {
                server_entity,
                client_id: player_info.id,
                position: transform.translation,
            },
        );
    }
}
fn rand_position() -> Vec3 {
    let quadrant = fastrand::u8(0..4);
    match quadrant {
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
    }
}
