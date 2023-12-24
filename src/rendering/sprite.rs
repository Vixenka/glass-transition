use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<StandardMaterialSprite>::default());
    }
}

pub type StandardMaterialSprite = ExtendedMaterial<StandardMaterial, SpriteMaterial>;

#[derive(Debug, Clone, Copy, PartialEq, ShaderType)]
pub struct SpriteMaterialUniforms {
    pub billboard_size: Vec2,
}

#[derive(Debug, Clone, Asset, AsBindGroup, TypePath)]
pub struct SpriteMaterial {
    #[uniform(100)]
    pub uniforms: SpriteMaterialUniforms,
}

impl MaterialExtension for SpriteMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/sprite.wgsl".into()
    }
}
