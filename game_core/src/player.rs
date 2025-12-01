use bevy::asset::Assets;
use bevy::math::{Vec2, Vec3};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::ReflectResource;
use bevy::prelude::{
    Circle, ColorMaterial, Commands, Component, Deref, Entity, MeshMaterial2d, Reflect, ResMut,
    Resource, Transform, Visibility,
};
use bevy::prelude::{ReflectComponent, Timer, TimerMode};
use bevy_renet::renet::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Resource)]
pub struct PlayersPositionTimer {
    pub timer: Timer,
}

impl Default for PlayersPositionTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.033, TimerMode::Repeating),
        }
    }
}
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub aim_direction: f32,
    pub shoot: bool,
}

#[derive(Component, Resource, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component, Resource)]
pub struct AimDirection(pub f32);

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    pub intent: Vec2,
    pub target_intent: Vec2,
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            target_intent: Vec2::ZERO,
            max_speed: 400.0,
        }
    }
}

#[derive(Resource, Debug, Default, Deref)]
pub struct MouseWorldCoords(pub Option<Vec2>);

#[derive(Debug, Component)]
pub struct PlayerInfo {
    pub id: ClientId,
    pub name: String,
}

#[derive(Component)]
pub struct ControlledPlayer;

pub fn spawn_player(
    client_id: &ClientId,
    position: Vec3,
    commands: &mut Commands,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
) -> Entity {
    use avian2d::prelude::*;

    let mut entity_commands = commands.spawn((
        Transform::from_translation(position),
        Visibility::default(),
        PlayerInfo {
            id: *client_id,
            name: format!("Player_{client_id}"),
        },
        RigidBody::Dynamic,
        Collider::rectangle(32.0, 32.0),
        Mass(50.0),
        LinearDamping(1.5),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.1),
        Restitution::new(0.0),
    ));

    if let (Some(meshes), Some(materials)) = (meshes.as_mut(), materials.as_mut()) {
        entity_commands.insert((
            Mesh2d(meshes.add(Mesh::from(Circle::new(40.0)))),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
        ));
    }

    entity_commands.id()
}
