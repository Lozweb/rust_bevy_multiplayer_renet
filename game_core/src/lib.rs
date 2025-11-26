use bevy::math::{Vec2, Vec3};
use bevy::prelude::Component;

pub mod asset_tracking;
pub mod client;
pub mod enemy;
pub mod network;
pub mod player;
pub mod server;
pub mod transport;

/// Composant stockant les informations pour l'interpolation de position.
#[derive(Component, Debug)]
pub struct NetworkedTransform {
    /// Position cible à atteindre
    pub target_position: Vec3,
    /// Vélocité du joueur (pour extrapolation)
    pub velocity: Vec2,
    /// Direction de visée cible
    pub target_aim_direction: f32,
    /// Timestamp de la dernière mise à jour reçue
    pub last_update_time: f32,
}

impl Default for NetworkedTransform {
    fn default() -> Self {
        Self {
            target_position: Vec3::ZERO,
            velocity: Vec2::ZERO,
            target_aim_direction: 0.0,
            last_update_time: 0.0,
        }
    }
}
