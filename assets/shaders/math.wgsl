fn extract_scale(model_matrix: mat4x4f) -> vec3f {
    let x = vec3f(model_matrix[0][0], model_matrix[0][1], model_matrix[0][2]);
    let y = vec3f(model_matrix[1][0], model_matrix[1][1], model_matrix[1][2]);
    let z = vec3f(model_matrix[2][0], model_matrix[2][1], model_matrix[2][2]);
    return vec3f(length(x), length(y), length(z));
}
