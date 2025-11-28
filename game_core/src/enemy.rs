use crate::NetworkedTransform;
use avian2d::prelude::*;
use bevy::color::palettes::basic::{RED, WHITE, YELLOW};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

const RESTITUTION_COEFFICIENT: f32 = 0.0;
const FRICTION_COEFFICIENT: f32 = 0.1;
const LINEAR_DAMPING: f32 = 5.0;

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct EnemyServerEntity {
    pub server_entity: Entity,
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Enemy {
    pub health: u32,
    pub enemy_type: EnemyType,
}
impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        let health = match enemy_type {
            EnemyType::Basic => 20,
            EnemyType::Medium => 30,
            EnemyType::Hard => 50,
        };
        Self { health, enemy_type }
    }

    pub fn enemy_bundle(&self, position: Vec3) -> impl Bundle {
        (
            *self,
            Transform::from_translation(position),
            RigidBody::Dynamic,
            Collider::circle(EnemyType::size(&self.enemy_type)),
            Mass(EnemyType::mass(&self.enemy_type)),
            LinearDamping(LINEAR_DAMPING),
            Friction::new(FRICTION_COEFFICIENT),
            Restitution::new(RESTITUTION_COEFFICIENT),
            NetworkedTransform::default(),
        )
    }

    pub fn apply_damage(&mut self, damage: u32) {
        if damage >= self.health {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.health == 0
    }
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub enum EnemyType {
    Basic,
    Medium,
    Hard,
}

impl EnemyType {
    pub fn size(&self) -> f32 {
        match self {
            EnemyType::Basic => 20.0,
            EnemyType::Medium => 30.0,
            EnemyType::Hard => 40.0,
        }
    }

    pub fn mass(&self) -> f32 {
        match self {
            EnemyType::Basic => 50.0,
            EnemyType::Medium => 100.0,
            EnemyType::Hard => 150.0,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            EnemyType::Basic => Color::from(WHITE),
            EnemyType::Medium => Color::from(YELLOW),
            EnemyType::Hard => Color::from(RED),
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    position: Vec3,
    enemy_type: EnemyType,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
) -> Entity {
    let mut entity_commands = commands.spawn(Enemy::new(enemy_type).enemy_bundle(position));

    if let (Some(mesh), Some(materials)) = (meshes.as_mut(), materials.as_mut()) {
        entity_commands.insert((
            Mesh2d(mesh.add(Mesh::from(Circle::new(enemy_type.size())))),
            MeshMaterial2d(materials.add(ColorMaterial::from(enemy_type.color()))),
        ));
    }

    entity_commands.id()
}
