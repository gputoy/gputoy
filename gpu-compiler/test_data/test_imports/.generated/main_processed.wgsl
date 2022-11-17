struct MyStruct {
    field: vec3<f32>,
    color: vec4<f32>,
    count: i32,
};
struct FragIn {
  @builtin(position) position: vec4<f32>,
  @location(0) uv: vec2<f32>,
};
@fragment
fn main(in: FragIn) -> @location(0) vec4<f32> {
    TestStruct0;
    TestStruct1; 
    return vec4<f32>(in.uv.x, in.uv.y, 0., 1.);
}