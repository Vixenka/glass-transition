use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{
    character::player::{LocalPlayer, Player},
    math::lerp_exponent_in_time,
    network::has_local_player,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PreUpdate, move_camera.run_if(has_local_player));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

#[allow(clippy::type_complexity)]
fn move_camera(
    time: Res<Time>,
    mut camera: Query<(Entity, &mut Transform), (With<Camera>, Without<LocalPlayer>)>,
    mut players: Query<(&mut Player, &Transform), With<LocalPlayer>>,
) {
    let (camera_entity, mut transform) = camera.single_mut();
    let (mut player, player_transform) = players.single_mut();

    let target_position = player_transform.translation + Vec3::new(5.0, 5.0, 5.0);
    transform.translation = transform.translation.lerp(
        target_position,
        lerp_exponent_in_time(2.0, 0.0001, time.delta_seconds()),
    );

    player.attached_camera = Some(camera_entity);
}
