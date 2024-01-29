use std::f32::consts::PI;

use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_rapier3d::prelude::*;
use developer_tools::prototype_material::PrototypeMaterial;

pub mod camera;
pub mod character;
pub mod developer_tools;
pub mod math;
pub mod network;

const TIMESTEP: f64 = 1.0 / 60.0;

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_seconds(TIMESTEP))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "GLASS TRANSITION".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                }),
        )
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(RapierPhysicsPlugin::<()>::default().with_physics_scale(1.0))
        .add_plugins(RapierDebugRenderPlugin {
            enabled: false,
            ..default()
        })
        .add_plugins(network::NetworkPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(character::CharacterPlugin)
        .add_plugins(developer_tools::DeveloperToolsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut prototype_materials: ResMut<Assets<PrototypeMaterial>>,
    assets: Res<AssetServer>,
) {
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(25.0, 1.0, 25.0),
        MaterialMeshBundle {
            mesh: meshes.add(shape::Box::new(50.0, 2.0, 50.0).into()),
            material: PrototypeMaterial::get(&mut prototype_materials, &assets, "floor"),
            ..default()
        },
    ));

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(2.0, 1.0, 0.5),
        MaterialMeshBundle {
            transform: Transform::from_xyz(0.0, 1.0, 5.0),
            mesh: meshes.add(shape::Box::new(4.0, 2.0, 1.0).into()),
            material: PrototypeMaterial::get(&mut prototype_materials, &assets, "wall"),
            ..default()
        },
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 32000.0,
            shadows_enabled: false,
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
