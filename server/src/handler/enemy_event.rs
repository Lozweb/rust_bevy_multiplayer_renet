use crate::network::send_enemy_event;
use bevy::prelude::{Query, ResMut, Transform};
use bevy_renet::renet::{ClientId, RenetServer};
use game_core::enemy::{Enemy, EnemyServerEntity};
use game_core::server::EnemyMessages;
use tracing::info;

pub fn sending_existing_enemies(
    client_id: ClientId,
    enemies: &Query<(&Transform, &Enemy, &EnemyServerEntity)>,
    server: &mut ResMut<RenetServer>,
) {
    for (transform, enemy, entity) in enemies.iter() {
        info!(
            "Sending existing enemy to client {:?}: {:?}",
            client_id, entity.server_entity
        );
        send_enemy_event(
            client_id,
            server,
            EnemyMessages::EnemySpawned {
                server_entity: entity.server_entity,
                enemy_type: enemy.enemy_type,
                position: transform.translation,
            },
        );
    }
}
