use bevy::app::{App, Startup};
use game_core::level::{setup_level, spawn_initial_enemies};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (setup_level, spawn_initial_enemies));
}
