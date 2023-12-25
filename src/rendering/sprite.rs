use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<StandardMaterialSprite>::default());
    }
}

pub type StandardMaterialSprite = ExtendedMaterial<StandardMaterial, SpriteMaterial>;

#[derive(Debug, Clone, Asset, AsBindGroup, TypePath)]
pub struct SpriteMaterial {}

impl MaterialExtension for SpriteMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/sprite.wgsl".into()
    }
}
