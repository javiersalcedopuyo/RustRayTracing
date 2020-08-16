pub mod ppm;
pub mod vec3;

const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(i_degrees: f32) -> f32 { i_degrees * PI / 180.0 }

pub fn rand_point_in_unit_sphere() -> vec3::Vec3
{
    loop
    {
        let p = vec3::Vec3::rand(-1.0, 1.0);
        if p.norm2() >= 1.0 { continue; }
        return p;
    }
}