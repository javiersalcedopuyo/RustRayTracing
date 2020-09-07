mod utils;
mod ray;
mod camera;
mod hittables;
mod materials;
mod scene_generator;
mod ray_tracer;

use ray_tracer::RayTracer;
use std::time::Instant;

fn main()
{
    let raytracer = RayTracer::init();
    let start     = Instant::now();

    let image     = raytracer.render();

    println!("RENDER TIME: {} ms", start.elapsed().as_millis());

    image.to_file("out.ppm");
}
