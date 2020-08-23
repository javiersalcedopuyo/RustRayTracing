pub mod hit_record;
pub mod sphere;

use hit_record::HitRecord;

use super::ray::Ray;
use super::utils::vec3::Vec3;

pub trait Hittable
{
    fn hit(&self, i_ray: &Ray, i_min_d: f32, i_max_d: f32) -> Option<HitRecord>;
    fn get_normal_at(&self, i_pos: Vec3) -> Vec3;
}

pub enum Hittables
{
    Sphere( sphere::Sphere ),
    Cube // TODO
}

impl Hittable for Hittables
{
    fn hit(&self, i_ray: &Ray, i_min_d: f32, i_max_d: f32) -> Option<HitRecord>
    {
        match self
        {
            Self::Sphere(s) => s.hit(i_ray, i_min_d, i_max_d),
            _ => None
        }
    }

    fn get_normal_at(&self, i_pos: Vec3) -> Vec3
    {
        match self
        {
            Self::Sphere(s) => s.get_normal_at(i_pos),
            _ => Vec3::zero()
        }
    }
}