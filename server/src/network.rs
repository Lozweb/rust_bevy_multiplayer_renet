use crate::handler::collision_event::collision_event;
use crate::handler::player_input::{
    apply_movement, interpolate_movement_intent, process_client_inputs,
};
use crate::handler::position_sync::{sync_enemies_positions, sync_players_position};
use crate::handler::server_event::server_event;
use bevy::app::{App, FixedUpdate, Update};
use bevy::prelude::{IntoScheduleConfigs, ResMut};
use bevy_renet::renet::{ClientId, RenetServer};
use game_core::network::ServerChannel;
use game_core::server::{
    EnemyMessages, EnemyPositionMessages, PlayerMessages, PlayerPositionMessages,
    ProjectileMessages, ServerReliableMessages, ServerUnreliableMessages,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            server_event,
            process_client_inputs,
            interpolate_movement_intent.after(process_client_inputs),
            apply_movement.after(interpolate_movement_intent),
            sync_players_position.after(apply_movement),
        ),
    );
    app.add_systems(FixedUpdate, (sync_enemies_positions, collision_event));
}

pub fn send_player_event(
    client_id: ClientId,
    server: &mut ResMut<RenetServer>,
    message: PlayerMessages,
) {
    ServerReliableMessages::send(
        &client_id,
        &ServerReliableMessages::PlayerEvent(message),
        ServerChannel::EntityEvent,
        server,
    )
}

pub fn broadcast_player_event(server: &mut ResMut<RenetServer>, message: PlayerMessages) {
    ServerReliableMessages::broadcast(
        &ServerReliableMessages::PlayerEvent(message),
        ServerChannel::EntityEvent,
        server,
    )
}

pub fn broadcast_player_position(
    server: &mut ResMut<RenetServer>,
    message: PlayerPositionMessages,
) {
    ServerUnreliableMessages::broadcast(
        &ServerUnreliableMessages::PlayerPositionsEvent(message),
        ServerChannel::EntitiesPosition,
        server,
    )
}

pub fn send_enemy_event(
    client_id: ClientId,
    server: &mut ResMut<RenetServer>,
    message: EnemyMessages,
) {
    ServerReliableMessages::send(
        &client_id,
        &ServerReliableMessages::EnemyEvent(message),
        ServerChannel::EntityEvent,
        server,
    )
}

pub fn broadcast_enemy_event(server: &mut ResMut<RenetServer>, message: EnemyMessages) {
    ServerReliableMessages::broadcast(
        &ServerReliableMessages::EnemyEvent(message),
        ServerChannel::EntityEvent,
        server,
    )
}

pub fn broadcast_enemy_position(server: &mut ResMut<RenetServer>, message: EnemyPositionMessages) {
    ServerUnreliableMessages::broadcast(
        &ServerUnreliableMessages::EnemyPositionsEvent(message),
        ServerChannel::EntitiesPosition,
        server,
    )
}

pub fn broadcast_projectile_event(server: &mut ResMut<RenetServer>, message: ProjectileMessages) {
    ServerReliableMessages::broadcast(
        &ServerReliableMessages::ProjectileEvent(message),
        ServerChannel::EntityEvent,
        server,
    )
}
