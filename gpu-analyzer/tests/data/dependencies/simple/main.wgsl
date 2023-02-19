@fragment
fn main(in: @import FragIn) -> @location(0) vec4<f32> {
    return vec4<f32>(in.uv.x, in.uv.y, 0., 1.);
}