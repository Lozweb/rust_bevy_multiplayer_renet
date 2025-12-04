use crate::player::AimDirection;
use avian2d::prelude::{
    Collider, CollisionEventsEnabled, Friction, LinearDamping, LinearVelocity, LockedAxes, Mass,
    Restitution, RigidBody,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Reflect, Component)]
#[reflect(Component)]
pub struct Projectile {
    pub damage: u32,
    pub owner: Entity,
}

#[derive(Debug, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct ProjectileLifeTime {
    pub timer: Timer,
}

impl ProjectileLifeTime {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

impl Projectile {
    pub fn projectil_bundle(&self, position: Vec3, aim_direction: AimDirection) -> impl Bundle {
        let speed = 2400.0;
        let spawn_distance = 40.0;
        let dir = Vec3::new(aim_direction.0.cos(), aim_direction.0.sin(), 0.0);
        let spawn_pos_2d = position + dir * spawn_distance;
        let spawn_pos = Vec3::new(spawn_pos_2d.x, spawn_pos_2d.y, 10.0);

        (
            *self,
            Transform::from_translation(spawn_pos),
            RigidBody::Dynamic,
            Collider::circle(5.),
            Mass(20.),
            LinearVelocity(Vec2::new(
                aim_direction.0.cos() * speed,
                aim_direction.0.sin() * speed,
            )),
            LinearDamping(1.5),
            Friction::new(0.1),
            Restitution::new(0.0),
            LockedAxes::ROTATION_LOCKED,
            CollisionEventsEnabled,
            ProjectileLifeTime::new(2.0),
        )
    }
}

pub fn spawn_projectil(
    projectile: &Projectile,
    position: Vec3,
    aim_direction: AimDirection,
    commands: &mut Commands,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
) -> Entity {
    let mut entity_commands = commands.spawn(projectile.projectil_bundle(position, aim_direction));

    if let (Some(meshes), Some(materials)) = (meshes.as_mut(), materials.as_mut()) {
        entity_commands.insert((
            Mesh2d(meshes.add(Mesh::from(Circle::new(5.)))),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
        ));
    }

    entity_commands.id()
}
