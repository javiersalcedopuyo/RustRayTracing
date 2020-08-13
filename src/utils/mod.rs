pub mod ppm;
pub mod vec3;

const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(i_degrees: f32) -> f32 { i_degrees * PI / 180.0 }