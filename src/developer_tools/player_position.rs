use std::collections::VecDeque;

use bevy::{prelude::*, utils::HashMap};
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts,
};
use egui_plot::{Legend, Line, Plot, PlotPoints, PlotUi};

use crate::character::player::{LocalPlayer, Player};

use super::tool_enabled;

pub struct PlayerPositionPlugin;

impl Plugin for PlayerPositionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerPositionGraph>().add_systems(
            Update,
            ui.run_if(tool_enabled(|tools| tools.player_position)),
        );
    }
}

#[derive(Debug, Clone, Resource, Default)]
struct PlayerPositionGraph {
    data: HashMap<Entity, PlayerPositionGraphInner>,
}

#[derive(Debug, Clone)]
struct PlayerPositionGraphInner {
    previous_body: Vec3,
    body_delta: VecDeque<Vec3>,
    previous_camera: Vec3,
    camera_delta: VecDeque<Vec3>,
}

impl PlayerPositionGraphInner {
    pub fn update_body(&mut self, body_position: Vec3) {
        let delta = body_position - self.previous_body;
        self.previous_body = body_position;

        self.body_delta.pop_front();
        self.body_delta.push_back(delta);
    }

    pub fn update_camera(&mut self, camera_position: Vec3) {
        let delta = camera_position - self.previous_camera;
        self.previous_camera = camera_position;

        self.camera_delta.pop_front();
        self.camera_delta.push_back(delta);
    }
}

impl Default for PlayerPositionGraphInner {
    fn default() -> Self {
        Self {
            previous_body: Vec3::ZERO,
            body_delta: vec![Vec3::ZERO; 120].into(),
            previous_camera: Vec3::ZERO,
            camera_delta: vec![Vec3::ZERO; 120].into(),
        }
    }
}

fn ui(
    mut ctx: EguiContexts,
    mut position_graph: ResMut<PlayerPositionGraph>,
    players: Query<(Entity, &Transform, &Player), With<LocalPlayer>>,
    cameras: Query<&Transform, With<Camera>>,
) {
    egui::Window::new("Player position").show(ctx.ctx_mut(), |ui| {
        if players.is_empty() {
            ui.colored_label(Color32::RED, "No players found");
            return;
        }

        for (entity, position, player) in players.iter() {
            let data = match position_graph.data.get_mut(&entity) {
                Some(data) => data,
                None => {
                    position_graph
                        .data
                        .insert(entity, PlayerPositionGraphInner::default());
                    position_graph.data.get_mut(&entity).unwrap()
                }
            };

            data.update_body(position.translation);

            ui.collapsing(player, |ui| {
                write_graph(
                    ui,
                    &data.body_delta,
                    "Body delta position",
                    "body_delta_position",
                );

                if let Some(attached_camera) = player.attached_camera {
                    data.update_camera(cameras.get(attached_camera).unwrap().translation);
                    write_graph(
                        ui,
                        &data.camera_delta,
                        "Camera delta position",
                        "camera_delta_position",
                    );
                } else {
                    ui.colored_label(Color32::RED, "No attached camera");
                }
            });
        }
    });
}

fn write_graph(ui: &mut egui::Ui, data: &VecDeque<Vec3>, heading: &str, id: &str) {
    ui.collapsing(heading, |ui| {
        Plot::new(id)
            .include_x(0.0)
            .include_x(data.len() as f64)
            .include_y(0.0)
            .include_y(144.0)
            .view_aspect(3.0)
            .height(128.0)
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                write_line(plot_ui, data, 0, "X", Color32::RED);
                write_line(plot_ui, data, 1, "Y", Color32::GREEN);
                write_line(plot_ui, data, 2, "Z", Color32::BLUE);
            });

        ui.horizontal(|ui| {
            write_info(ui, data, 0, "X", Color32::RED);
            write_info(ui, data, 1, "Y", Color32::GREEN);
            write_info(ui, data, 2, "Z", Color32::BLUE);
        });
    });
}

fn write_line(
    plot_ui: &mut PlotUi,
    data: &VecDeque<Vec3>,
    element: usize,
    text: &str,
    color: Color32,
) {
    let points = PlotPoints::from_parametric_callback(
        |x| (x, data[x as usize][element].abs() as f64 * 1000.0),
        0.0..=(data.len() - 1) as f64,
        data.len(),
    );

    plot_ui.line(Line::new(points).fill(5.0).name(text).color(color));
}

fn write_info(
    ui: &mut egui::Ui,
    data: &VecDeque<Vec3>,
    element: usize,
    text: &str,
    color: Color32,
) {
    ui.colored_label(
        color,
        format!("{text}: {:.2}", data.back().unwrap()[element] * 1000.0),
    );
}
