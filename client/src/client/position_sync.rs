use crate::game::player::Player;
use crate::resource::ClientLobby;
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use game_core::enemy::Enemy;
use game_core::network::{MessageDeserialize, ServerChannel};
use game_core::player::{AimDirection, ControlledPlayer};
use game_core::server::ServerMessages;
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
    mut enemies: Query<&mut NetworkedTransform, (With<Enemy>, Without<Player>)>,
) {
    while let Some(message) = client.receive_message(ServerChannel::Snapshots) {
        match ServerMessages::from_bytes(&message) {
            ServerMessages::PlayerPositionUpdate {
                client_id,
                position,
                velocity,
                aim_direction,
            } => {
                if let Some(player_entities) = lobby.get_player_entities(&client_id) {
                    if let Ok(mut networked_transform) =
                        players.get_mut(player_entities.client_entity)
                    {
                        networked_transform.target_position = position;
                        networked_transform.velocity = velocity;
                        networked_transform.target_aim_direction = aim_direction;
                        networked_transform.last_update_time = time.elapsed_secs();

                        trace!(
                            "Updated target for player {}: {:?}, aim: {}",
                            client_id, position, aim_direction
                        );
                    }
                } else {
                    trace!(
                        "Received position update for unknown player {}, ignoring",
                        client_id
                    );
                }
            }
            ServerMessages::EnemyPositions(enemies_position) => {
                for (server_entity, position) in enemies_position {
                    if let Some(client_entity) = lobby.get_enemy_entity(&server_entity)
                        && let Ok(mut networked_transform) = enemies.get_mut(*client_entity)
                    {
                        networked_transform.target_position = position;
                        networked_transform.last_update_time = time.elapsed_secs();

                        trace!(
                            "Updated target for enemy {:?}: {:?}",
                            server_entity, position
                        );
                    }
                }
            }
            _ => { /* Ignorer les autres messages */ }
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
        let target = networked.target_position;
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
    mut enemies: Query<
        (Entity, &NetworkedTransform, &mut Transform),
        (With<Enemy>, Without<Player>),
    >,
) {
    let delta = time.delta_secs();
    let enemy_count = enemies.iter().count();

    if enemy_count > 0 {
        debug!("🎬 [CLIENT] Interpolating {} enemy/enemies", enemy_count);
    }

    for (entity, networked, mut transform) in &mut enemies {
        let target = networked.target_position;
        let distance = transform.translation.distance(target);

        if distance > TELEPORT_THRESHOLD {
            info!(
                "⚡ [CLIENT] Enemy {:?} teleporting: {:?} -> {:?} (dist: {:.2})",
                entity, transform.translation, target, distance
            );
            transform.translation = target;
        } else if distance > 0.01 {
            let t = (REMOTE_INTERPOLATION_SPEED * delta).min(1.0);
            let old_pos = transform.translation;
            transform.translation = transform.translation.lerp(target, t);
            debug!(
                "🎯 [CLIENT] Enemy {:?} interpolating: {:?} -> {:?} (target: {:?}, dist: {:.2}, t: {:.3})",
                entity, old_pos, transform.translation, target, distance, t
            );
        }
    }
}
