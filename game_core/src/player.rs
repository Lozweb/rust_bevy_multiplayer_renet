use bevy::asset::Assets;
use bevy::math::{Vec2, Vec3};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::ReflectComponent;
use bevy::prelude::ReflectResource;
use bevy::prelude::{
    Circle, ColorMaterial, Commands, Component, Deref, Entity, MeshMaterial2d, Reflect, ResMut,
    Resource, Transform, Visibility,
};
use bevy_renet::renet::ClientId;
use serde::{Deserialize, Serialize};

/// Snapshot des entrées d'un joueur, transmis sur le réseau.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    /// Déplacement vers le haut
    pub up: bool,
    /// Déplacement vers le bas
    pub down: bool,
    /// Déplacement vers la gauche
    pub left: bool,
    /// Déplacement vers la droite
    pub right: bool,
    /// Action de saut
    pub jump: bool,
    /// Direction de visée en radians
    pub aim_direction: f32,
    /// Action de tir
    pub shoot: bool,
}

/// Direction de visée courante en radians.
#[derive(Component, Resource, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component, Resource)]
pub struct AimDirection(pub f32);

/// Contrôleur de mouvement pour un personnage.
///
/// Utilisé côté client et serveur pour gérer le mouvement basé sur les intentions.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct MovementController {
    /// Direction de mouvement normalisée (0.0 à 1.0)
    pub intent: Vec2,
    /// Direction cible (input du joueur, pour interpolation serveur)
    pub target_intent: Vec2,
    /// Vitesse maximale en unités/seconde
    pub max_speed: f32,
}

impl Default for MovementController {
    fn default() -> Self {
        Self {
            intent: Vec2::ZERO,
            target_intent: Vec2::ZERO,
            max_speed: 400.0,
        }
    }
}

/// Position de la souris dans l'espace monde.
///
/// Contient `None` si la position n'est pas disponible.
#[derive(Resource, Debug, Default, Deref)]
pub struct MouseWorldCoords(pub Option<Vec2>);

/// Informations d'un joueur connecté.
#[derive(Debug, Component)]
pub struct PlayerInfo {
    /// Identifiant unique du client
    pub id: ClientId,
    /// Nom affiché du joueur
    pub name: String,
}

/// Marque l'entité contrôlée par le joueur local.
///
/// Utilisé pour identifier le joueur local (caméra, entrées, possession).
#[derive(Component)]
pub struct ControlledPlayer;

/// Crée une entité joueur avec physique et rendu optionnel.
///
/// # Arguments
///
/// * `client_id` - Identifiant du client propriétaire
/// * `position` - Position initiale du joueur
/// * `commands` - Commands Bevy pour spawner l'entité
/// * `meshes` - Assets de mesh (optionnel, pour le rendu)
/// * `materials` - Assets de matériaux (optionnel, pour le rendu)
///
/// # Returns
///
/// L'`Entity` créée
pub fn spawn_player(
    client_id: &ClientId,
    position: Vec3,
    commands: &mut Commands,
    meshes: &mut Option<ResMut<Assets<Mesh>>>,
    materials: &mut Option<ResMut<Assets<ColorMaterial>>>,
) -> Entity {
    use avian2d::prelude::*;

    let mut entity_commands = commands.spawn((
        Transform::from_translation(position),
        Visibility::default(),
        PlayerInfo {
            id: *client_id,
            name: format!("Player_{client_id}"),
        },
        RigidBody::Dynamic,
        Collider::rectangle(32.0, 32.0),
        Mass(50.0),
        LinearDamping(1.5), // Damping réduit pour plus de réactivité
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.1),    // Friction faible pour glissement fluide
        Restitution::new(0.0), // Pas de rebond sur les collisions
    ));

    if let (Some(meshes), Some(materials)) = (meshes.as_mut(), materials.as_mut()) {
        entity_commands.insert((
            Mesh2d(meshes.add(Mesh::from(Circle::new(40.0)))),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
        ));
    }

    entity_commands.id()
}
