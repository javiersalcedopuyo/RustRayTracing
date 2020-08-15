use super::utils::vec3::Vec3;
use super::ray::Ray;

pub struct Rect
{
    pub width:  f32,
    pub height: f32
}

pub struct Camera
{
    pub focal_len:     f32,
    pub viewport:      Rect,
    pub origin:        Vec3,
    pub forward:       Vec3,
    pub up:            Vec3,
    pub left:          Vec3,
    lower_left_corner: Vec3
}

impl Camera
{
    pub fn new() -> Self
    {
        let mut result = Self
        {
            focal_len : 1.0,
            viewport  : Rect{width: 1.0, height: 1.0},
            origin    : Vec3::zero(),
            forward   : Vec3::new(0.0, 0.0, 1.0),
            up        : Vec3::new(0.0, 1.0, 0.0),
            left      : Vec3::new(-1.0, 0.0, 0.0),
            lower_left_corner : Vec3::zero()
        };

        result.resize(1.0, 1.0);

        return result;
    }

    pub fn resize(&mut self, width: f32, height: f32)
    {
        self.viewport = Rect{width, height};

        let hrz = self.left    * self.viewport.width;
        let vrt = self.up      * self.viewport.height;
        let dpt = self.forward * self.focal_len;

        self.lower_left_corner = self.origin + hrz*0.5 - vrt*0.5 + dpt;
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray
    {
        let hrz       = -self.left * self.viewport.width;
        let vrt       = self.up   * self.viewport.height;
        let pixel_pos = self.lower_left_corner + hrz*u + vrt*v;

        return Ray::new(self.origin, pixel_pos - self.origin);
    }
}