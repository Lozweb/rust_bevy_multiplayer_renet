use crate::audio::music;
use crate::screens::Screen;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use game_core::tiled_level::create_collision_bundle;
use game_core::tiled_parser::parse_tiled_collisions;

#[derive(Component)]
pub struct Level;

/// Marqueur pour les colliders créés côté client (pas envoyés par le serveur)
#[derive(Component)]
struct LocalCollider;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, on_map_loaded);
    app.add_systems(OnEnter(Screen::Gameplay), setup_client_collisions);
}

/// Crée les colliders côté client pour une meilleure sensation de jeu
fn setup_client_collisions(mut commands: Commands) {
    let map_path = "../assets/level/level_1.tmx";

    match parse_tiled_collisions(map_path) {
        Ok(collisions) => {
            for (idx, rect) in collisions.iter().enumerate() {
                // Créer un collider statique côté client
                // Position avec z = -100 pour être cohérent avec la map visuelle
                let mut collision_rect = rect.clone();
                collision_rect.position.z = -100.0;

                commands
                    .spawn(create_collision_bundle(
                        format!("ClientWall_{}", idx),
                        collision_rect,
                    ))
                    .insert((LocalCollider, DespawnOnExit(Screen::Gameplay)));
            }
            info!(
                "✅ [CLIENT] Created {} local collision walls for better game feel",
                collisions.len()
            );
        }
        Err(e) => {
            warn!("⚠️ [CLIENT] Failed to load client-side collisions: {}", e);
        }
    }
}

fn on_map_loaded(
    mut events: MessageReader<TiledEvent<MapCreated>>,
    assets: Res<Assets<TiledMapAsset>>,
) {
    for event in events.read() {
        if let Some(map) = event.get_map(&assets) {
            info!(
                "✅ Map Tiled chargée : {}x{} tuiles (taille: {}x{}px)",
                map.width,
                map.height,
                map.width * map.tile_width,
                map.height * map.tile_height
            );
        }
    }
}

pub fn spawn_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Level"),
            Level,
            Transform::default(),
            Visibility::default(),
            DespawnOnExit(Screen::Gameplay),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Gameplay Music"),
                music(asset_server.load("audio/music/space_ambiance.ogg")),
            ));

            parent.spawn((
                Name::new("TiledMap"),
                TiledMap(asset_server.load("level/level_1.tmx")),
                TilemapAnchor::Center,
                TiledMapLayerZOffset(-100.0),
                Transform::from_translation(Vec3::new(0.0, 0.0, -100.0)),
            ));
        });

    info!("✅ Level spawned with Tiled map");
}
