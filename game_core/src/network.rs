use bevy::log::error;
use bevy::prelude::Component;
use bevy_renet::renet::{ChannelConfig, ConnectionConfig, SendType};
use bincode::error::DecodeError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, SystemTime};

pub const PROTOCOL_ID: u64 = 1;

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: ClientChannel::channel_config(),
        server_channels_config: ServerChannel::channel_config(),
    }
}

pub fn get_current_time() -> Duration {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|e| {
            error!("Erreur lors de la récupération du temps système: {e}");
            Duration::from_secs(0)
        })
}

pub fn get_socket(socket_address: SocketAddr) -> UdpSocket {
    UdpSocket::bind(socket_address)
        .unwrap_or_else(|e| panic!("Erreur lors de la création du socket UDP: {e}"))
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerChannel {
    EntitiesPosition,
    EntityEvent,
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::EntitiesPosition => 0,
            ServerChannel::EntityEvent => 1,
        }
    }
}

impl ServerChannel {
    pub fn channel_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: ServerChannel::EntitiesPosition.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::Unreliable,
            },
            ChannelConfig {
                channel_id: ServerChannel::EntityEvent.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(250),
                },
            },
        ]
    }
}

pub enum ClientChannel {
    Input,
    ReliableCommand,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Input => 0,
            ClientChannel::ReliableCommand => 1,
        }
    }
}

impl ClientChannel {
    pub fn channel_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: ClientChannel::Input.into(),
                max_memory_usage_bytes: 5 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::ZERO,
                },
            },
            ChannelConfig {
                channel_id: ClientChannel::ReliableCommand.into(),
                max_memory_usage_bytes: 2 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(200),
                },
            },
        ]
    }
}

pub trait MessageSerialize {
    fn to_bytes(&self) -> Vec<u8>;
}

impl<T> MessageSerialize for T
where
    T: Serialize,
{
    fn to_bytes(&self) -> Vec<u8> {
        serialize_message(self)
    }
}

pub trait MessageDeserialize: Sized {
    fn from_bytes(bytes: &[u8]) -> Self;
}

pub trait DeserializeErrorFallback {
    fn deserialize_error(err: DecodeError) -> Self;
}

impl<T> MessageDeserialize for T
where
    T: DeserializeOwned + DeserializeErrorFallback,
{
    fn from_bytes(bytes: &[u8]) -> Self {
        bincode::serde::decode_from_slice::<T, _>(bytes, bincode::config::standard())
            .map(|(msg, _)| msg)
            .unwrap_or_else(|err| {
                error!("Deserialization error: {:?}", err);
                T::deserialize_error(err)
            })
    }
}

pub fn serialize_message<T: MessageSerialize + Serialize>(message: &T) -> Vec<u8> {
    bincode::serde::encode_to_vec(message, bincode::config::standard()).unwrap_or_else(|err| {
        error!("Serialization error: {:?}", err);
        Vec::new()
    })
}
