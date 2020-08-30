pub mod debug;
pub mod metallic;
pub mod lambertian;
pub mod dielectric;

use super::ray::Ray;
use super::utils::vec3::Vec3;
use super::hittables::hit_record::HitRecord;

pub trait Material: std::fmt::Debug
{
    fn scatter(&self,
               i_ray: &Ray,
               i_record: &HitRecord,
               o_attenuation: &mut Vec3,
               o_ray: &mut Ray) -> bool;
}