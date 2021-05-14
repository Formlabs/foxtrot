[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] in_vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
    if (in_vertex_index == 0u || in_vertex_index == 5u) {
        return vec4<f32>(-1.0, -1.0, 0.0, 1.0);
    } elseif (in_vertex_index == 1u) {
        return vec4<f32>(1.0, -1.0, 0.0, 1.0);
    } elseif (in_vertex_index == 2u || in_vertex_index == 3u) {
        return vec4<f32>(1.0, 1.0, 0.0, 1.0);
    } elseif (in_vertex_index == 4u) {
        return vec4<f32>(-1.0, 1.0, 0.0, 1.0);
    } else {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
}

[[stage(fragment)]]
fn fs_main([[builtin(position)]] coord_in: vec4<f32>) -> [[location(0)]] vec4<f32> {
  return vec4<f32>(coord_in.x / 800.0, coord_in.y / 800.0, 0.0, 1.0);
}
