use bevy::math::Vec3;
use bevy::prelude::{Entity, Message};
use bevy_renet::renet::ClientId;

#[derive(Message)]
/// Énumération des événements de jeu envoyés via le système de messages.
///
/// - `PlayerCreated` : déclenché lorsqu'un joueur est créé sur le serveur.
/// - `PlayerRemoved` : déclenché lorsqu'un joueur est retiré (déconnexion/suppression).
pub enum GameEvent {
    /// Un nouveau joueur a été créé.
    ///
    /// * `client_id` : identifiant du client associé au joueur.
    /// * `entity` : entité Bevy représentant le joueur.
    /// * `position` : position initiale du joueur dans le monde.
    PlayerCreated {
        client_id: ClientId,
        entity: Entity,
        position: Vec3,
    },
    /// Un joueur a été retiré.
    ///
    /// * `client_id` : identifiant du client retiré.
    PlayerRemoved { client_id: ClientId },
}
