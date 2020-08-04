use std::path::Path;
use std::io::Write;
use std::fs::File;

#[derive(Clone)]
pub struct RGB
{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub struct ImagePPM
{
    pub width:  u32,
    pub height: u32,
    pub pixels: Vec<RGB>
}

impl ImagePPM
{
    pub fn new_filled(w:u32, h:u32, p:RGB) -> Self
    {
        Self
        {
            width:  w,
            height: h,
            pixels: vec![p; (w*h) as usize]
        }
    }

    pub fn size(&self) -> usize { return self.pixels.len(); }

    pub fn set_pixel(&mut self, x: u32, y: u32, i_val: RGB)
    {
        let idx = ((y * self.width) + x) as usize;
        self.pixels[idx] = i_val;
    }

    pub fn get_data(&self) -> Vec<u8>
    {
        let mut result = vec![0; self.size()*3];

        let mut i = 0;
        for pixel in &self.pixels
        {
            result[i+0] = pixel.r;
            result[i+1] = pixel.g;
            result[i+2] = pixel.b;

            i += 3;
            if i >= result.len() { break; }
        }
        return result;
    }

    pub fn to_file(&self, i_file_name: &str)
    {
        let     path   = Path::new(i_file_name);
        let mut file   = File::create(path).unwrap();
        let     header = format!("P6 {} {} 255\n", self.width, self.height);

        file.write(header.as_bytes()).unwrap();
        file.write(self.get_data().as_slice()).unwrap();
    }
}

