#import bevy_render::instance_index::get_instance_index
#import bevy_pbr::{
    mesh_functions::{get_model_matrix, mesh_position_local_to_clip, mesh_position_local_to_world, mesh_normal_local_to_world},
    pbr_types::pbr_input_new,
    pbr_functions::{apply_pbr_lighting, calculate_view, prepare_world_normal},
    mesh_bindings::mesh,
    mesh_view_bindings::view,
}

#import "shaders/math.wgsl"::extract_scale

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3f,
    @location(1) normal: vec3f
}

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) scaled_local_position: vec3f,
    @location(1) local_normal: vec3f,
    @location(2) world_position: vec4f,
    @location(3) world_normal: vec3f,
    @location(4) @interpolate(flat) instance_index: u32,
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let model_matrix = get_model_matrix(vertex.instance_index);
    out.position = mesh_position_local_to_clip(model_matrix, vec4f(vertex.position, 1.0));
    out.scaled_local_position = vertex.position * extract_scale(model_matrix);
    out.local_normal = vertex.normal;
    out.world_position = mesh_position_local_to_world(model_matrix, vec4f(vertex.position, 1.0));
    out.world_normal = mesh_normal_local_to_world(vertex.normal, get_instance_index(vertex.instance_index));
    out.instance_index = vertex.instance_index;

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
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> @location(0) vec4f {
    let position = in.scaled_local_position - floor(in.scaled_local_position);
    let texel = sample_triplanar(base_texture, base_sampler, position, in.local_normal);

    var color: vec4f;
    if texel.a < 0.4 {
        color = material.color;
    } else {
        color = texel;
    }

    var pbr_input = pbr_input_new();
    pbr_input.material.base_color = color;
    pbr_input.world_position = in.world_position;
    pbr_input.frag_coord = in.position;
    pbr_input.flags = mesh[in.instance_index].flags;
    pbr_input.is_orthographic = view.projection[3].w == 1.0;
    pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);
    pbr_input.world_normal = prepare_world_normal(in.world_normal, false, is_front);
    pbr_input.N = normalize(pbr_input.world_normal);

    return apply_pbr_lighting(pbr_input);
}
