use crate::game::player::Player;
use crate::PausableSystems;
use bevy::camera::Camera2d;
use bevy::prelude::{
    Commands, Component, IntoScheduleConfigs, Name, Query, Startup, Transform, Update, With,
    Without,
};
use game_core::player::ControlledPlayer;

#[derive(Component)]
pub struct MainCamera;

pub(super) fn plugin(app: &mut bevy::prelude::App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(Update, camera_follow.in_set(PausableSystems));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Transform::default(),
        MainCamera,
    ));
}

/// Système qui fait suivre la caméra au joueur local.
///
/// La caméra suit uniquement le joueur marqué avec `ControlledPlayer`,
/// c'est-à-dire le joueur contrôlé par ce client.
fn camera_follow(
    player_query: Query<&Transform, (With<Player>, With<ControlledPlayer>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };

    let Ok(mut camera2d) = camera_query.single_mut() else {
        return;
    };

    camera2d.translation.x = player.translation.x;
    camera2d.translation.y = player.translation.y;
}
