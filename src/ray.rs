use super::utils::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray
{
    pub time:      f32,
    pub origin:    Vec3,
    pub direction: Vec3
}

impl Ray
{
    pub fn new(origin: Vec3, d: Vec3, time: f32) -> Ray
    {
        Ray {origin,
             direction: d.normalized(),
             time}
    }

    pub fn at(&self, t: f32) -> Vec3 { return self.origin + self.direction * t; }
}
