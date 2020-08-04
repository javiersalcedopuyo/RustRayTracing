mod ppm;
use ppm::{RGB, ImagePPM};

fn main()
{
    let w = 800;
    let h = 600;
    let mut image = ImagePPM::new_filled(w, h, RGB{r: 255, g:0, b:0});

    const GREEN :RGB = RGB{ r:0, g:255, b:0 };
    const BLUE  :RGB = RGB{ r:0, g:0,   b:255 };

    for x in 0..w {
        for y in 0..h
        {
            if y > h/3 && y <= 2*h/3
            {
                image.set_pixel(x, y, GREEN);
            }
            else if y > 2*h/3
            {
                image.set_pixel(x, y, BLUE);
            }
        }
    }

    image.to_file("out.ppm");
}
