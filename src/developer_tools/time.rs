use std::{collections::VecDeque, time::SystemTime};

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Slider},
    EguiContexts,
};
use egui_plot::{Legend, Line, Plot, PlotPoints, PlotUi};

use super::tool_enabled;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeGraph>()
            .add_systems(
                FixedUpdate,
                update_reached.run_if(tool_enabled(|tools| tools.time)),
            )
            .add_systems(Update, ui.run_if(tool_enabled(|tools| tools.time)));
    }
}

#[derive(Debug, Clone, Resource)]
struct TimeGraph {
    system_time: SystemTime,
    real_delta_time: VecDeque<f64>,
    fixed_delta_time: VecDeque<f64>,
    last_fixed_elapsed_time: f64,
    reached_fixed_delta_time: VecDeque<f64>,
}

impl TimeGraph {
    pub fn update(&mut self, real_delta_time: f64, fixed_delta_time: f64) {
        self.fixed_delta_time.pop_front();
        self.fixed_delta_time.push_back(fixed_delta_time);
        self.real_delta_time.pop_front();
        self.real_delta_time.push_back(real_delta_time);
    }

    pub fn update_reached(&mut self, reached_fixed_delta_time: f64) {
        self.reached_fixed_delta_time.pop_front();
        self.reached_fixed_delta_time
            .push_back(reached_fixed_delta_time);
    }
}

impl Default for TimeGraph {
    fn default() -> Self {
        Self {
            system_time: SystemTime::now(),
            real_delta_time: vec![0.0; 120].into(),
            fixed_delta_time: vec![0.0; 120].into(),
            last_fixed_elapsed_time: 0.0,
            reached_fixed_delta_time: vec![0.0; 120].into(),
        }
    }
}

fn update_reached(mut time_graph: ResMut<TimeGraph>) {
    let elapsed = time_graph.system_time.elapsed().unwrap().as_secs_f64();
    let time = elapsed - time_graph.last_fixed_elapsed_time;
    time_graph.update_reached(time);
    time_graph.last_fixed_elapsed_time = elapsed;
}

fn ui(
    mut ctx: EguiContexts,
    mut fixed_time: ResMut<Time<Fixed>>,
    real_time: Res<Time<bevy::time::Real>>,
    mut time_graph: ResMut<TimeGraph>,
) {
    time_graph.update(
        real_time.delta_seconds_f64(),
        fixed_time.delta_seconds_f64(),
    );

    egui::Window::new("Time").show(ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            let mut timestep_hz = 1.0 / fixed_time.timestep().as_secs_f64();
            ui.label("Timestep");
            ui.add(Slider::new(&mut timestep_hz, 8.0..=260.0));
            ui.end_row();
            fixed_time.set_timestep_hz(timestep_hz);
        });

        ui.horizontal(|ui| {
            Plot::new("delta_time")
                .include_x(0.0)
                .include_x(time_graph.fixed_delta_time.len() as f64)
                .include_y(0.0)
                .include_y(50.0)
                .view_aspect(3.0)
                .height(128.0)
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    write_time_line(plot_ui, &time_graph.real_delta_time, "Real");
                    write_time_line(plot_ui, &time_graph.fixed_delta_time, "Fixed");
                    write_time_line(plot_ui, &time_graph.reached_fixed_delta_time, "Reached");
                });
        });

        ui.horizontal(|ui| {
            write_time_info(ui, &time_graph.real_delta_time, "Real");
            write_time_info(ui, &time_graph.fixed_delta_time, "Fixed");
            write_time_info(ui, &time_graph.reached_fixed_delta_time, "Reached");
        });
    });
}

fn write_time_line(plot_ui: &mut PlotUi, data: &VecDeque<f64>, text: &str) {
    let points = PlotPoints::from_parametric_callback(
        |x| (x, data[x as usize] * 1000.0),
        0.0..=(data.len() - 1) as f64,
        data.len(),
    );

    plot_ui.line(Line::new(points).fill(5.0).name(text));
}

fn write_time_info(ui: &mut egui::Ui, data: &VecDeque<f64>, text: &str) {
    ui.label(format!("{text}: {:.2} ms", data.back().unwrap() * 1000.0));
}
