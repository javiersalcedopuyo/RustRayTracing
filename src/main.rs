mod utils;
mod ray;
mod camera;
mod hittables;

use utils::ppm::ImagePPM;
use utils::vec3::Vec3;
use ray::Ray;
use camera::Camera;
use hittables::hittable::Hittable;
use hittables::hittable::HitRecord;
use hittables::sphere::Sphere;

use std::time::Instant;

fn compute_ray(i_ray: Ray, i_scene: &Vec<Box<dyn Hittable>>) -> Vec3
{
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
        let n = closest_hit.unwrap().normal;
        return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
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

    let aspect_ratio = (w as f32) / (h as f32);
    camera.resize(2.0*aspect_ratio, 2.0);

    let mut scene: Vec<Box<dyn Hittable>> = Vec::new();
    scene.push(Box::new(Sphere::init(0.3,   Vec3::new(0.0, 0.0, -1.0))));
    scene.push(Box::new(Sphere::init(100.0, Vec3::new(0.0,101.5, -1.0))));

    let start = Instant::now();
    for y in 0..h
    {
        #[cfg(debug_assertions)]
            println!("Scanline {} / {}", y+1, h);

        for x in 0..w
        {
            let u = (x as f32) / (w-1) as f32;
            let v = (y as f32) / (h-1) as f32;

            let hrz = camera.left    * camera.viewport.width;
            let vrt = camera.up      * camera.viewport.height;
            let dpt = camera.forward * camera.focal_len;

            let lower_left_corner = camera.origin - hrz*0.5 - vrt*0.5 - dpt;

            let pixel_pos = lower_left_corner + hrz*u + vrt*v;

            let ray   = Ray::new(camera.origin, pixel_pos - camera.origin);
            let color = compute_ray(ray, &scene);

            image.set_pixel(x, y, color);
        }
    }
    println!("RENDER TIME: {} ms", start.elapsed().as_millis());

    image.to_file("out.ppm");
}
