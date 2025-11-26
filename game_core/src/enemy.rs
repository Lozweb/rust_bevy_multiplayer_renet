use crate::NetworkedTransform;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Reflect)]
#[reflect(Component)]
pub struct Enemy {
    pub server_entity: Entity,
}

pub fn spawn_enemy(
    commands: &mut Commands,
    position: Vec3,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
) -> Entity {
    use avian2d::prelude::*;
    let mut entity_commands = commands.spawn((
        Transform::from_translation(position),
        RigidBody::Dynamic,
        Collider::circle(20.0),
        Mass(100.0),
        LinearDamping(2.0),
        Friction::new(0.1),
        Restitution::new(0.0),
        NetworkedTransform::default(),
    ));

    if let (Some(mesh), Some(materials)) = (meshes.as_mut(), materials.as_mut()) {
        entity_commands.insert((
            Mesh2d(mesh.add(Mesh::from(Circle::new(20.0)))),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
        ));
    }

    entity_commands.id()
}
