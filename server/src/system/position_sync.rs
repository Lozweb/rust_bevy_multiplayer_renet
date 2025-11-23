use crate::system::input_handler::AimDirection;
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use game_core::debug_state::Log;
use game_core::network::{MessageSerialize, ServerChannel};
use game_core::player::PlayerInfo;
use game_core::server::ServerMessages;

/// Timer pour contrôler la fréquence d'envoi des updates de position (20 Hz).
#[derive(Resource)]
pub struct PositionSyncTimer {
    pub timer: Timer,
}

impl Default for PositionSyncTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.05, TimerMode::Repeating),
        }
    }
}

/// Système qui diffuse les positions de tous les joueurs à tous les clients.
///
/// S'exécute toutes les 50ms (20 Hz) sur le canal `NetworkedEntities` (unreliable)
/// pour réduire la latence et optimiser la bande passante.
pub fn broadcast_player_positions(
    time: Res<Time>,
    mut timer: ResMut<PositionSyncTimer>,
    mut server: ResMut<RenetServer>,
    players: Query<(&PlayerInfo, &Transform, &LinearVelocity, &AimDirection)>,
    mut log: Option<ResMut<Log>>,
) {
    timer.timer.tick(time.delta());

    if !timer.timer.just_finished() {
        return;
    }

    let mut update_count = 0;

    for (player_info, transform, velocity, aim_direction) in players.iter() {
        let update = ServerMessages::PlayerPositionUpdate {
            client_id: player_info.id,
            position: transform.translation,
            velocity: velocity.0,
            aim_direction: aim_direction.0,
        };

        server.broadcast_message(
            ServerChannel::NetworkedEntities,
            ServerMessages::to_bytes(&update),
        );

        update_count += 1;
    }

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
