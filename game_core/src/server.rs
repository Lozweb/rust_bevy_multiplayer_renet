use crate::enemy::EnemyType;
use crate::network::{MessageSerialize, ServerChannel};
use bevy::prelude::{Component, Entity, ResMut, Vec2, Vec3};
use bevy_renet::renet::{ClientId, RenetServer};
use bincode::error::DecodeError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct NetworkedEnemyData {
    pub server_entity: Entity,
    pub position: Vec3,
    pub health: u32,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerReliableMessages {
    EnemyEvent(EnemyMessages),
    PlayerEvent(PlayerMessages),
    ProjectileEvent(ProjectileMessages),
    ErrorMessage { reason: String },
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerUnreliableMessages {
    EnemyPositionsEvent(EnemyPositionMessages),
    PlayerPositionsEvent(PlayerPositionMessages),
    ErrorMessage { cause: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EnemyMessages {
    EnemySpawned {
        server_entity: Entity,
        enemy_type: EnemyType,
        position: Vec3,
    },
    EnemyDeath {
        server_entity: Entity,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EnemyPositionMessages {
    EnemyPositionsUpdate { enemy_data: Vec<NetworkedEnemyData> },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectileMessages {
    ProjectileSpawned {
        server_entity: Entity,
        damage: u32,
        position: Vec3,
        direction: f32,
    },
    ProjectileCollision {
        server_entity: Entity,
    },
    ProjectileCleanup {
        server_entity: Entity,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerMessages {
    PlayerCreate {
        server_entity: Entity,
        client_id: ClientId,
        position: Vec3,
    },
    PlayerRemove {
        client_id: ClientId,
    },
    PlayerDamaged {
        player_entity: Entity,
        damage: u32,
        current_health: u32,
    },
    PlayerDeath {
        player_entity: Entity,
    },
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerPositionMessages {
    PlayerPositionUpdate {
        client_id: ClientId,
        position: Vec3,
        velocity: Vec2,
        aim_direction: f32,
    },
}

impl crate::network::DeserializeErrorFallback for ServerReliableMessages {
    fn deserialize_error(err: DecodeError) -> Self {
        ServerReliableMessages::ErrorMessage {
            reason: format!("Failed to deserialize ServerMessages: {}", err),
        }
    }
}

impl crate::network::DeserializeErrorFallback for ServerUnreliableMessages {
    fn deserialize_error(err: DecodeError) -> Self {
        ServerUnreliableMessages::ErrorMessage {
            cause: format!("Failed to deserialize ServerMessages: {}", err),
        }
    }
}

impl ServerUnreliableMessages {
    pub fn broadcast(
        server_message: &ServerUnreliableMessages,
        chanel: ServerChannel,
        server: &mut ResMut<RenetServer>,
    ) {
        server.broadcast_message(chanel, ServerUnreliableMessages::to_bytes(server_message));
    }

    pub fn send(
        client_id: &ClientId,
        server_message: &ServerUnreliableMessages,
        chanel: ServerChannel,
        server: &mut ResMut<RenetServer>,
    ) {
        server.send_message(
            *client_id,
            chanel,
            ServerUnreliableMessages::to_bytes(server_message),
        );
    }
}

impl ServerReliableMessages {
    pub fn broadcast(
        server_message: &ServerReliableMessages,
        chanel: ServerChannel,
        server: &mut ResMut<RenetServer>,
    ) {
        server.broadcast_message(chanel, ServerReliableMessages::to_bytes(server_message));
    }

    pub fn send(
        client_id: &ClientId,
        server_message: &ServerReliableMessages,
        chanel: ServerChannel,
        server: &mut ResMut<RenetServer>,
    ) {
        server.send_message(
            *client_id,
            chanel,
            ServerReliableMessages::to_bytes(server_message),
        );
    }
}
