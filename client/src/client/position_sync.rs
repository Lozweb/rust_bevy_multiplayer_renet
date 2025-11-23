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
    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
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
            | ServerMessages::ErrorMessage { .. } => {}
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
    // Interpolation rapide pour les joueurs distants (30Hz serveur)
    const INTERPOLATION_SPEED: f32 = 25.0;
    // Réconciliation très douce pour le joueur local (évite saccades)
    const LOCAL_RECONCILIATION_SPEED: f32 = 3.0;
    const AIM_INTERPOLATION_SPEED: f32 = 30.0;

    // Seuil de distance pour déclencher la réconciliation du joueur local
    // Plus élevé = moins de corrections = mouvement plus fluide
    const LOCAL_RECONCILIATION_THRESHOLD: f32 = 15.0;
    // Seuil de téléportation (éviter les cas extrêmes)
    const TELEPORT_THRESHOLD: f32 = 100.0;

    let delta = time.delta_secs();

    for (networked, mut transform, mut aim_dir, is_local) in &mut players {
        let distance = transform.translation.distance(networked.target_position);

        if is_local.is_some() {
            // JOUEUR LOCAL : Réconciliation douce uniquement si divergence significative
            // Cela permet au joueur d'avoir sa physique locale tout en se synchronisant avec le serveur
            if distance > TELEPORT_THRESHOLD {
                // Divergence critique : téléporter immédiatement
                transform.translation = networked.target_position;
                trace!(
                    "Local player teleported to server position (divergence: {})",
                    distance
                );
            } else if distance > LOCAL_RECONCILIATION_THRESHOLD {
                // Divergence modérée : réconciliation douce pour éviter les saccades
                let t = (LOCAL_RECONCILIATION_SPEED * delta).min(1.0);
                transform.translation = transform.translation.lerp(networked.target_position, t);
                trace!("Local player reconciling (divergence: {})", distance);
            }
            // Sinon : divergence acceptable, la physique locale prime
        } else {
            // JOUEUR DISTANT : Interpolation normale (pas de physique locale)
            if distance > TELEPORT_THRESHOLD {
                transform.translation = networked.target_position;
            } else {
                // Interpolation de la position - suit la position serveur autoritaire
                let t = (INTERPOLATION_SPEED * delta).min(1.0);
                transform.translation = transform.translation.lerp(networked.target_position, t);
            }
        }

        // Interpolation de la direction de visée pour TOUS les joueurs
        // (gestion du wrap-around à 2π)
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
