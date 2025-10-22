use bevy::prelude::Component;
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
