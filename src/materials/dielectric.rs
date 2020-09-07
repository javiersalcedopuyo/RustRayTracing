use super::Material;
use super::super::ray::Ray;
use super::super::utils::vec3::Vec3;
use super::super::hittables::hit_record::HitRecord;

#[derive(Copy, Clone, Debug)]
pub struct DielectricMat
{
    refraction_idx: f32,
    albedo: Vec3
}

impl DielectricMat
{
    pub fn new(refraction_idx: f32, albedo: Vec3) -> Self { Self{refraction_idx, albedo} }

    fn schlick_approx(cosine: f32, refraction_idx: f32) -> f32
    {
        let mut r0 = (1.0 - refraction_idx) / (1.0 + refraction_idx);
        r0 = r0*r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for DielectricMat
{
    fn scatter(&self,
               i_ray: &Ray,
               i_record: &HitRecord,
               o_attenuation: &mut Vec3) -> Ray
    {
        let eta = if i_record.front_face { 1.0 / self.refraction_idx }
                  else { self.refraction_idx };

        let input_ray_dir = i_ray.direction.normalized();

        let candidate = -input_ray_dir.dot(i_record.normal);
        let cos_theta = if candidate < 1.0 { candidate } else { 1.0 }; // std::cmp::min doesn't work with floats
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let reflect_prob = Self::schlick_approx(cos_theta, eta);

        let resulting_ray_dir = if eta * sin_theta > 1.0 || rand::random::<f32>() < reflect_prob {
                                    input_ray_dir.reflect(i_record.normal)
                                } else {
                                    input_ray_dir.refract(i_record.normal, eta)
                                };

        *o_attenuation = self.albedo;
        return Ray::new(i_record.position, resulting_ray_dir);
    }
}