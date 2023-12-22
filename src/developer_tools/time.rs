use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Slider},
    EguiContexts,
};
use egui_plot::{Line, Plot, PlotPoints};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeGraph>().add_systems(Update, ui);
    }
}

#[derive(Debug, Clone, Resource)]
pub struct TimeGraph {
    delta_time: VecDeque<f32>,
}

impl TimeGraph {
    pub fn update(&mut self, delta_time: f32) {
        self.delta_time.pop_front();
        self.delta_time.push_back(delta_time);
    }
}

impl Default for TimeGraph {
    fn default() -> Self {
        Self {
            delta_time: vec![0.0; 120].into(),
        }
    }
}

pub fn ui(
    mut ctx: EguiContexts,
    mut fixed_time: ResMut<Time<Fixed>>,
    real_time: Res<Time<bevy::time::Real>>,
    mut time_graph: ResMut<TimeGraph>,
) {
    time_graph.update(real_time.delta_seconds());

    egui::Window::new("Time").show(ctx.ctx_mut(), |ui| {
        let mut timestep_hz = 1.0 / fixed_time.timestep().as_secs_f64();
        ui.add(Slider::new(&mut timestep_hz, 1.0..=144.0).text("Timestep"));
        fixed_time.set_timestep_hz(timestep_hz);

        let plot_points = PlotPoints::from_parametric_callback(
            |x| (x, time_graph.delta_time[x as usize] as f64 * 1000.0),
            0.0..=(time_graph.delta_time.len() - 1) as f64,
            time_graph.delta_time.len(),
        );
        Plot::new("delta_time")
            .include_x(0.0)
            .include_x(time_graph.delta_time.len() as f64)
            .include_y(0.0)
            .include_y(20.0)
            .view_aspect(3.0)
            .height(128.0)
            .show(ui, |plot_ui| plot_ui.line(Line::new(plot_points)));
    });
}
