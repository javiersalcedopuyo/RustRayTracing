use super::super::utils::vec3::Vec3;
use super::super::ray::Ray;
use super::hittable::{HitRecord, Hittable};

pub struct Sphere
{
    pub radius: f32,
    pub center: Vec3
}

impl Sphere
{
    //pub fn new() -> Self { Self { radius: 1.0, center: Vec3::new() } }
    pub fn init(radius: f32, center: Vec3) -> Self { Self { radius, center } }
}

impl Hittable for Sphere
{
    fn get_normal(&self, i_pos: Vec3) -> Vec3
    {
        return (i_pos - self.center) / self.radius;
    }

    fn hit(&self, i_ray: &Ray, i_min_d: f32, i_max_d: f32) -> HitRecord
    {
        let oc = i_ray.origin - self.center;

        // (R(t)-C)(R(t)-C) = r^2
        // R(t) = O + Dt
        // (O+Dt-c)(O+Dt-C) = r^2
        // D·Dt^2 + 2D·(O-C)t + (O-C)·(O-C) - r^2 = 0

        let a = i_ray.direction.dot( i_ray.direction );
        let b = i_ray.direction.dot(oc) * 2.0;
        let c = oc.dot(oc) - self.radius*self.radius;

        let discriminant = b*b - 4.0*a*c;

        let dist = if discriminant < 0.0 { -1.0 }
                   else { (-b - discriminant.sqrt()) / (2.0 * a) };

        let pos  = i_ray.at(dist);

        return HitRecord
        {
            distance: if dist >= i_min_d && dist <= i_max_d { dist } else { -1.0 },
            position: pos,
            normal:   self.get_normal(pos)
        };
    }
}