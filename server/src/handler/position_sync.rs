use crate::network::{broadcast_enemy_position, broadcast_player_position};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use game_core::enemy::{EnemiesPositionTimer, Enemy};
use game_core::player::{AimDirection, PlayerInfo, PlayersPositionTimer};
use game_core::server::{EnemyPositionMessages, NetworkedEnemyData, PlayerPositionMessages};

pub fn sync_players_position(
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
        broadcast_player_position(
            &mut server,
            PlayerPositionMessages::PlayerPositionUpdate {
                client_id: player_info.id,
                position: transform.translation,
                velocity: velocity.0,
                aim_direction: aim_direction.0,
            },
        );
    }
}

pub fn sync_enemies_positions(
    time: Res<Time>,
    mut timer: ResMut<EnemiesPositionTimer>,
    mut server: ResMut<RenetServer>,
    enemies: Query<(Entity, &Transform, &Enemy), With<Enemy>>,
) {
    timer.timer.tick(time.delta());

    if !timer.timer.just_finished() {
        return;
    }

    let enemy_data: Vec<NetworkedEnemyData> = enemies
        .iter()
        .map(|(entity, transform, enemy)| NetworkedEnemyData {
            server_entity: entity,
            position: transform.translation,
            health: enemy.health,
        })
        .collect();

    if !enemy_data.is_empty() {
        broadcast_enemy_position(
            &mut server,
            EnemyPositionMessages::EnemyPositionsUpdate {
                enemy_data: enemy_data.clone(),
            },
        );
    }
}
