use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
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

use crate::character::player::{self, Player};

use super::network_error::NetworkError;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            process_server_events.run_if(resource_exists::<RenetServer>()),
        );
    }
}

#[derive(Resource)]
pub struct Server;

pub fn start_listening(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    network_channels: Res<NetworkChannels>,
    server_port: u16,
) -> Result<(), NetworkError> {
    let server = RenetServer::new(ConnectionConfig {
        server_channels_config: network_channels.get_server_configs(),
        client_channels_config: network_channels.get_client_configs(),
        ..default()
    });

    let public_address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), server_port);
    let socket = UdpSocket::bind(public_address).map_err(|_| NetworkError::UnableBindSocket)?;
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
        .map_err(|_| NetworkError::UnableCreateServerTransport)?;

    info!("Server started on {}", public_address);

    commands.insert_resource(server);
    commands.insert_resource(transport);
    commands.insert_resource(Server);

    player::spawn(
        &mut commands,
        &mut meshes,
        &mut materials,
        player::Player {
            client_id: SERVER_ID.into(),
        },
        player::PlayerKind::Local,
    );

    Ok(())
}

fn process_server_events(
    mut server_event: EventReader<ServerEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Player)>,
) {
    for event in server_event.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Player {client_id} connected.");

                player::spawn(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    player::Player {
                        client_id: (*client_id).into(),
                    },
                    player::PlayerKind::Remote,
                );
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Player {client_id} disconnected: {reason}");

                query
                    .iter()
                    .filter(|x| x.1.client_id == client_id)
                    .for_each(|x| {
                        commands.entity(x.0).despawn_recursive();
                    });
            }
        }
    }
}
