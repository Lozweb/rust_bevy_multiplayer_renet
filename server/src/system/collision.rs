use avian2d::prelude::CollisionStart;
use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use game_core::projectile::{Projectile, ProjectileLifeTime};
use game_core::server::ServerMessages;
use std::collections::HashSet;

pub fn collision(
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
    time: Res<Time>,
    mut collision_reader: MessageReader<CollisionStart>,
    mut projectile_query: Query<(Entity, &Projectile, &mut ProjectileLifeTime)>,
) {
    let mut despawned_projectiles = HashSet::new();

    for collision_event in collision_reader.read() {
        let mut try_despawn = |entity: Entity| {
            if despawned_projectiles.contains(&entity) {
                return;
            }
            if projectile_query.contains(entity) {
                handle_collision_event(&mut commands, &mut server, &entity);
                despawned_projectiles.insert(entity);
            }
        };

        try_despawn(collision_event.collider1);
        try_despawn(collision_event.collider2);
    }

    let delta = time.delta();

    for (entity, _projectile, mut lifetime) in &mut projectile_query {
        if despawned_projectiles.contains(&entity) {
            continue;
        }

        lifetime.timer.tick(delta);

        if lifetime.timer.just_finished() {
            cleanup_expired_projectiles(&mut commands, &mut server, &entity);
            despawned_projectiles.insert(entity);
        }
    }
}

fn cleanup_expired_projectiles(
    commands: &mut Commands,
    server: &mut ResMut<RenetServer>,
    entity: &Entity,
) {
    info!("Cleaning up expired projectiles");
    ServerMessages::broadcast(
        &ServerMessages::ProjectileCleanup {
            server_entity: *entity,
        },
        server,
    );
    commands.entity(*entity).despawn();
}

fn handle_collision_event(
    commands: &mut Commands,
    server: &mut ResMut<RenetServer>,
    entity: &Entity,
) {
    ServerMessages::broadcast(
        &ServerMessages::ProjectileCollision {
            server_entity: *entity,
        },
        server,
    );
    commands.entity(*entity).despawn();
}
