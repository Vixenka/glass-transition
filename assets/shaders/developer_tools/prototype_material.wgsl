#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) local_position: vec3<f32>,
    @location(1) local_normal: vec3<f32>,
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.position = mesh_position_local_to_clip(get_model_matrix(0u), vec4<f32>(vertex.position, 1.0));
    out.local_position = vertex.position;
    out.local_normal = vertex.normal;
    return out;
}

struct PrototypeMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0) var<uniform> material: PrototypeMaterial;
@group(1) @binding(1) var base_texture: texture_2d<f32>;
@group(1) @binding(2) var base_sampler: sampler;

fn sample_triplanar(texture: texture_2d<f32>, texture_sampler: sampler, position: vec3<f32>, normal: vec3<f32>) -> vec4<f32> {
    let threshold = 0.8;

    var blend_weights = pow(abs(normal), vec3(3.0, 3.0, 3.0));
    blend_weights /= dot(blend_weights, vec3(1.0, 1.0, 1.0));

    var finalWeights = vec3(0.0, 0.0, 0.0);
    if blend_weights.x > threshold {
        finalWeights.x = blend_weights.x;
    }
    if blend_weights.y > threshold {
        finalWeights.y = blend_weights.y;
    }
    if blend_weights.z > threshold {
        finalWeights.z = blend_weights.z;
    }
    finalWeights /= finalWeights.x + finalWeights.y + finalWeights.z;

    var result = vec4(0.0, 0.0, 0.0, 0.0);
    var temp = textureSample(texture, texture_sampler, position.zy);
    if finalWeights.x > 0.0 {
        result += finalWeights.x * temp;
    }
    temp = textureSample(texture, texture_sampler, position.xz);
    if finalWeights.y > 0.0 {
        result += finalWeights.y * temp;
    }
    temp = textureSample(texture, texture_sampler, position.xy);
    if finalWeights.z > 0.0 {
        result += finalWeights.z * temp;
    }

    return result;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let position = mesh.local_position - floor(mesh.local_position);
    let texel = sample_triplanar(base_texture, base_sampler, position, mesh.local_normal);

    if texel.a < 0.4 {
        return material.color;
    }
    return texel;
}
