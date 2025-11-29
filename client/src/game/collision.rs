use crate::resource::ClientLobby;
use avian2d::prelude::CollisionStart;
use bevy::prelude::{Commands, MessageReader, Query, Res};
use game_core::projectile::Projectile;
use std::collections::HashSet;

pub fn projectiles_client_cleanup(
    mut commands: Commands,
    mut collision_reader: MessageReader<CollisionStart>,
    lobby: Res<ClientLobby>,
    projectile_query: Query<&Projectile>,
) {
    let mut despawned_projectiles = HashSet::new();

    for collision_event in collision_reader.read() {
        let mut try_despawn = |entity| {
            if despawned_projectiles.contains(&entity) {
                return;
            }

            if lobby.get_projectile_entity(&entity).is_none() {
                return;
            }

            if projectile_query.get(entity).is_ok() {
                commands.entity(entity).despawn();
                despawned_projectiles.insert(entity);
            }
        };

        try_despawn(collision_event.collider1);
        try_despawn(collision_event.collider2);
    }
}
