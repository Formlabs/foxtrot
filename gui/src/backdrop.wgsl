struct VertexOutput {
    [[location(0)]] color: vec4<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    var c1: vec4<f32> = vec4<f32>(0.05, 0.06, 0.10, 1.0);
    var c2: vec4<f32> = vec4<f32>(0.17, 0.22, 0.29, 1.0);
    if (in_vertex_index == 0u || in_vertex_index == 5u) {
        out.color = c1;
        out.position = vec4<f32>(-1.0, -1.0, 0.0, 1.0);
    } elseif (in_vertex_index == 1u) {
        out.color = c1;
        out.position = vec4<f32>(1.0, -1.0, 0.0, 1.0);
    } elseif (in_vertex_index == 2u || in_vertex_index == 3u) {
        out.color = c2;
        out.position = vec4<f32>(1.0, 1.0, 0.0, 1.0);
    } elseif (in_vertex_index == 4u) {
        out.color = c2;
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
