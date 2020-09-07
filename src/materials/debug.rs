use super::Material;
use super::super::ray::Ray;
use super::super::utils::{ vec3::Vec3, rand_point_in_unit_hemisphere };
use super::super::hittables::hit_record::HitRecord;

#[derive(Copy, Clone, Debug)]
pub struct DebugMat;

impl Material for DebugMat
{
    fn scatter(&self,
               _i_ray: &Ray,
               i_record: &HitRecord,
               o_attenuation: &mut Vec3) -> Ray
    {
        let scatter_dir = rand_point_in_unit_hemisphere(i_record.normal);
        *o_attenuation  = Vec3::debug_color();
        return Ray::new(i_record.position, scatter_dir);
    }
}