mod utils;
mod ray;
mod camera;
mod hittables;

use rand::random;
use utils::ppm::ImagePPM;
use utils::vec3::Vec3;
use ray::Ray;
use camera::Camera;
use hittables::hittable::Hittable;
use hittables::hittable::HitRecord;
use hittables::sphere::Sphere;

use std::time::Instant;

fn compute_ray(i_ray: Ray, i_scene: &Vec<Box<dyn Hittable>>, i_depth: i32) -> Vec3
{
    if i_depth <= 0 { return Vec3::zero(); }

    let mut closest_hit: Option<HitRecord> = None;
    for obj in i_scene
    {
        let hit = obj.hit(&i_ray, 0.0, 10.0);

        if hit.is_some() &&
           (closest_hit.is_none() || hit.unwrap().distance < closest_hit.unwrap().distance)
        {
            closest_hit = hit;
        }
    }

    if closest_hit.is_some()
    {
        let hit    = closest_hit.unwrap();
        //let target = hit.position + hit.normal + utils::rand_point_in_unit_sphere();
        let target = hit.position + utils::rand_point_in_unit_hemisphere(hit.normal);
        return compute_ray(Ray::new(hit.position, target-hit.position), i_scene, i_depth-1) * 0.5;
    }

    let dir = i_ray.direction.normalized();
    let t   = 0.5 * (dir.y() + 1.0);

    return Vec3::lerp(Vec3::new(0.0, 0.4, 1.0), Vec3::new(1.0, 1.0, 1.0), t);
}

fn main()
{
    let w = 800;
    let h = 600;
    let mut image  = ImagePPM::new_filled(w, h, Vec3::zero());
    let mut camera = Camera::new();
    let sample_count = 100;
    let max_depth    = 50;

    let aspect_ratio = (w as f32) / (h as f32);
    camera.resize(2.0*aspect_ratio, 2.0);

    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();
    scene.push(Box::new(Sphere::init(0.3,   Vec3::new(0.0, 0.0, 1.0))));
    scene.push(Box::new(Sphere::init(100.0, Vec3::new(0.0,-100.5, 1.0))));

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
