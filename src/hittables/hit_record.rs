use super::super::utils::vec3::Vec3;
use super::super::ray::Ray;
use super::super::materials::Material;

#[derive(Clone, Debug)]
pub struct HitRecord
{
    pub front_face: bool,
    pub distance:   f32,
    pub position:   Vec3,
    pub normal:     Vec3,
    pub p_material: std::rc::Rc<dyn Material>
}

impl HitRecord
{
    pub fn is_front_face(i_ray: &Ray, i_outward_norm: Vec3) -> bool
    {
        return i_ray.direction.dot( i_outward_norm ) < 0.0;
    }
}