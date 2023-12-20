pub mod client;
pub mod replication;
pub mod server;

use std::str::FromStr;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContexts,
};
use bevy_replicon::{
    renet::RenetServer, replicon_core::NetworkChannels, server::TickPolicy, ReplicationPlugins,
};

use crate::character::player::LocalPlayerResource;

use self::{
    client::Client,
    server::{Server, ServerPlugin},
};

pub const MAX_TICK_RATE: u16 = 30;
pub const PROTOCOL_ID: u64 = 0;

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
        .insert_resource(NetworkUiState {
            address: String::from_str("127.0.0.1:13001").unwrap(),
        })
        .add_plugins((replication::ReplicationPlugin, ServerPlugin))
        .add_systems(Update, ui);
    }
}

#[derive(Resource)]
pub struct NetworkUiState {
    address: String,
}

pub fn has_server() -> impl FnMut(Option<Res<Server>>) -> bool + Clone {
    move |server| server.is_some()
}

pub fn has_client() -> impl FnMut(Option<Res<Client>>) -> bool + Clone {
    move |client| client.is_some()
}

pub fn has_local_player() -> impl FnMut(Option<Res<LocalPlayerResource>>) -> bool + Clone {
    move |local_player| local_player.is_some()
}

pub fn has_client_and_local(
) -> impl FnMut(Option<Res<Client>>, Option<Res<LocalPlayerResource>>) -> bool + Clone {
    move |client, local| client.is_some() && local.is_some()
}

#[allow(clippy::too_many_arguments)]
fn ui(
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut ctx: EguiContexts,
    commands: Commands,
    state: ResMut<NetworkUiState>,
    network_channels: Res<NetworkChannels>,
    server: Option<Res<RenetServer>>,
    client: Option<Res<Client>>,
) {
    egui::Window::new("Network managment").show(ctx.ctx_mut(), |ui| {
        if server.is_none() && client.is_none() {
            ui_connect(meshes, materials, state, commands, network_channels, ui);
        }
    });
}

fn ui_connect(
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut state: ResMut<NetworkUiState>,
    commands: Commands,
    network_channels: Res<NetworkChannels>,
    ui: &mut egui::Ui,
) {
    ui.label("Address");
    ui.text_edit_singleline(&mut state.address);

    if ui.button("Connect").clicked() {
        client::start_connection(commands, &state.address, network_channels);
    } else if ui.button("Host game").clicked() {
        server::start_listening(
            commands,
            meshes,
            materials,
            &state.address,
            network_channels,
        );
    }
}
