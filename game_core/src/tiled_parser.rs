use crate::tiled_level::CollisionRect;
use bevy::prelude::*;
use std::path::Path;
use tiled::Loader;

/// Parse une map Tiled et extrait les rectangles de collision.
///
/// Cette fonction est utilisée côté client ET serveur pour garantir
/// que les collisions sont identiques des deux côtés.
///
/// # Arguments
/// * `map_path` - Chemin vers le fichier .tmx
///
/// # Returns
/// * `Ok(Vec<CollisionRect>)` - Liste des rectangles de collision
/// * `Err(String)` - Message d'erreur si le parsing échoue
pub fn parse_tiled_collisions(map_path: &str) -> Result<Vec<CollisionRect>, String> {
    // Charger le fichier TMX
    let mut loader = Loader::new();
    let map = loader
        .load_tmx_map(Path::new(map_path))
        .map_err(|e| format!("Failed to parse TMX: {:?}", e))?;

    let mut collisions = Vec::new();

    let tile_width = map.tile_width as f32;
    let tile_height = map.tile_height as f32;
    let map_width = map.width;
    let map_height = map.height;

    // Parcourir tous les calques
    for layer in map.layers() {
        // Vérifier si c'est un calque de tuiles
        if let Some(tile_layer) = layer.as_tile_layer() {
            // Parcourir toutes les tuiles
            for y in 0..tile_layer.height().unwrap() {
                for x in 0..tile_layer.width().unwrap() {
                    if let Some(tile) = tile_layer.get_tile(x as i32, y as i32) {
                        let tile_id = tile.id();

                        // Liste des IDs de tuiles considérées comme obstacles
                        // IMPORTANT: Cette liste doit rester synchronisée pour client/serveur
                        if is_collision_tile(tile_id) {
                            // Position en pixels (origine en bas à gauche pour Bevy)
                            let pixel_x = x as f32 * tile_width;
                            let pixel_y = (map_height - y - 1) as f32 * tile_height;

                            // Centrer la map (origine au centre)
                            let center_x =
                                pixel_x - (map_width as f32 * tile_width / 2.0) + tile_width / 2.0;
                            let center_y = pixel_y - (map_height as f32 * tile_height / 2.0)
                                + tile_height / 2.0;

                            collisions.push(CollisionRect {
                                position: Vec3::new(center_x, center_y, 0.0),
                                size: Vec2::new(tile_width, tile_height),
                            });
                        }
                    }
                }
            }
        }
    }

    info!(
        "✅ Parsed {} collision tiles from map {}",
        collisions.len(),
        map_path
    );

    Ok(collisions)
}

/// Détermine si un ID de tuile doit créer une collision.
///
/// # Liste des IDs de collision
/// - 8: Bordure bleue principale
/// - 16, 17, 18: Coins et bordures supérieures
/// - 25, 26, 27: Tuiles grises
/// - 34, 35, 36: Bordures inférieures
/// - 64, 65, 66: Bordures intérieures supérieures
/// - 73, 75: Bordures latérales
/// - 82, 83, 84: Bordures intérieures inférieures
fn is_collision_tile(tile_id: u32) -> bool {
    matches!(
        tile_id,
        8 | 16 | 17 | 18 | 25 | 26 | 27 | 34 | 35 | 36 | 64 | 65 | 66 | 73 | 75 | 82 | 83 | 84
    )
}
