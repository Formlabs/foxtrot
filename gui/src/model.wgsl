struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] normal: vec4<f32>;
};

[[block]]
struct Locals {
    transform: mat4x4<f32>;
};
[[group(0), binding(0)]]
var r_locals: Locals;

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] position: vec4<f32>,
    [[location(1)]] normal: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = r_locals.transform * vec4<f32>(position.xyz, 1.0);
    out.normal = normal;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(abs(in.normal.xyz), 1.0);
}
