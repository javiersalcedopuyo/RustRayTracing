use super::super::utils::vec3::Vec3;
use super::super::ray::Ray;

pub struct HitRecord
{
    pub position: Vec3,
    pub normal:   Vec3,
    pub distance: f32
}

pub trait Hittable
{
    fn hit(&self, i_ray: &Ray, i_min_d: f32, i_max_d: f32) -> HitRecord;
    fn get_normal(&self, i_pos: Vec3) -> Vec3;
}