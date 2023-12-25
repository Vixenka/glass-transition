use bevy::{prelude::*, window::PrimaryWindow};
use bevy_replicon::replicon_core::replication_rules::AppReplicationExt;
use serde::{Deserialize, Serialize};

use crate::network::has_local_player;

use super::{LocalPlayer, Player};

pub struct InteractionPointPlugin;

impl Plugin for InteractionPointPlugin {
    fn build(&self, app: &mut App) {
        app.replicate::<InteractionPoint>()
            .add_systems(PreUpdate, find.run_if(has_local_player));
    }
}

#[derive(Component, Deserialize, Serialize)]
pub struct InteractionPoint;

#[derive(Component)]
pub struct TargetedInteractionPoint {
    pub target: Entity,
}

fn find(
    mut commands: Commands,
    players: Query<(Entity, &Player, &Transform), With<LocalPlayer>>,
    cameras: Query<(&GlobalTransform, &Camera)>,
    window: Query<&Window, With<PrimaryWindow>>,
    points: Query<(Entity, &Transform), With<InteractionPoint>>,
) {
    for (player_entity, player, player_transform) in players.iter() {
        let interest_point = match get_interest_point(player, player_transform, &cameras, &window) {
            Some(point) => point,
            None => continue,
        };

        let mut closest = None;
        let mut closest_distance = f32::MAX;
        for (point_entity, point_transform) in points.iter() {
            let distance = point_transform.translation.distance(interest_point);
            if distance < closest_distance {
                closest = Some(point_entity);
                closest_distance = distance;
            }
        }

        if let Some(closest) = closest {
            commands
                .entity(player_entity)
                .insert(TargetedInteractionPoint { target: closest });
        } else {
            commands
                .entity(player_entity)
                .remove::<TargetedInteractionPoint>();
        }
    }
}

fn get_interest_point(
    player: &Player,
    player_transform: &Transform,
    cameras: &Query<(&GlobalTransform, &Camera)>,
    window: &Query<&Window, With<PrimaryWindow>>,
) -> Option<Vec3> {
    let camera_entity = player.attached_camera?;
    let (camera_transform, camera) = cameras.get(camera_entity).ok()?;
    let cursor_position = window.single().cursor_position()?;
    let ray = camera.viewport_to_world(camera_transform, cursor_position)?;

    let distance = ray.intersect_plane(player_transform.translation, Vec3::Y)?;
    Some(ray.get_point(distance))
}
