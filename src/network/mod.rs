pub mod client;
pub mod network_error;
pub mod replication;
pub mod server;

use std::{net::IpAddr, str::FromStr};

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts,
};
use bevy_replicon::{
    renet::RenetServer, replicon_core::NetworkChannels, server::TickPolicy, ReplicationPlugins,
};

use crate::character::player::LocalPlayerResource;

use self::{
    client::Client,
    network_error::NetworkError,
    server::{Server, ServerPlugin},
};

pub const MAX_TICK_RATE: u16 = 30;
pub const PROTOCOL_ID: u64 = 0;
pub const DEFAULT_PORT: u16 = 13001;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            ReplicationPlugins
                .build()
                .set(bevy_replicon::server::ServerPlugin {
                    tick_policy: TickPolicy::MaxTickRate(MAX_TICK_RATE),
                    ..default()
                }),
        )
        .insert_resource(MultiplayerUiState {
            address: String::from_str("127.0.0.1:13001").unwrap(),
            last_error: None,
        })
        .add_plugins((replication::ReplicationPlugin, ServerPlugin))
        .add_systems(Update, ui);
    }
}

pub fn has_server(server: Option<Res<Server>>) -> bool {
    server.is_some()
}
pub fn has_client(client: Option<Res<Client>>) -> bool {
    client.is_some()
}
pub fn has_local_player(local_player: Option<Res<LocalPlayerResource>>) -> bool {
    local_player.is_some()
}
pub fn has_client_and_local_player(
    client: Option<Res<Client>>,
    local_player: Option<Res<LocalPlayerResource>>,
) -> bool {
    client.is_some() && local_player.is_some()
}

#[derive(Resource)]
pub struct MultiplayerUiState {
    address: String,
    last_error: Option<String>,
}

#[allow(clippy::too_many_arguments)]
fn ui(
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut ctx: EguiContexts,
    commands: Commands,
    state: ResMut<MultiplayerUiState>,
    network_channels: Res<NetworkChannels>,
    server: Option<Res<RenetServer>>,
    client: Option<Res<Client>>,
) {
    if server.is_none() && client.is_none() {
        egui::Window::new("Multiplayer").show(ctx.ctx_mut(), |ui| {
            if let Some(err) = &state.last_error {
                ui.colored_label(Color32::RED, err);
            }

            ui_connect(meshes, materials, state, commands, network_channels, ui);
        });
    }
}

fn ui_connect(
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<MultiplayerUiState>,
    commands: Commands,
    network_channels: Res<NetworkChannels>,
    ui: &mut egui::Ui,
) {
    ui.label("Address and port");
    ui.text_edit_singleline(&mut state.address);

    if let Err(err) = ui_connect_buttons(meshes, materials, &state, commands, network_channels, ui)
    {
        state.last_error = Some(err.to_string());
    } else {
        state.last_error = None;
    }
}

fn ui_connect_buttons(
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    state: &ResMut<MultiplayerUiState>,
    commands: Commands,
    network_channels: Res<NetworkChannels>,
    ui: &mut egui::Ui,
) -> Result<(), NetworkError> {
    if ui.button("Connect").clicked() {
        let (ip, port) = parse_address_and_port(&state.address)?;
        return client::start_connection(commands, network_channels, ip, port);
    } else if ui.button("Host game").clicked() {
        let (_ip, port) = parse_address_and_port(&state.address)?;
        return server::start_listening(commands, meshes, materials, network_channels, port);
    }
    Ok(())
}

fn parse_address_and_port(value: &str) -> Result<(IpAddr, u16), NetworkError> {
    let mut split = value.split(':');
    let ip = match split.next() {
        Some(ip) => IpAddr::from_str(ip).map_err(|_| NetworkError::InvalidAddress)?,
        None => return Err(NetworkError::MissingAddress),
    };

    let port = match split.next() {
        Some(port) => u16::from_str(port).map_err(|_| NetworkError::InvalidPort)?,
        None => DEFAULT_PORT,
    };

    if split.next().is_some() {
        Err(NetworkError::InvalidAddress)
    } else {
        Ok((ip, port))
    }
}
