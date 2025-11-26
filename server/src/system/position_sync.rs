use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use game_core::enemy::Enemy;
use game_core::network::{MessageSerialize, ServerChannel};
use game_core::player::{AimDirection, PlayerInfo};
use game_core::server::ServerMessages;

#[derive(Resource)]
pub struct PlayersPositionTimer {
    pub timer: Timer,
}

impl Default for PlayersPositionTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.033, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct EnemiesPositionTimer {
    pub timer: Timer,
}

impl Default for EnemiesPositionTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.033, TimerMode::Repeating),
        }
    }
}

pub fn broadcast_player_positions(
    time: Res<Time>,
    mut timer: ResMut<PlayersPositionTimer>,
    mut server: ResMut<RenetServer>,
    players: Query<(&PlayerInfo, &Transform, &LinearVelocity, &AimDirection)>,
) {
    timer.timer.tick(time.delta());

    if !timer.timer.just_finished() {
        return;
    }

    for (player_info, transform, velocity, aim_direction) in players.iter() {
        let update = ServerMessages::PlayerPositionUpdate {
            client_id: player_info.id,
            position: transform.translation,
            velocity: velocity.0,
            aim_direction: aim_direction.0,
        };

        server.broadcast_message(ServerChannel::Snapshots, ServerMessages::to_bytes(&update));
    }
}

pub fn broadcast_enemy_positions(
    time: Res<Time>,
    mut timer: ResMut<EnemiesPositionTimer>,
    mut server: ResMut<RenetServer>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    timer.timer.tick(time.delta());

    if !timer.timer.just_finished() {
        return;
    }

    let enemy_positions: Vec<(Entity, Vec3)> = enemies
        .iter()
        .map(|(entity, transform)| (entity, transform.translation))
        .collect();

    if !enemy_positions.is_empty() {
        let update = ServerMessages::EnemyPositions(enemy_positions);
        server.broadcast_message(ServerChannel::Snapshots, ServerMessages::to_bytes(&update));
    }
}
