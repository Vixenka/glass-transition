use std::{
    net::{IpAddr, SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_replicon::{
    prelude::*,
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        ConnectionConfig,
    },
};
use serde::{Deserialize, Serialize};

use super::network_error::NetworkError;

#[derive(Resource)]
pub struct Client {
    pub id: ClientId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientId(u64);

impl From<bevy_replicon::renet::ClientId> for ClientId {
    fn from(client_id: bevy_replicon::renet::ClientId) -> Self {
        Self(client_id.raw())
    }
}

impl PartialEq<&bevy_replicon::renet::ClientId> for ClientId {
    fn eq(&self, other: &&bevy_replicon::renet::ClientId) -> bool {
        self.0 == other.raw()
    }
}

pub fn start_connection(
    mut commands: Commands,
    network_channels: Res<NetworkChannels>,
    server_address: IpAddr,
    server_port: u16,
) -> Result<(), NetworkError> {
    println!("Starting client...");

    let client = RenetClient::new(ConnectionConfig {
        server_channels_config: network_channels.get_server_configs(),
        client_channels_config: network_channels.get_client_configs(),
        ..Default::default()
    });

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;

    let address = SocketAddr::new(server_address, server_port);
    let socket =
        UdpSocket::bind((server_address, 0)).map_err(|_| NetworkError::UnableBindSocket)?;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: super::PROTOCOL_ID,
        server_addr: address,
        user_data: None,
    };
    let transport = NetcodeClientTransport::new(current_time, authentication, socket)
        .map_err(|_| NetworkError::UnableCreateClientTransport)?;

    info!("Client started on {}", address);

    commands.insert_resource(Client {
        id: ClientId(client_id),
    });
    commands.insert_resource(client);
    commands.insert_resource(transport);

    Ok(())
}
