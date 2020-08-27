use super::Material;
use super::super::ray::Ray;
use super::super::utils::vec3::Vec3;
use super::super::hittables::hit_record::HitRecord;

#[derive(Copy, Clone, Debug)]
pub struct MetallicMat;

impl Material for MetallicMat
{
    fn scatter(&self,
               _i_ray: &Ray,
               _i_record: &HitRecord,
               _o_attenuation: &mut Vec3,
               _o_ray: &mut Ray) -> bool
    {
        return false;
    }
}