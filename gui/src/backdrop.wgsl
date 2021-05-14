struct VertexOutput {
    [[location(0)]] color: vec4<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    if (in_vertex_index == 0u || in_vertex_index == 5u) {
        out.color = vec4<f32>(0.1, 0.1, 0.1, 1.0);
        out.position = vec4<f32>(-1.0, -1.0, 0.0, 1.0);
    } elseif (in_vertex_index == 1u) {
        out.color = vec4<f32>(0.1, 0.1, 0.1, 1.0);
        out.position = vec4<f32>(1.0, -1.0, 0.0, 1.0);
    } elseif (in_vertex_index == 2u || in_vertex_index == 3u) {
        out.color = vec4<f32>(0.4, 0.1, 0.1, 1.0);
        out.position = vec4<f32>(1.0, 1.0, 0.0, 1.0);
    } elseif (in_vertex_index == 4u) {
        out.color = vec4<f32>(0.4, 0.4, 0.1, 1.0);
        out.position = vec4<f32>(-1.0, 1.0, 0.0, 1.0);
    } else {
        out.color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
        out.position = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}
