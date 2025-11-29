use bevy::prelude::*;

mod animation;
mod camera;
mod collision;
pub mod level;
pub(crate) mod movement;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        camera::plugin,
    ));
    app.add_systems(FixedUpdate, collision::projectiles_client_cleanup);
}
