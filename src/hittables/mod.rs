pub mod hit_record;
pub mod sphere;

use hit_record::HitRecord;

use super::ray::Ray;
use super::utils::vec3::Vec3;

pub trait Hittable
{
    fn hit(&self, i_ray: &Ray, i_min_d: f32, i_max_d: f32) -> Option<HitRecord>;
    fn get_normal_at(&self, i_pos: Vec3) -> Vec3;
    fn get_updated_position(&self, i_time: f32) -> Vec3;
}