use super::super::utils::vec3::Vec3;
use super::super::ray::Ray;
use super::hit_record::HitRecord;
use super::Hittable;

#[derive(Copy, Clone, Debug)]
pub struct Sphere
{
    pub radius: f32,
    pub center: Vec3
}

impl Sphere
{
    pub fn init(radius: f32, center: Vec3) -> Self { Self { radius, center } }
}

impl Hittable for Sphere
{
    fn get_normal_at(&self, i_pos: Vec3) -> Vec3
    {
        return (i_pos - self.center) / self.radius;
    }

    fn hit(&self, i_ray: &Ray, i_min_d: f32, i_max_d: f32) -> Option<HitRecord>
    {
        let oc = i_ray.origin - self.center;

        // (R(t)-C)(R(t)-C) = r^2
        // R(t) = O + Dt
        // (O+Dt-c)(O+Dt-C) = r^2
        // D·Dt^2 + 2D·(O-C)t + (O-C)·(O-C) - r^2 = 0

        let a = i_ray.direction.dot( i_ray.direction );
        let b = i_ray.direction.dot(oc) * 2.0;
        let c = oc.dot(oc) - self.radius*self.radius;

        let discriminant = b*b - 4.0*a*c;

        if discriminant < 0.0 { return None; }

        let mut distance   = (-b - discriminant.sqrt()) / (2.0 * a) ;

        if distance <= i_max_d && distance >= i_min_d
        {
            let position   = i_ray.at(distance);
            let normal     = self.get_normal_at(position);
            let front_face = HitRecord::is_front_face(i_ray, normal);

            return Some(HitRecord { distance, position, normal, front_face });
        }

        distance = (-b - discriminant.sqrt()) / (2.0 * a) ;

        if distance <= i_max_d && distance >= i_min_d
        {
            let position   = i_ray.at(distance);
            let normal     = self.get_normal_at(position);
            let front_face = HitRecord::is_front_face(i_ray, normal);

            return Some(HitRecord { distance, position, normal, front_face });
        }

        return None
    }
}