use super::super::utils::vec3::Vec3;
use super::super::ray::Ray;

pub struct HitRecord
{
    pub front_face: bool,
    pub distance:   f32,
    pub position:   Vec3,
    pub normal:     Vec3,
}

impl HitRecord
{
    pub fn null_hit() -> Self
    {
        Self { front_face: false, distance: -1.0, position: Vec3::zero(), normal: Vec3::zero() }
    }

    pub fn is_front_face(i_ray: &Ray, i_outward_norm: Vec3) -> bool
    {
        return i_ray.direction.dot( i_outward_norm ) < 0.0;
    }

    pub fn is_null(&self) -> bool { self.normal == Vec3::zero() }
}

pub trait Hittable
{
    fn hit(&self, i_ray: &Ray, i_min_d: f32, i_max_d: f32) -> HitRecord;
    fn get_normal(&self, i_pos: Vec3) -> Vec3;
}