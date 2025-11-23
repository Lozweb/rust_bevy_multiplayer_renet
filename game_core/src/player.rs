use bevy::asset::Assets;
use bevy::math::{Vec2, Vec3};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{
    Circle, ColorMaterial, Commands, Component, Deref, Entity, MeshMaterial2d, ResMut, Resource,
    Transform, Visibility,
};
use bevy_renet::renet::ClientId;
use serde::{Deserialize, Serialize};

/// Snapshot des entrées d'un joueur à envoyer/recevoir sur le réseau.
///
/// Ce type est sérialisable via `serde` et peut être attaché en tant que
/// `Component` ou `Resource` selon le besoin (par ex. envoi périodique
/// d'inputs au serveur ou stockage local).
///
/// Champs :
/// - `up`, `down`, `left`, `right` : directions de mouvement (bool).
/// - `jump` : action de saut.
/// - `aim_direction` : direction du visée en radians (f32).
/// - `shoot` : tir/attaque.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub aim_direction: f32,
    pub shoot: bool,
}

/// Resource contenant la direction de visée courante du joueur local.
///
/// Valeur en radians. Utilisé pour partager rapidement l'angle entre
/// systèmes (UI, visée, synchronisation).
#[derive(Resource, Default)]
pub struct AimDirection(pub f32);

/// Position de la souris dans l'espace monde.
///
/// Contient `None` si la position n'est pas disponible. Le `Deref` permet
/// d'accéder directement à l'`Option<Vec2>` lorsque la ressource est récupérée.
#[derive(Resource, Debug, Default, Deref)]
pub struct MouseWorldCoords(pub Option<Vec2>);

/// Représente un joueur connecté au serveur.
///
/// Contient l'identifiant réseau fourni par `bevy_renet` et le nom affiché.
#[derive(Debug, Component)]
pub struct PlayerInfo {
    /// Identifiant unique du client (fourni par `bevy_renet').
    pub id: ClientId,
    /// Nom affiché du joueur.
    pub name: String,
}
/// Marque une entité comme contrôlée par le joueur local.
///
/// Utilisé pour identifier l'entité du joueur que le client local contrôle
/// (par exemple pour la caméra, les entrées et la logique de possession).
/// Composant "tag" sans données.
#[derive(Component)]
pub struct ControlledPlayer;

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
        // Composants physiques pour la simulation serveur
        RigidBody::Dynamic,
        Collider::rectangle(32.0, 32.0),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
    ));

    // Ajouter les composants visuels uniquement si les assets sont disponibles
    if let (Some(meshes), Some(materials)) = (meshes.as_mut(), materials.as_mut()) {
        entity_commands.insert((
            Mesh2d(meshes.add(Mesh::from(Circle::new(40.0)))),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
        ));
    }

    entity_commands.id()
}
