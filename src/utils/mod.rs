pub mod ppm;
pub mod vec3;

use vec3::Vec3;

#[allow(dead_code)]
const PI:  f32 = 3.1415926535897932385;
#[allow(dead_code)]
const TAU: f32 = 6.2831853071;
#[allow(dead_code)]
pub fn degrees_to_radians(i_degrees: f32) -> f32 { i_degrees * PI / 180.0 }

#[allow(dead_code)]
pub fn rand_point_in_unit_sphere() -> Vec3
{
    loop
    {
        let p = Vec3::rand(-1.0, 1.0);
        if p.norm2() >= 1.0 { continue; }
        return p;
    }
}

#[allow(dead_code)]
pub fn rand_point_in_unit_hemisphere(i_normal: Vec3) -> Vec3
{
    let point_in_sphere = rand_point_in_unit_sphere();
    return point_in_sphere * 1.0_f32.copysign( point_in_sphere.dot(i_normal) );
}