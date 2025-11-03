use bevy::camera::Camera2d;
use bevy::prelude::Commands;

#[allow(dead_code)]
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
