use super::utils::vec3::Vec3;
use super::ray::Ray;

use super::utils;

pub struct Rect
{
    pub width:  f32,
    pub height: f32
}

pub struct Camera
{
    pub lens_radius:   f32,
    pub focus_dist:    f32,
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
    pub fn new(vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self
    {
        let h      = (utils::degrees_to_radians(vfov) * 0.5).tan();
        let height = 2.0 * h;
        let width  = aspect_ratio * height;

        let mut result = Self
        {
            focus_dist,
            lens_radius: aperture*0.5,
            focal_len  : 1.0,
            viewport   : Rect{width, height},
            origin     : Vec3::zero(),
            forward    : Vec3::new(0.0, 0.0, 1.0),
            up         : Vec3::new(0.0, 1.0, 0.0),
            left       : Vec3::new(-1.0, 0.0, 0.0),
            lower_left_corner: Vec3::zero()
        };
        result.recalculate_lower_left_corner();

        return result;
    }

    //pub fn resize(&mut self, width: f32, height: f32)
    //{
    //    self.viewport = Rect{width, height};
    //    self.recalculate_lower_left_corner();
    //}

    pub fn look_at(&mut self, target: Vec3)
    {
        let look_dir = (target - self.origin).normalized();

        self.forward = look_dir;
        self.left = Vec3::new(0.0, 1.0, 0.0)
                        .cross(look_dir)
                        .normalized();

        self.up = look_dir.cross(self.left);

        self.recalculate_lower_left_corner();
    }

    pub fn move_to(&mut self, new_pos: Vec3)
    {
        self.origin = new_pos;
        self.recalculate_lower_left_corner();
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray
    {
        let hrz       = self.left * self.viewport.width;
        let vrt       = self.up   * self.viewport.height;
        let pixel_pos = self.lower_left_corner + (hrz*u + vrt*v);

        //let rd     = utils::rand_point_in_unit_disk() * self.lens_radius ;
        //let offset = self.left * rd.x() + self.up * rd.y();

        return Ray::new(self.origin, pixel_pos - self.origin);
    }

    fn recalculate_lower_left_corner(&mut self)
    {
        let hrz = self.left    * self.viewport.width;
        let vrt = self.up      * self.viewport.height;
        let dpt = self.forward * self.focal_len;

        self.lower_left_corner = self.origin - hrz*0.5 - vrt*0.5 + dpt;
    }
}