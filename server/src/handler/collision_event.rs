use crate::network::{broadcast_enemy_event, broadcast_player_event, broadcast_projectile_event};
use avian2d::prelude::CollisionStart;
use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use game_core::enemy::{ContactDamage, Enemy};
use game_core::player::{PlayerHealth, PlayerInfo};
use game_core::projectile::{Projectile, ProjectileLifeTime};
use game_core::server::{EnemyMessages, PlayerMessages, ProjectileMessages};
use std::collections::HashSet;

pub fn collision_event(
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
    time: Res<Time>,
    mut collision_reader: MessageReader<CollisionStart>,

    mut projectile_query: Query<(Entity, &Projectile, &mut ProjectileLifeTime)>,
    mut enemy_query: Query<&mut Enemy>,

    mut enemy_contact_query: Query<(Entity, &mut ContactDamage), With<Enemy>>,
    mut player_query: Query<(Entity, &mut PlayerHealth), With<PlayerInfo>>,
) {
    let mut despawned_projectiles = HashSet::new();
    let delta = time.delta();

    for collision_event in collision_reader.read() {
        let e1 = collision_event.collider1;
        let e2 = collision_event.collider2;

        let projectile_collision = if let Ok((p_ent, p, p_time)) = projectile_query.get(e1) {
            Some((p_ent, e2, p, p_time))
        } else if let Ok((p_ent, p, p_time)) = projectile_query.get(e2) {
            Some((p_ent, e1, p, p_time))
        } else {
            None
        };

        if let Some((projectile_entity, target_entity, projectile, _)) = projectile_collision {
            if target_entity == projectile.owner {
                continue;
            }

            if let Ok(mut enemy) = enemy_query.get_mut(target_entity) {
                enemy.apply_damage(projectile.damage);

                if enemy.is_dead() {
                    broadcast_enemy_event(
                        &mut server,
                        EnemyMessages::EnemyDeath {
                            server_entity: target_entity,
                        },
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
            continue;
        }

        let enemy_contact = if let Ok((enemy_ent, enemy_cd)) = enemy_contact_query.get_mut(e1) {
            info!("Enemy {:?} collided with entity {:?}", enemy_ent, e2);
            Some((enemy_ent, enemy_cd, e2))
        } else if let Ok((enemy_ent, enemy_cd)) = enemy_contact_query.get_mut(e2) {
            info!("Enemy {:?} collided with entity {:?}", enemy_ent, e1);
            Some((enemy_ent, enemy_cd, e1))
        } else {
            None
        };

        if let Some((_enemy_ent, mut contact_damage, player_entity)) = enemy_contact
            && let Ok((player_ent, mut player_health)) = player_query.get_mut(player_entity)
            && contact_damage.cooldown.is_finished()
        {
            let damage = contact_damage.damage;
            player_health.apply_damage(damage);
            info!(
                "💥 Player {:?} took {} damage (health: {})",
                player_ent, damage, player_health.current
            );

            broadcast_player_event(
                &mut server,
                PlayerMessages::PlayerDamaged {
                    player_entity: player_ent,
                    damage,
                    current_health: player_health.current,
                },
            );

            contact_damage.cooldown.reset();

            if player_health.is_dead() {
                info!("💀 Player {:?} is dead", player_ent);
                broadcast_player_event(
                    &mut server,
                    PlayerMessages::PlayerDeath {
                        player_entity: player_ent,
                    },
                )
            }
        }
    }

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
    broadcast_projectile_event(
        server,
        ProjectileMessages::ProjectileCleanup {
            server_entity: *entity,
        },
    );
    commands.entity(*entity).despawn();
}

fn mark_projectile_despawn(
    commands: &mut Commands,
    server: &mut ResMut<RenetServer>,
    entity: Entity,
    despawn_set: &mut HashSet<Entity>,
) {
    broadcast_projectile_event(
        server,
        ProjectileMessages::ProjectileCollision {
            server_entity: entity,
        },
    );
    commands.entity(entity).despawn();
    despawn_set.insert(entity);
}
