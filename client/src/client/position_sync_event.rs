use crate::game::player::Player;
use crate::resource::ClientLobby;
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use game_core::enemy::Enemy;
use game_core::network::{MessageDeserialize, ServerChannel};
use game_core::player::{AimDirection, ControlledPlayer};
use game_core::server::EnemyPositionMessages::EnemyPositionsUpdate;
use game_core::server::ServerUnreliableMessages::{
    EnemyPositionsEvent, ErrorMessage, PlayerPositionsEvent,
};
use game_core::server::{PlayerPositionMessages, ServerUnreliableMessages};
use game_core::NetworkedTransform;

/// Paramètres de réconciliation pour le joueur local.
const LOCAL_RECONCILIATION_SPEED: f32 = 5.0;
const LOCAL_RECONCILIATION_THRESHOLD: f32 = 15.0;

/// Paramètres pour les joueurs distants.
const REMOTE_INTERPOLATION_SPEED: f32 = 25.0;
const AIM_INTERPOLATION_SPEED: f32 = 30.0;
const TELEPORT_THRESHOLD: f32 = 100.0;

pub fn receive_position_updates(
    mut client: ResMut<RenetClient>,
    lobby: Res<ClientLobby>,
    time: Res<Time>,
    mut players: Query<&mut NetworkedTransform, With<Player>>,
    mut enemies: Query<(&mut NetworkedTransform, &mut Enemy), Without<Player>>,
) {
    while let Some(message) = client.receive_message(ServerChannel::EntitiesPosition) {
        match ServerUnreliableMessages::from_bytes(&message) {
            PlayerPositionsEvent(PlayerPositionMessages::PlayerPositionUpdate {
                client_id,
                position,
                velocity,
                aim_direction,
            }) => {
                if let Some(player_entities) = lobby.get_player_entities(&client_id)
                    && let Ok(mut networked_transform) =
                        players.get_mut(player_entities.client_entity)
                {
                    networked_transform.target_position = position;
                    networked_transform.velocity = velocity;
                    networked_transform.target_aim_direction = aim_direction;
                    networked_transform.last_update_time = time.elapsed_secs();
                }
            }
            EnemyPositionsEvent(EnemyPositionsUpdate { enemy_data }) => {
                for data in enemy_data {
                    if let Some(client_entity) = lobby.get_enemy_entity(&data.server_entity)
                        && let Ok((mut network_transform, mut enemy_component)) =
                            enemies.get_mut(*client_entity)
                    {
                        network_transform.target_position = data.position;
                        network_transform.last_update_time = time.elapsed_secs();

                        if enemy_component.health != data.health {
                            enemy_component.health = data.health;
                        }
                    }
                }
            }
            ErrorMessage { cause: reason } => {
                error!("Erreur : {}", reason);
            }
        }
    }
}

pub fn interpolate_networked_players(
    time: Res<Time>,
    mut players: Query<
        (
            &NetworkedTransform,
            &mut Transform,
            &mut AimDirection,
            Option<&ControlledPlayer>,
        ),
        With<Player>,
    >,
) {
    let delta = time.delta_secs();

    for (networked, mut transform, mut aim_dir, is_local) in &mut players {
        let current_z = transform.translation.z;
        let target = Vec3::new(
            networked.target_position.x,
            networked.target_position.y,
            current_z,
        );
        let distance = transform.translation.distance(target);

        if is_local.is_some() {
            if distance > TELEPORT_THRESHOLD {
                transform.translation = target;
            } else if distance > LOCAL_RECONCILIATION_THRESHOLD {
                let t = (LOCAL_RECONCILIATION_SPEED * delta).min(1.0);
                transform.translation = transform.translation.lerp(target, t);
            }
        } else if distance > TELEPORT_THRESHOLD {
            transform.translation = target;
        } else {
            let t = (REMOTE_INTERPOLATION_SPEED * delta).min(1.0);
            transform.translation = transform.translation.lerp(target, t);
        }

        // Direction de visée
        let current_aim = aim_dir.0;
        let target_aim = networked.target_aim_direction;
        let mut diff = target_aim - current_aim;
        if diff > std::f32::consts::PI {
            diff -= 2.0 * std::f32::consts::PI;
        } else if diff < -std::f32::consts::PI {
            diff += 2.0 * std::f32::consts::PI;
        }
        let aim_t = (AIM_INTERPOLATION_SPEED * delta).min(1.0);
        aim_dir.0 = current_aim + diff * aim_t;
    }
}

pub fn interpolate_networked_enemies(
    time: Res<Time>,
    mut enemies: Query<(&NetworkedTransform, &mut Transform), (With<Enemy>, Without<Player>)>,
) {
    let delta = time.delta_secs();

    for (networked, mut transform) in &mut enemies {
        let current_z = transform.translation.z;
        let target = Vec3::new(
            networked.target_position.x,
            networked.target_position.y,
            current_z,
        );
        let distance = transform.translation.distance(target);

        if distance > TELEPORT_THRESHOLD {
            transform.translation = target;
        } else if distance > 0.01 {
            let t = (REMOTE_INTERPOLATION_SPEED * delta).min(1.0);
            transform.translation = transform.translation.lerp(target, t);
        }
    }
}
