use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use game_core::enemy::{AiState, ContactDamage, Enemy, EnemyAi};
use game_core::player::{PlayerHealth, PlayerInfo};

#[derive(Resource)]
pub struct TargetAcquisitionTimer(pub Timer);

impl Default for TargetAcquisitionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
}

pub fn enemy_target_acquisition(
    time: Res<Time>,
    mut timer: ResMut<TargetAcquisitionTimer>,
    mut enemies: Query<(&Transform, &mut EnemyAi)>,
    players: Query<(Entity, &Transform, &PlayerHealth), With<PlayerInfo>>,
) {
    timer.0.tick(time.delta());
    if !timer.0.just_finished() {
        return;
    }

    for (enemy_transfrom, mut ai) in &mut enemies {
        let mut closest: Option<(Entity, f32)> = None;

        for (player_entity, player_transform, health) in &players {
            if health.is_dead() {
                continue;
            }

            let distance = enemy_transfrom
                .translation
                .distance(player_transform.translation);

            if distance < ai.aggro_range {
                if let Some((_, closest_dist)) = closest {
                    if distance < closest_dist {
                        closest = Some((player_entity, distance));
                    }
                } else {
                    closest = Some((player_entity, distance));
                }
            }
        }

        if let Some((target, _)) = closest {
            ai.target = Some(target);
            ai.state = AiState::Chasing;
        } else {
            ai.target = None;
            ai.state = AiState::Idle;
        }
    }
}

pub fn enemny_chase_movement(
    mut enemies: Query<(&Transform, &EnemyAi, &mut LinearVelocity), With<Enemy>>,
    targets: Query<&Transform, With<PlayerInfo>>,
) {
    for (enemy_transform, ai, mut velocity) in &mut enemies {
        match ai.state {
            AiState::Chasing => {
                if let Some(target_entity) = ai.target {
                    if let Ok(target_transform) = targets.get(target_entity) {
                        let direction = (target_transform.translation
                            - enemy_transform.translation)
                            .truncate()
                            .normalize_or_zero();

                        let target_velocity = direction * ai.move_speed;
                        velocity.0 = velocity.0.lerp(target_velocity, 0.1);
                    } else {
                        velocity.0 = velocity.0.lerp(Vec2::ZERO, 0.2);
                    }
                }
            }
            AiState::Idle => {
                velocity.0 = velocity.0.lerp(Vec2::ZERO, 0.2);
            }
        }
    }
}

pub fn tick_enemy_damage_cooldowns(
    time: Res<Time>,
    mut enemies: Query<&mut ContactDamage, With<Enemy>>,
) {
    for mut contact_damage in &mut enemies {
        contact_damage.cooldown.tick(time.delta());
    }
}
