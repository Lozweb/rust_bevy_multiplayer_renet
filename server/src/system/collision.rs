use avian2d::prelude::CollisionStart;
use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use game_core::enemy::Enemy;
use game_core::network::ServerChannel;
use game_core::projectile::{Projectile, ProjectileLifeTime};
use game_core::server::{EnemyMessages, ProjectileMessages, ServerReliableMessages};
use std::collections::HashSet;

pub fn collision(
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
    time: Res<Time>,
    mut collision_reader: MessageReader<CollisionStart>,
    mut projectile_query: Query<(Entity, &Projectile, &mut ProjectileLifeTime)>,
    mut enemy_query: Query<&mut Enemy>,
) {
    let mut despawned_projectiles = HashSet::new();

    for collision_event in collision_reader.read() {
        let e1 = collision_event.collider1;
        let e2 = collision_event.collider2;

        let (projectile_entity, target_entity, projectile, _projectile_life_time) =
            if let Ok((p_ent, p, p_time)) = projectile_query.get(e1) {
                (p_ent, e2, p, p_time)
            } else if let Ok((p_ent, p, p_time)) = projectile_query.get(e2) {
                (p_ent, e1, p, p_time)
            } else {
                continue;
            };

        if let Ok(mut enemy) = enemy_query.get_mut(target_entity) {
            enemy.apply_damage(projectile.damage);

            if enemy.is_dead() {
                ServerReliableMessages::broadcast(
                    &ServerReliableMessages::EnemyEvent(EnemyMessages::EnemyDeath {
                        server_entity: target_entity,
                    }),
                    ServerChannel::EntityEvent,
                    &mut server,
                );
                commands.entity(target_entity).despawn();
            }
        }

        if !despawned_projectiles.contains(&projectile_entity) {
            mark_projectile_despawn(
                &mut commands,
                &mut server,
                projectile_entity,
                &mut despawned_projectiles,
            );
        }
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
    ServerReliableMessages::broadcast(
        &ServerReliableMessages::ProjectileEvent(ProjectileMessages::ProjectileCleanup {
            server_entity: *entity,
        }),
        ServerChannel::EntityEvent,
        server,
    );
    commands.entity(*entity).despawn();
}

fn mark_projectile_despawn(
    commands: &mut Commands,
    server: &mut ResMut<RenetServer>,
    entity: Entity,
    despawn_set: &mut HashSet<Entity>,
) {
    ServerReliableMessages::broadcast(
        &ServerReliableMessages::ProjectileEvent(ProjectileMessages::ProjectileCollision {
            server_entity: entity,
        }),
        ServerChannel::EntityEvent,
        server,
    );
    commands.entity(entity).despawn();
    despawn_set.insert(entity);
}
