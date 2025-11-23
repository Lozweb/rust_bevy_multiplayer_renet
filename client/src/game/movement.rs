//! Gestion des entrées du joueur et traduction en mouvement via un contrôleur de personnage.
//!
//! Un contrôleur de personnage est l'ensemble des systèmes qui régissent le mouvement des personnages.
//!
//! Dans notre cas, le contrôleur de personnage a la logique suivante :
//! - Définir l'intention de [`MovementController`] basée sur les entrées directionnelles du clavier.
//!   Ceci est fait dans le module `player`, car c'est spécifique au personnage joueur.
//! - Appliquer la rotation du système de visée basée sur [`AimDirection`].

use crate::game::player::{AimRig, Player};
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use game_core::player::{AimDirection, ControlledPlayer, MovementController};

pub(super) fn plugin(app: &mut App) {
    // Applique le mouvement UNIQUEMENT pour le joueur local (qui a la physique)
    // Les joueurs distants utilisent l'interpolation réseau
    app.add_systems(
        Update,
        (apply_local_movement, apply_aim_direction)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

// NOTE: apply_movement a été SUPPRIMÉ pour les joueurs distants
// Architecture Full Server Authority : les joueurs distants
// suivent le serveur via interpolation dans position_sync.rs
//
// MAIS le joueur local a besoin de physique locale pour un mouvement fluide instantané !
// Ce système applique le mouvement UNIQUEMENT au joueur local.

/// Applique le mouvement du joueur local basé sur le MovementController.
///
/// IMPORTANT : Ce système s'applique UNIQUEMENT au joueur local (ControlledPlayer).
/// Les joueurs distants n'ont pas de mouvement actif, seulement l'interpolation réseau.
///
/// Le joueur local utilise un RigidBody::Dynamic pour des collisions physiques réelles.
/// La synchronisation avec le serveur est maintenue par l'interpolation réseau.
fn apply_local_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut LinearVelocity), With<ControlledPlayer>>,
) {
    // Accélération augmentée pour plus de réactivité
    const ACCELERATION: f32 = 30.0;

    let delta = time.delta_secs();

    for (controller, mut velocity) in &mut movement_query {
        let desired_velocity = controller.intent * controller.max_speed;

        // Interpolation progressive vers la vélocité désirée
        // Permet au moteur physique de gérer correctement les collisions
        let t = (ACCELERATION * delta).min(1.0);
        velocity.0 = velocity.0.lerp(desired_velocity, t);
    }
}

fn apply_aim_direction(
    players_query: Query<(&AimDirection, &Children), With<Player>>,
    mut aim_rig_query: Query<&mut Transform, With<AimRig>>,
) {
    for (aim_direction, children) in &players_query {
        for &child in children {
            if let Ok(mut rig_transform) = aim_rig_query.get_mut(child) {
                rig_transform.rotation = Quat::from_rotation_z(aim_direction.0);
            }
        }
    }
}
