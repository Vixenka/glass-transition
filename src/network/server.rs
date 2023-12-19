use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    str::FromStr,
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_replicon::{
    prelude::*,
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, ServerEvent,
    },
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            event_system.run_if(resource_exists::<RenetServer>()),
        );
    }
}

pub fn start_listening(
    mut commands: Commands,
    address: &str,
    network_channels: Res<NetworkChannels>,
) {
    let server = RenetServer::new(ConnectionConfig {
        server_channels_config: network_channels.get_server_configs(),
        client_channels_config: network_channels.get_client_configs(),
        ..default()
    });

    let public_address = SocketAddr::new(
        Ipv4Addr::LOCALHOST.into(),
        u16::from_str(address.split(':').last().unwrap_or("13001"))
            .expect("Port is not a u16 number"),
    );
    let socket = UdpSocket::bind(public_address).expect("Unable to bind socket");
    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 2,
        protocol_id: super::PROTOCOL_ID,
        authentication: ServerAuthentication::Unsecure,
        public_addresses: vec![public_address],
    };
    let transport = NetcodeServerTransport::new(server_config, socket)
        .expect("Unable to create server transport");

    info!("Server started on {}", public_address);

    commands.insert_resource(server);
    commands.insert_resource(transport);
}

fn event_system(mut server_event: EventReader<ServerEvent>) {
    for event in server_event.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("player: {client_id} Connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("client {client_id} disconnected: {reason}");
            }
        }
    }
}
