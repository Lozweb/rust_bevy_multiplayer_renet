use bevy::prelude::{Entity, Resource};
use bevy_renet::renet::ClientId;
use std::collections::HashMap;

/// Informations liant l'entité côté client à l'entité correspondante côté serveur.
///
/// - `client_entity` : entité locale représentant le joueur dans le client.
/// - `server_entity` : entité correspondante telle qu'identifiée par le serveur.
#[derive(Debug)]
pub struct PlayerEntities {
    pub client_entity: Entity,
    pub server_entity: Entity,
}

/// Représente l'état du lobby côté client.
///
/// Contient une table de hachage qui associe chaque `ClientId` aux
/// `PlayerEntities` correspondantes pour suivre les joueurs connectés.
#[derive(Debug, Default, Resource)]
pub struct ClientLobby {
    /// Mappe l'identifiant Renet d'un client aux entités du joueur.
    pub(crate) players: HashMap<ClientId, PlayerEntities>,
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

    /// Retourne la paire `(ClientId, PlayerEntities)` correspondant à une
    /// `server_entity` donnée, si elle est présente dans le lobby client.
    ///
    /// Recherche linéaire sur la table `players` : complexité O(n).
    ///
    /// # Arguments
    ///
    /// * `server_entity` - Référence à l'entité côté serveur à chercher.
    ///
    /// # Retour
    ///
    /// * `Some((&ClientId, &PlayerEntities))` si une entrée avec
    ///   `entities.server_entity == *server_entity` est trouvée, sinon `None`.
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

/// Identifiant unique du client courant généré localement.
///
/// Valeur publique pour être facilement accessible depuis les systèmes.
#[derive(Debug, Resource)]
pub struct CurrentClientId(pub u64);
