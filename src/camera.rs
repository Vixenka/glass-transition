use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{character::player::LocalPlayer, network::has_local_player};

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

fn move_camera(
    time: Res<Time>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<LocalPlayer>)>,
    players: Query<&Transform, With<LocalPlayer>>,
) {
    let mut transform = camera.single_mut();
    let player_transform = players.single();

    let target_position = player_transform.translation + Vec3::new(5.0, 5.0, 5.0);
    transform.translation = transform
        .translation
        .lerp(target_position, time.delta_seconds());
}
