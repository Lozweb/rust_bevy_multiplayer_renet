use avian2d::prelude::*;
use bevy::prelude::*;

pub const COLLISION_LAYER_NAME: &str = "Collision";

pub const SPAWN_LAYER_NAME: &str = "Spawns";

#[derive(Debug, Clone)]
pub struct CollisionRect {
    pub position: Vec3,
    pub size: Vec2,
}

/// Crée un bundle pour un mur/obstacle avec physique statique.
///
/// Inclut tous les composants nécessaires pour :
/// - La physique (RigidBody, Collider)
/// - La visibilité des gizmos de debug (Visibility, GlobalTransform)
/// - L'identification (Name, Wall)
pub fn create_collision_bundle(name: String, rect: CollisionRect) -> impl Bundle {
    (
        Name::new(name),
        crate::level::Wall,
        Transform::from_translation(rect.position),
        GlobalTransform::default(),
        Visibility::default(),
        RigidBody::Static,
        Collider::rectangle(rect.size.x, rect.size.y),
    )
}
