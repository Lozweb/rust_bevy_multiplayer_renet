use bevy::prelude::{Entity, Resource};
use bevy_renet::renet::ClientId;
use std::collections::HashMap;

/// Paire d'entités client/serveur pour un joueur.
#[derive(Debug)]
pub struct PlayerEntities {
    /// Entité locale représentant le joueur côté client
    pub client_entity: Entity,
    /// Entité correspondante côté serveur
    pub server_entity: Entity,
}

/// Lobby client : associe les clients connectés à leurs entités.
#[derive(Debug, Default, Resource)]
pub struct ClientLobby {
    /// Mappe l'identifiant d'un client aux entités du joueur
    pub players: HashMap<ClientId, PlayerEntities>,
}

impl ClientLobby {
    /// Ajoute un joueur au lobby client.
    pub fn add_player(&mut self, client_id: &ClientId, entities: PlayerEntities) {
        self.players.insert(*client_id, entities);
    }

    /// Supprime un joueur du lobby client.
    pub fn remove_player(&mut self, client_id: &ClientId) -> Option<PlayerEntities> {
        self.players.remove(client_id)
    }

    /// Récupère les entités associées à un client.
    pub fn get_player_entities(&self, client_id: &ClientId) -> Option<&PlayerEntities> {
        self.players.get(client_id)
    }

    /// Recherche un client par son entité serveur.
    ///
    /// Recherche linéaire O(n) dans le HashMap.
    pub fn get_player_by_server_entity(
        &self,
        server_entity: &Entity,
    ) -> Option<(&ClientId, &PlayerEntities)> {
        self.players.iter().find_map(|(client_id, entities)| {
            if &entities.server_entity == server_entity {
                Some((client_id, entities))
            } else {
                None
            }
        })
    }
}

/// Identifiant unique du client courant.
#[derive(Debug, Resource)]
pub struct CurrentClientId(pub ClientId);
