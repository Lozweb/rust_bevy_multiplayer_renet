use avian2d::prelude::CollisionStart;
use bevy::prelude::{Commands, MessageReader, Query};
use game_core::projectile::Projectile;
use std::collections::HashSet;

pub fn collision(
    mut commands: Commands,
    mut collision_reader: MessageReader<CollisionStart>,
    projectile_query: Query<&Projectile>,
) {
    let mut despawned_projectiles = HashSet::new();

    for collision_event in collision_reader.read() {
        let mut try_despawn = |entity| {
            if despawned_projectiles.contains(&entity) {
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
