#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3f,
    @location(1) normal: vec3f
}

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) scaled_local_position: vec3f,
    @location(1) local_normal: vec3f,
}

fn extract_scale(model_matrix: mat4x4f) -> vec3f {
    let x = vec3f(model_matrix[0][0], model_matrix[0][1], model_matrix[0][2]);
    let y = vec3f(model_matrix[1][0], model_matrix[1][1], model_matrix[1][2]);
    let z = vec3f(model_matrix[2][0], model_matrix[2][1], model_matrix[2][2]);
    return vec3f(length(x), length(y), length(z));
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = get_model_matrix(vertex.instance_index);
    out.position = mesh_position_local_to_clip(model_matrix, vec4f(vertex.position, 1.0));
    out.scaled_local_position = vertex.position * extract_scale(model_matrix);
    out.local_normal = vertex.normal;

    return out;
}

struct PrototypeMaterial {
    color: vec4f,
};

@group(1) @binding(0) var<uniform> material: PrototypeMaterial;
@group(1) @binding(1) var base_texture: texture_2d<f32>;
@group(1) @binding(2) var base_sampler: sampler;

fn sample_triplanar(texture: texture_2d<f32>, texture_sampler: sampler, position: vec3f, normal: vec3f) -> vec4f {
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
fn fragment(mesh: VertexOutput) -> @location(0) vec4f {
    let position = mesh.scaled_local_position - floor(mesh.scaled_local_position);
    let texel = sample_triplanar(base_texture, base_sampler, position, mesh.local_normal);

    if texel.a < 0.4 {
        return material.color;
    }
    return texel;
}
