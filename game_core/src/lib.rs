use bevy::math::{Vec2, Vec3};
use bevy::prelude::Component;

pub mod asset_tracking;
pub mod client;
pub mod enemy;
pub mod level;
pub mod network;
pub mod player;
pub mod projectile;
pub mod server;
pub mod transport;

#[derive(Component, Debug)]
pub struct NetworkedTransform {
    pub target_position: Vec3,
    pub velocity: Vec2,
    pub target_aim_direction: f32,
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
