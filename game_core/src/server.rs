use crate::enemy::EnemyType;
use crate::network::{MessageSerialize, ServerChannel};
use bevy::prelude::{info, Component, Entity, ResMut, Vec2, Vec3};
use bevy_renet::renet::{ClientId, RenetServer};
use bincode::error::DecodeError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerCreate {
        client_id: ClientId,
        position: Vec3,
        entity: Entity,
    },
    PlayerRemove {
        client_id: ClientId,
    },
    PlayerPositionUpdate {
        client_id: ClientId,
        position: Vec3,
        velocity: Vec2,
        aim_direction: f32,
    },
    EnemySpawned {
        server_entity: Entity,
        enemy_type: EnemyType,
        position: Vec3,
    },
    EnemyPositions(Vec<(Entity, Vec3)>),
    ErrorMessage {
        reason: String,
    },
    ProjectileSpawned {
        server_entity: Entity,
        position: Vec3,
        direction: f32,
    },
}

impl crate::network::DeserializeErrorFallback for ServerMessages {
    fn deserialize_error(err: DecodeError) -> Self {
        ServerMessages::ErrorMessage {
            reason: format!("Failed to deserialize ServerMessages: {}", err),
        }
    }
}

impl ServerMessages {
    pub fn broadcast(server_message: &ServerMessages, server: &mut ResMut<RenetServer>) {
        server.broadcast_message(
            ServerChannel::ReliableState,
            ServerMessages::to_bytes(server_message),
        );
        info!("Broadcasted: {:?}", server_message);
    }

    pub fn send(
        client_id: &ClientId,
        server_message: &ServerMessages,
        server: &mut ResMut<RenetServer>,
    ) {
        server.send_message(
            *client_id,
            ServerChannel::ReliableState,
            ServerMessages::to_bytes(server_message),
        );
        info!("Send: {:?}", server_message);
    }
}
