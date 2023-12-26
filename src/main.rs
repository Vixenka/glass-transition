use std::{f32::consts::PI, fs::File, path::PathBuf};

use bevy::{log::LogPlugin, pbr::CascadeShadowConfigBuilder, prelude::*};
use bevy_rapier3d::prelude::*;
use clap::Parser;
use tracing_subscriber::{prelude::*, EnvFilter};

use developer_tools::prototype_material::PrototypeMaterial;

pub mod camera;
pub mod character;
pub mod developer_tools;
pub mod math;
pub mod network;

#[derive(Parser)]
pub struct CommandLineArgs {
    /// Log file to output logs to.
    ///
    /// By default, logs are only printed to stderr. If this is specified, they will also be printed
    /// out to a file. Note that the file specified will get overwritten.
    ///
    /// The filter used for logging to the file can be customized by setting the `GT_FILE_LOG`
    /// environment variable.
    #[clap(long)]
    pub log_file: Option<PathBuf>,
}

const TIMESTEP: f64 = 1.0 / 60.0;

#[bevy_main]
fn main() {
    let args = CommandLineArgs::parse();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .without_time()
                .with_writer(std::io::stderr)
                .with_filter(
                    EnvFilter::builder()
                        .with_env_var("GT_LOG")
                        .try_from_env()
                        .unwrap_or_else(|_| EnvFilter::new("info,wgpu=error,naga=warn")),
                ),
        )
        .with(args.log_file.and_then(|path| {
            File::create("glass-transition.log").ok().map(|file| {
                tracing_subscriber::fmt::layer()
                    .with_ansi(false)
                    .with_writer(file)
                    .with_filter(
                        EnvFilter::builder()
                            .with_env_var("GT_FILE_LOG")
                            .try_from_env()
                            .unwrap_or_else(|_| EnvFilter::new("info,wgpu=error,naga=warn")),
                    )
            })
        }))
        .init();

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
                })
                .disable::<LogPlugin>(),
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
