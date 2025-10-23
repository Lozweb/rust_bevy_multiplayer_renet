use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::{
    Circle, ColorMaterial, Commands, Component, Entity, MeshMaterial2d, Name, ResMut, Transform,
};
use bevy_renet::renet::ClientId;

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
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    commands
        .spawn((
            Name::new(format!("Player_{client_id}")),
            Transform::from_translation(position),
            Mesh2d(meshes.add(Mesh::from(Circle::new(40.0)))),
            MeshMaterial2d(materials.add(ColorMaterial::default())),
        ))
        .id()
}
