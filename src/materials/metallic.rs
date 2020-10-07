use super::Material;
use super::super::ray::Ray;
use super::super::{utils, utils::vec3::Vec3};
use super::super::hittables::hit_record::HitRecord;

#[derive(Copy, Clone, Debug)]
pub struct MetallicMat
{
    roughness: f32,
    albedo: Vec3
}

impl MetallicMat
{
    pub fn new(roughness: f32, albedo: Vec3) -> Self
    {
        let roughness = if roughness > 1.0 { 1.0 } else if roughness < 0.0 { 0.0 } else { roughness };
        Self { roughness, albedo }
    }
}

impl Material for MetallicMat
{
    fn scatter(&self,
               i_ray: &Ray,
               i_record: &HitRecord,
               o_attenuation: &mut Vec3) -> Ray
    {
        let reflected  = i_ray.direction
                             .normalized()
                             .reflect( i_record.normal );

        let offset     = utils::rand_point_in_unit_sphere() * self.roughness;
        *o_attenuation = self.albedo;

        return Ray::new(i_record.position, reflected + offset, i_ray.time);
    }
}