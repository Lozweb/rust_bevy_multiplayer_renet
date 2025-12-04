use bevy::app::{App, Startup};
use bevy::prelude::*;
use game_core::level::{spawn_initial_enemies, ServerLevel};
use game_core::tiled_level::create_collision_bundle;
use game_core::tiled_parser::parse_tiled_collisions;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (setup_level_from_tiled, spawn_initial_enemies));
}

fn setup_level_from_tiled(mut commands: Commands) {
    let map_path = "assets/level/level_1.tmx";

    match parse_tiled_collisions(map_path) {
        Ok(collisions) => {
            commands
                .spawn((
                    Name::new("ServerLevel"),
                    ServerLevel,
                    Transform::default(),
                    Visibility::default(),
                ))
                .with_children(|parent| {
                    for (idx, rect) in collisions.iter().enumerate() {
                        parent.spawn(create_collision_bundle(
                            format!("Wall_{}", idx),
                            rect.clone(),
                        ));
                    }
                });

            info!(
                "✅ Server level created from Tiled map with {} colliders",
                collisions.len()
            );
        }
        Err(e) => {
            error!("❌ Failed to load Tiled map: {}", e);
            error!("Falling back to empty level");

            commands.spawn((
                Name::new("ServerLevel"),
                ServerLevel,
                Transform::default(),
                Visibility::default(),
            ));
        }
    }
}
