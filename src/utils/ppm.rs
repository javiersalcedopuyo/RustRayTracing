use std::path::Path;
use std::io::Write;
use std::fs::File;

use super::vec3::Vec3;

pub struct ImagePPM
{
    pub width:  u32,
    pub height: u32,
    pub pixels: Vec<Vec3>
}

impl ImagePPM
{
    pub fn new_filled(w:u32, h:u32, p:Vec3) -> Self
    {
        Self
        {
            width:  w,
            height: h,
            pixels: vec![p; (w*h) as usize]
        }
    }

    pub fn size(&self) -> usize { return self.pixels.len(); }

    pub fn set_pixel(&mut self, x: u32, y: u32, i_val: Vec3)
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
            result[i+0] = (pixel.r() * 255.0).round() as u8;
            result[i+1] = (pixel.g() * 255.0).round() as u8;
            result[i+2] = (pixel.b() * 255.0).round() as u8;

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

