use bevy::prelude::*;

use crate::character::player::interaction_point::{InteractionPoint, TargetedInteractionPoint};

use super::tool_enabled;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            debug_lines.run_if(tool_enabled(|tools| tools.interaction)),
        );
    }
}

fn debug_lines(
    mut gizmos: Gizmos,
    players: Query<(&Transform, &TargetedInteractionPoint)>,
    points: Query<&Transform, With<InteractionPoint>>,
) {
    for (player_transform, targeted_point) in players.iter() {
        if let Ok(target_transform) = points.get(targeted_point.target) {
            gizmos.line(
                player_transform.translation,
                target_transform.translation,
                Color::GREEN,
            );
        }
    }
}
