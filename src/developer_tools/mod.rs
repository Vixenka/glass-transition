use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use self::prototype_material::PrototypeMaterial;

pub mod interaction;
pub mod player_position;
pub mod prototype_material;
pub mod spawn;
pub mod time;

pub struct DeveloperToolsPlugin;

impl Plugin for DeveloperToolsPlugin {
    fn build(&self, app: &mut App) {
        // Plugins
        app.add_plugins(MaterialPlugin::<PrototypeMaterial>::default())
            .add_plugins(interaction::InteractionPlugin)
            .add_plugins(player_position::PlayerPositionPlugin)
            .add_plugins(time::TimePlugin)
            .add_plugins(spawn::SpawnPlugin);

        // Developer tools debug menu
        app.insert_resource(DeveloperTools {
            hub: cfg!(debug_assertions),
            ..default()
        })
        .add_systems(Update, toggle_hub_ui)
        .add_systems(Update, hub_ui.run_if(tool_enabled(|tools| tools.hub)));
    }
}

/// Resource which stores which developer tools are currently enabled.
#[derive(Debug, Clone, Resource, Default)]
pub struct DeveloperTools {
    pub hub: bool,

    pub interaction: bool,
    pub player_position: bool,
    pub spawn: bool,
    pub time: bool,
}

pub fn tool_enabled(
    f: fn(&DeveloperTools) -> bool,
) -> impl Fn(Res<DeveloperTools>) -> bool + Clone {
    move |developer_tools| developer_tools.hub && f(&developer_tools)
}

fn toggle_hub_ui(mut tools: ResMut<DeveloperTools>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Grave) {
        tools.hub = !tools.hub;
    }
}

fn hub_ui(mut tools: ResMut<DeveloperTools>, mut ctx: EguiContexts) {
    egui::Window::new("Developer Tools").show(ctx.ctx_mut(), |ui| {
        ui.horizontal_wrapped(|ui| {
            // Please keep these sorted alphabetically!
            ui.toggle_value(&mut tools.interaction, "Interaction");
            ui.toggle_value(&mut tools.player_position, "Player position");
            ui.toggle_value(&mut tools.spawn, "Spawn");
            ui.toggle_value(&mut tools.time, "Time");
        });
    });
}
