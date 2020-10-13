use super::Material;
use super::super::ray::Ray;
use super::super::utils::vec3::Vec3;
use super::super::hittables::hit_record::HitRecord;

#[derive(Copy, Clone, Debug)]
pub struct LambertianMat
{
    pub albedo: Vec3
}

impl Material for LambertianMat
{
    fn scatter(&self,
               i_ray: &Ray,
               i_record: &HitRecord,
               o_attenuation: &mut Vec3) -> Ray
    {
        let scatter_dir = i_record.normal + Vec3::rand_unit();
        *o_attenuation = self.albedo;
        return Ray::new(i_record.position, scatter_dir, i_ray.time);
    }
}