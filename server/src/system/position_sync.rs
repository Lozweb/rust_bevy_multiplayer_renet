use crate::system::input_handler::AimDirection;
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use game_core::debug_state::Log;
use game_core::network::{MessageSerialize, ServerChannel};
use game_core::player::PlayerInfo;
use game_core::server::ServerMessages;

/// Timer pour contrôler la fréquence d'envoi des updates de position.
///
/// Configuré pour envoyer des updates 20 fois par seconde (50ms entre chaque envoi).
/// Cela réduit la charge réseau tout en maintenant une synchronisation fluide.
#[derive(Resource)]
pub struct PositionSyncTimer {
    pub timer: Timer,
}

impl Default for PositionSyncTimer {
    fn default() -> Self {
        Self {
            // 20 Hz = 50ms entre chaque update
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
        }
    }
}

/// Système qui broadcast les positions de tous les joueurs à tous les clients.
///
/// S'exécute périodiquement (contrôlé par `PositionSyncTimer`) pour :
/// - Lire la position et vélocité de chaque joueur
/// - Créer un message `PlayerPositionUpdate`
/// - L'envoyer à tous les clients sur le canal `NetworkedEntities` (unreliable)
///
/// Le canal unreliable est approprié car :
/// - La perte occasionnelle d'un paquet n'est pas critique (le prochain arrivera)
/// - Réduit la latence (pas de retransmission)
/// - Optimise la bande passante
#[allow(clippy::type_complexity)]
pub fn broadcast_player_positions(
    time: Res<Time>,
    mut timer: ResMut<PositionSyncTimer>,
    mut server: ResMut<RenetServer>,
    players: Query<(&PlayerInfo, &Transform, &LinearVelocity, &AimDirection)>,
    mut log: Option<ResMut<Log>>,
) {
    // Mettre à jour le timer
    timer.timer.tick(time.delta());

    // Envoyer uniquement quand le timer est terminé
    if !timer.timer.just_finished() {
        return;
    }

    let mut update_count = 0;

    // Pour chaque joueur
    for (player_info, transform, velocity, aim_direction) in players.iter() {
        // Créer le message de mise à jour
        let update = ServerMessages::PlayerPositionUpdate {
            client_id: player_info.id,
            position: transform.translation,
            velocity: velocity.0,
            aim_direction: aim_direction.0,
        };

        // Envoyer à tous les clients sur le canal NetworkedEntities
        server.broadcast_message(
            ServerChannel::NetworkedEntities,
            ServerMessages::to_bytes(&update),
        );

        update_count += 1;
    }

    // Log pour debugging (désactivé en prod pour performances)
    if update_count > 0 {
        trace!("Broadcasted {} position updates", update_count);

        if let Some(log) = log.as_mut() {
            log.add(
                "PositionSync".to_string(),
                game_core::debug_state::MessageDirection::Sent,
                format!("Broadcasted {} position updates", update_count),
            );
        }
    }
}
