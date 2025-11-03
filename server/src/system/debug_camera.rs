use bevy::prelude::*;

#[derive(Component)]
pub struct DebugCamera;

pub fn update_debug_camera(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Query<&mut Transform, With<DebugCamera>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = camera.single_mut() {
        let speed = 300.0 * time.delta_secs();

        if keyboard.pressed(KeyCode::ArrowUp) {
            transform.translation.y += speed;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= speed;
        }
        if keyboard.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= speed;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            transform.translation.x += speed;
        }
    }
}
pub fn setup_debug_camera(mut commands: Commands) {
    commands.spawn((Camera2d, DebugCamera));
}
