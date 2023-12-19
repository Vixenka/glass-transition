use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*, render::camera::ScalingMode};
use bevy_rapier3d::prelude::*;
use character::player::PlayerBundle;

pub mod character;
pub mod developer_tools;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "GLASS TRANSITION".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            RapierPhysicsPlugin::<()>::default()
                .in_fixed_schedule()
                .with_physics_scale(1.0),
        )
        .add_plugins(RapierDebugRenderPlugin {
            enabled: true,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, character::ground_characters)
        .add_systems(FixedUpdate, character::player::control)
        .add_systems(FixedUpdate, character::move_characters)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(2.5, 1.0, 2.5),
        PbrBundle {
            mesh: meshes.add(shape::Box::new(5.0, 2.0, 5.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
    ));

    commands.spawn((
        PlayerBundle::new(Transform::from_xyz(0.0, 3.0, 0.0)),
        meshes.add(
            shape::Cylinder {
                radius: character::player::RADIUS,
                height: character::player::HALF_HEIGHT * 2.0,
                resolution: 16,
                segments: 1,
            }
            .into(),
        ),
        materials.add(Color::WHITE.into()),
        VisibilityBundle::default(),
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            PI / 2.,
            -PI / 4.,
        )),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 7.0,
            maximum_distance: 25.0,
            ..default()
        }
        .into(),
        ..default()
    });
}
