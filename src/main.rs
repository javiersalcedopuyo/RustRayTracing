mod utils;

use utils::ppm::ImagePPM;
use utils::vec3::Vec3;

//const GREEN :RGB = RGB{ r:0.0, g:1.0, b:0.0 };
//const BLUE  :RGB = RGB{ r:0.0, g:0.0, b:1.0 };

fn main()
{
    let w = 800;
    let h = 600;
    let mut image = ImagePPM::new_filled(w, h, Vec3::new());

    for y in 0..h
    {
        println!("Scanline {} / {}", y+1, h);

        for x in 0..w
        {
            let r = x as f32 / (w-1) as f32;
            let g = y as f32 / (h-1) as f32;
            let b = 0.25;

            let mut pixel = Vec3::new();
            pixel.set(r,g,b);

            image.set_pixel(x, y, pixel);
        }
    }

    image.to_file("out.ppm");
}
