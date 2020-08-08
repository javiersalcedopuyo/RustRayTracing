mod utils;
mod ray;
mod camera;

use utils::ppm::ImagePPM;
use utils::vec3::Vec3;
use ray::Ray;
use camera::Camera;

use std::time::{Duration, Instant};

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32
{
    let oc = ray.origin - center;

    // (R(t)-C)(R(t)-C) = r^2
    // R(t) = O + Dt
    // (O+Dt-c)(O+Dt-C) = r^2
    // D·Dt^2 + 2D·(O-C)t + (O-C)·(O-C) - r^2 = 0

    let a = ray.direction.dot( ray.direction );
    let b = ray.direction.dot(oc) * 2.0;
    let c = oc.dot(oc) - radius*radius;

    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn compute_ray(ray: Ray) -> Vec3
{
    let sphere_center = Vec3::init(0.0, 0.0, -1.0);
    let t = hit_sphere(sphere_center, 0.4, &ray);

    if t > 0.0 {
        let n = (ray.at(t) - sphere_center).normalized();
        return Vec3::init(n.x()+1.0, n.y()+1.0, n.z()+1.0) * 0.5;
    }
    let dir = ray.direction.normalized();
    let t   = 0.5 * (dir.y() + 1.0);

    return Vec3::lerp(Vec3::init(0.0, 0.4, 1.0), Vec3::init(1.0, 1.0, 1.0), t);
}

fn main()
{
    let w = 800;
    let h = 600;
    let mut image  = ImagePPM::new_filled(w, h, Vec3::new());
    let mut camera = Camera::new();

    let aspect_ratio = (w as f32) / (h as f32);
    camera.resize(2.0*aspect_ratio, 2.0);

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
            let color = compute_ray(ray);

            image.set_pixel(x, y, color);
        }
    }
    println!("RENDER TIME: {} ms", start.elapsed().as_millis());

    image.to_file("out.ppm");
}
