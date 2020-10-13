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
    t0:                f32, // Shutter open time
    t1:                f32, // Shutter close time
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
    pub fn new(vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32, t0:f32, t1: f32) -> Self
    {
        let h      = (utils::degrees_to_radians(vfov) * 0.5).tan();
        let height = 2.0 * h;
        let width  = aspect_ratio * height;

        let mut result = Self
        {
            t0, t1,
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
        self.left = Vec3::new(0.0, 1.0, 0.0) // World's UP
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
        let hrz       = self.left * self.viewport.width  * self.focus_dist;
        let vrt       = self.up   * self.viewport.height * self.focus_dist;
        let pixel_pos = self.lower_left_corner + (hrz*u + vrt*v);

        let rand_dir = utils::rand_point_in_unit_disk() * self.lens_radius ;
        let offset   = self.left * rand_dir.x() + self.up * rand_dir.y();
        let origin   = self.origin + offset;

        return Ray::new(origin, pixel_pos - origin, utils::rand_f32_in_range(self.t0, self.t1));
    }

    fn recalculate_lower_left_corner(&mut self)
    {
        let hrz = self.left    * self.viewport.width  * self.focus_dist;
        let vrt = self.up      * self.viewport.height * self.focus_dist;
        let dpt = self.forward * self.focal_len       * self.focus_dist;

        self.lower_left_corner = self.origin - hrz*0.5 - vrt*0.5 + dpt;
    }
}


#[cfg(test)]
mod tests
{
    use super::*;

    fn new_test_camera() -> Camera
    {
        let cam_aspect_ratio = 1.0;
        let cam_fov          = utils::radians_to_degrees( 2.0 * (0.5 as f32).atan() );
        let cam_aperture     = 0.0;
        let cam_focus_dist   = 1.0;
        let cam_shutter_t0   = 0.0;
        let cam_shutter_t1   = 1.0;

        return Camera::new(cam_fov,
                           cam_aspect_ratio,
                           cam_aperture,
                           cam_focus_dist,
                           cam_shutter_t0,
                           cam_shutter_t1);
    }

    #[test]
    fn look_at()
    {
        let mut camera = new_test_camera();
        let p = Vec3::rand(-10.0, 10.0);
        camera.look_at( p );

        assert_eq!( camera.forward, p.normalized() );
    }

    #[test]
    fn recalculate_lower_left_corner()
    {
        let mut camera = new_test_camera();
        let llc1 = camera.lower_left_corner;

        // Translation
        let t = Vec3::rand(-10.0, 10.0);
        camera.move_to( t );
        assert_eq!( camera.lower_left_corner, llc1 + t );
        camera.move_to( Vec3::zero() );

        // TODO: Rotation
        //camera.look_at( t );
        //assert_eq!( camera.lower_left_corner, );
    }

    #[test]
    fn get_ray()
    {
        let camera = new_test_camera();

        // To the plane of projection's center
        let ray = camera.get_ray(0.5, 0.5);
        assert_eq!( ray.direction, camera.forward );

        // To the PoP's lower left corner
        let ray = camera.get_ray(0.0, 0.0);
        assert_eq!( ray.direction, camera.lower_left_corner.normalized() );

        // To the PoP's top right corner
        let w   = camera.viewport.width;
        let h   = camera.viewport.height;
        let trc = camera.lower_left_corner + Vec3::new( -w, h, 0.0 );

        let ray = camera.get_ray(1.0, 1.0);
        assert_eq!( ray.direction, trc.normalized() );
    }
}
