use crate::game::player::Player;
use crate::resource::ClientLobby;
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use game_core::network::{MessageDeserialize, ServerChannel};
use game_core::player::{AimDirection, ControlledPlayer};
use game_core::server::ServerMessages;

/// Composant stockant les informations pour l'interpolation de position.
#[derive(Component, Debug)]
pub struct NetworkedTransform {
    /// Position cible à atteindre
    pub target_position: Vec3,
    /// Vélocité du joueur (pour extrapolation)
    pub velocity: Vec2,
    /// Direction de visée cible
    pub target_aim_direction: f32,
    /// Timestamp de la dernière mise à jour reçue
    pub last_update_time: f32,
}

impl Default for NetworkedTransform {
    fn default() -> Self {
        Self {
            target_position: Vec3::ZERO,
            velocity: Vec2::ZERO,
            target_aim_direction: 0.0,
            last_update_time: 0.0,
        }
    }
}

/// Paramètres de réconciliation pour le joueur local.
const LOCAL_RECONCILIATION_SPEED: f32 = 5.0;
const LOCAL_RECONCILIATION_THRESHOLD: f32 = 15.0;

/// Paramètres pour les joueurs distants.
const REMOTE_INTERPOLATION_SPEED: f32 = 25.0;
const AIM_INTERPOLATION_SPEED: f32 = 30.0;
const TELEPORT_THRESHOLD: f32 = 100.0;

/// Système qui reçoit et applique les mises à jour de position du serveur.
///
/// IMPORTANT : Met à jour TOUS les joueurs (y compris le local) depuis le serveur.
/// Architecture full server-authoritative pour zéro désynchronisation.
pub fn receive_position_updates(
    mut client: ResMut<RenetClient>,
    lobby: Res<ClientLobby>,
    time: Res<Time>,
    mut players: Query<&mut NetworkedTransform, With<Player>>,
) {
    while let Some(message) = client.receive_message(ServerChannel::Snapshots) {
        match ServerMessages::from_bytes(&message) {
            ServerMessages::PlayerPositionUpdate {
                client_id,
                position,
                velocity,
                aim_direction,
            } => {
                // Mettre à jour TOUS les joueurs (local ET distants)
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
            ServerMessages::PlayerCreate { .. }
            | ServerMessages::PlayerRemove { .. }
            | ServerMessages::ErrorMessage { .. }
            | ServerMessages::CriticalEvent(_) => {}
        }
    }
}

/// Système d'interpolation qui rend les mouvements des joueurs distants fluides.
///
/// IMPORTANT : Les joueurs distants n'ont PAS de physique côté client.
/// Ce système déplace directement leur Transform vers la position autoritaire du serveur.
///
/// Le joueur local utilise une RÉCONCILIATION DOUCE : si la divergence avec le serveur
/// est trop grande (>5 unités), on corrige progressivement pour éviter les saccades tout
/// en maintenant la synchronisation.
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
