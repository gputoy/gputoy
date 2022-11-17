struct FragIn {
  @builtin(position) position: vec4<f32>,
  @location(0) uv: vec2<f32>,
};
@vertex
fn main(@builtin(vertex_index) v_index: u32) -> FragIn {
    var pos = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0)
    );
    var output: FragIn;
    output.position = vec4<f32>(pos[v_index], 0.0, 1.0);
    output.uv = pos[v_index] * 0.5 + 0.5;
    return output;
}