mod utils;
mod ray;
mod camera;
mod hittables;
mod materials;
mod rand_scene;

use rand::random;
use utils::ppm::ImagePPM;
use utils::vec3::Vec3;
use ray::Ray;
use camera::Camera;
use hittables::hit_record::HitRecord;
use hittables::{Hittable, Intersectionable};

use std::time::Instant;

const SHADOW_BIAS: f32 = 0.001;

fn sample_skybox(i_ray: &Ray) -> Vec3
{
    let dir = i_ray.direction.normalized();
    let t   = 0.5 * (dir.y() + 1.0);

    return Vec3::lerp(Vec3::one()*0.75, Vec3::new(0.0, 0.3, 1.0), t);
}

fn compute_ray(i_ray: Ray, i_scene: &Vec<Intersectionable>, i_depth: i32) -> Vec3
{
    if i_depth <= 0 { return Vec3::zero(); }

    let mut closest_hit: Option<HitRecord> = None;
    for obj in i_scene
    {
        let hit = obj.hit(&i_ray, SHADOW_BIAS, 10.0);

        if hit.is_none() { continue; }

        let hit = hit.unwrap();
        let ch  = if closest_hit.is_some() { closest_hit.unwrap() } else { hit.clone() };

        closest_hit = if hit.distance < ch.distance { Some(hit) } else { Some(ch) };
    }

    if closest_hit.is_some()
    {
        let hit = closest_hit.unwrap();

        let mut attenuation   = Vec3::one();
        let mut scattered_ray = Ray::new(Vec3::zero(), Vec3::zero());

        if hit.p_material.scatter(&i_ray, &hit, &mut attenuation, &mut scattered_ray)
        {
            return compute_ray(scattered_ray, i_scene, i_depth-1) * attenuation;
        }

        return Vec3::zero();
    }

    return sample_skybox(&i_ray);
}

fn main()
{
    let w            = 800;
    let h            = 600;
    let max_depth    = 50;
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

    let mut image  = ImagePPM::new_filled(w, h, Vec3::zero());
    let mut camera = Camera::new(cam_fov, cam_aspect_ratio, cam_aperture, cam_focus_dist);

    camera.move_to( cam_pos );
    camera.look_at( cam_target );

    let scene = rand_scene::new();

    let start = Instant::now();
    for y in 0..h
    {
#[cfg(debug_assertions)]
        println!("Scanline {} / {}", y+1, h);

        for x in 0..w
        {
            let mut color = Vec3::zero();
            for _ in 0..sample_count
            {
                let offset = if sample_count > 1 { random::<f32>() } else { 0.0 };
                // NOTE: 0,0 is lower left
                let u = (x as f32 + offset) / (w-1) as f32;
                let v = ((h-y) as f32 + offset) / (h-1) as f32;

                color += compute_ray(camera.get_ray(u, v), &scene, max_depth);
            }

            // Gamma2 correction
            color = (color * (1.0 / sample_count as f32)).sqrt();
            image.set_pixel(x, y, color);
        }
    }
    println!("RENDER TIME: {} ms", start.elapsed().as_millis());

    image.to_file("out.ppm");
}
