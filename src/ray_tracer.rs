use rand::random;
use super::scene_generator;
use super::ray::Ray;
use super::hittables::{ Hittable,
                        hit_record::HitRecord };
use super::camera::{ Rect, Camera };
use super::utils::{ vec3::Vec3,
                    ppm::ImagePPM };

const MAX_DEPTH:   i32 = 50;
const SHADOW_BIAS: f32 = 0.001;

pub struct RayTracer
{
    sample_count: i32,
    output_size:  Rect,
    camera:       Camera,
    scene:        Vec<Box<dyn Hittable>>
}

impl RayTracer
{
    // PUBLIC
    // TODO: Customizable constructors
    pub fn new() -> Self
    {
        let w = 800;
        let h = 600;

        #[cfg(debug_assertions)]
            let sample_count = 1;
        #[cfg(not(debug_assertions))]
            let sample_count = 64;

        let cam_aspect_ratio = (w as f32) / (h as f32);
        let cam_fov          = 90.0;
        let cam_aperture     = 0.1;
        let cam_target       = Vec3::zero();
        let cam_pos          = Vec3::new(7.5, 2.0, -3.0);
        let cam_focus_dist   = (cam_pos - cam_target).norm();
        let cam_shutter_t0   = 0.0;
        let cam_shutter_t1   = 1.0;

        let mut camera = Camera::new(cam_fov,
                                     cam_aspect_ratio,
                                     cam_aperture,
                                     cam_focus_dist,
                                     cam_shutter_t0,
                                     cam_shutter_t1);

        camera.move_to( cam_pos );
        camera.look_at( cam_target );

        let scene = scene_generator::rand();
        //let scene = scene_generator::simple();

        Self{sample_count,
             output_size: Rect{width: w as f32, height: h as f32},
             camera,
             scene}
    }

    pub fn render(&self) -> ImagePPM
    {
        let mut result = ImagePPM::new_filled(self.output_size.width  as u32,
                                              self.output_size.height as u32,
                                              Vec3::zero());

        let h = self.output_size.height as u32;
        let w = self.output_size.width  as u32;

        for y in 0..h
        {
            #[cfg(debug_assertions)]
                println!("Scanline {} / {}", y+1, h);

            for x in 0..w
            {
                let mut color = Vec3::zero();
                for _ in 0..self.sample_count
                {
                    let offset = if self.sample_count > 1 { random::<f32>() } else { 0.0 };
                    // * 0,0 is lower left
                    let u = (x as f32 + offset) / (w-1) as f32;
                    let v = ((h-y) as f32 + offset) / (h-1) as f32;

                    color += self.compute_ray(self.camera.get_ray(u, v),
                                              &self.scene);
                }

                // Gamma2 correction
                color = (color * (1.0 / self.sample_count as f32)).sqrt();
                result.set_pixel(x, y, color);
            }
        }
        return result;
    }

    // PRIVATE
    fn sample_skybox(i_ray: &Ray) -> Vec3
    {
        let dir = i_ray.direction.normalized();
        let t   = 0.5 * (dir.y() + 1.0);

        return Vec3::lerp(Vec3::one()*0.75, Vec3::new(0.0, 0.3, 1.0), t);
    }

    fn compute_ray(&self, mut i_ray: Ray, i_scene: &Vec<Box<dyn Hittable>>) -> Vec3
    {
        let mut depth  = MAX_DEPTH;
        let mut result = Vec3::one();
        while depth > 0
        {
            let mut closest_hit: Option<HitRecord> = None;

            for obj in i_scene
            {
                let hit = obj.hit(&i_ray, SHADOW_BIAS, 10.0);

                if hit.is_none() { continue; }

                let hit = hit.unwrap();
                let ch  = closest_hit.unwrap_or( hit.clone() );

                closest_hit = if hit.distance < ch.distance { Some(hit) }
                              else { Some(ch) };
            }

            if closest_hit.is_some()
            {
                let hit = closest_hit.unwrap();
                let mut attenuation = Vec3::one();

                i_ray = hit.p_material.scatter(&i_ray, &hit, &mut attenuation);

                result *= attenuation;
                depth  -= 1;
            }
            else { return result * Self::sample_skybox(&i_ray); }
        }

        return Vec3::zero();
    }
}