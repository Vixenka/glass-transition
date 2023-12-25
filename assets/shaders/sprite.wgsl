#import bevy_pbr::{
    mesh_functions,
    mesh_view_bindings,
    skinning,
    morph::morph,
    forward_io::{Vertex, VertexOutput},
    view_transformations::position_world_to_clip,
}
#import bevy_render::instance_index::get_instance_index

#import "shaders/math.wgsl"::extract_scale

struct Billboard {
    world_position: vec4f,
    position: vec4f,
    normal: vec3f,
}

fn calculate_billboard(vertex_position: vec3f, view: mat4x4f, model: mat4x4f) -> Billboard {
    let camera_right = vec3f(view[0][0], view[1][0], view[2][0]);
    let camera_up = vec3f(view[0][1], view[1][1], view[2][1]);
    let camera_front = vec3f(view[0][2], view[1][2], view[2][2]);

    let billboard_center = model * vec4f(0.0, 0.0, 0.0, 1.0);
    let billboard_size = extract_scale(model);

    let world_space_vertex_position = billboard_center.xyz +
        camera_right * vertex_position.x * billboard_size.x +
        camera_up * vertex_position.y * billboard_size.y;

    let view_proj = mesh_view_bindings::view.projection * view;

    let world_position = vec4f(world_space_vertex_position, 0.0);
    let position = mesh_view_bindings::view.view_proj * vec4(world_position.xyz, 1.0);
    let normal = normalize(camera_front);

    return Billboard(world_position, position, normal);
}

@vertex
fn vertex(vertex_no_morph: Vertex) -> VertexOutput {
    var out: VertexOutput;

    var vertex = vertex_no_morph;

    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416 .
    let model = mesh_functions::get_model_matrix(vertex_no_morph.instance_index);

    let billboard = calculate_billboard(vertex.position, mesh_view_bindings::view.inverse_view, model);

#ifdef VERTEX_NORMALS
    out.world_normal = billboard.normal;
#endif

#ifdef VERTEX_POSITIONS
    out.world_position = billboard.world_position;
    out.position = billboard.position;
#endif

#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        model,
        vertex.tangent,
        // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
        // See https://github.com/gfx-rs/naga/issues/2416
        get_instance_index(vertex_no_morph.instance_index)
    );
#endif

#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

#ifdef VERTEX_OUTPUT_INSTANCE_INDEX
    // Use vertex_no_morph.instance_index instead of vertex.instance_index to work around a wgpu dx12 bug.
    // See https://github.com/gfx-rs/naga/issues/2416
    out.instance_index = get_instance_index(vertex_no_morph.instance_index);
#endif

#ifdef BASE_INSTANCE_WORKAROUND
    // Hack: this ensures the push constant is always used, which works around this issue:
    // https://github.com/bevyengine/bevy/issues/10509
    // This can be removed when wgpu 0.19 is released
    out.position.x += min(f32(get_instance_index(0u)), 0.0);
#endif

    return out;
}
