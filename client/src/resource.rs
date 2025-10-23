use bevy::prelude::{Entity, Resource};
use bevy_renet::renet::ClientId;
use game_core::client::PlayerEntities;
use std::collections::HashMap;

/// Représente l'état du lobby côté client.
///
/// Contient une table de hachage qui associe chaque `ClientId` aux
/// `PlayerEntities` correspondantes pour suivre les joueurs connectés.
#[derive(Debug, Default, Resource)]
pub struct ClientLobby {
    /// Mappe l'identifiant Renet d'un client aux entités du joueur.
    players: HashMap<ClientId, PlayerEntities>,
}

impl ClientLobby {
    /// Ajoute un joueur au lobby client.
    ///
    /// - `client_id` : identifiant unique du client (`ClientId`).
    /// - `entities` : entités associées au joueur (`PlayerEntities`).
    pub fn add_player(&mut self, client_id: &ClientId, entities: PlayerEntities) {
        self.players.insert(*client_id, entities);
    }

    /// Supprime un joueur du lobby client.
    ///
    /// - `client_id` : identifiant unique du client à retirer.
    /// - Retourne une option contenant les `PlayerEntities` retirées si trouvées.
    pub fn remove_player(&mut self, client_id: &ClientId) -> Option<PlayerEntities> {
        self.players.remove(client_id)
    }

    /// Récupère les entités du joueur associées à un `ClientId`.
    ///
    /// - `client_id` : identifiant unique du client.
    /// - Retourne une option contenant les `PlayerEntities` si trouvées.
    pub fn get_player_entities(&self, client_id: &ClientId) -> Option<&PlayerEntities> {
        self.players.get(client_id)
    }
}

/// Identifiant unique du client courant généré localement.
///
/// Valeur publique pour être facilement accessible depuis les systèmes.
#[derive(Debug, Resource)]
pub struct CurrentClientId(pub u64);

/// Mappe les entités côté client aux entités correspondantes côté serveur.
/// Utile pour synchroniser les états entre le client et le serveur.
///
/// Contient une table de hachage où la clé est l'entité côté client
/// et la valeur est l'entité correspondante côté serveur.
///
#[derive(Default, Resource)]
pub struct PlayerMapping(pub(crate) HashMap<Entity, Entity>);

impl PlayerMapping {
    /// Ajoute une correspondance entre une entité client et une entité serveur.
    ///
    /// - `client_entity` : entité côté client.
    /// - `server_entity` : entité correspondante côté serveur.
    pub fn add(&mut self, client_entity: Entity, server_entity: Entity) {
        self.0.insert(client_entity, server_entity);
    }

    /// Récupère l'entité serveur associée à une entité client.
    ///
    /// - `client_entity` : entité côté client.
    /// - Retourne une option contenant l'entité serveur si trouvée.
    pub fn get(&self, client_entity: &Entity) -> Option<&Entity> {
        self.0.get(client_entity)
    }

    /// Supprime la correspondance pour une entité client donnée.
    ///
    /// - `client_entity` : entité côté client à retirer.
    pub fn remove(&mut self, client_entity: &Entity) {
        self.0.remove(client_entity);
    }
}
