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
    app.add_systems(
        Update,
        (apply_local_movement, apply_aim_direction)
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

/// Applique le mouvement du joueur local basé sur le MovementController.
fn apply_local_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &mut LinearVelocity), With<ControlledPlayer>>,
) {
    const ACCELERATION: f32 = 30.0;
    let delta = time.delta_secs();

    for (controller, mut velocity) in &mut movement_query {
        let desired_velocity = controller.intent * controller.max_speed;
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
