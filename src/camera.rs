use super::utils::vec3::Vec3;

pub struct Rect
{
    pub width:  f32,
    pub height: f32
}

pub struct Camera
{
    pub focal_len: f32,
    pub viewport:  Rect,
    pub origin:    Vec3,
    pub forward:   Vec3,
    pub up:        Vec3,
    pub left:      Vec3
}

impl Rect
{
    pub fn new(width: f32, height: f32) -> Self { Self{width, height} }
}

impl Camera
{
    pub fn new() -> Self
    {
        Self
        {
            focal_len : 1.0,
            viewport  : Rect::new(1.0, 1.0),
            origin    : Vec3::new(),
            forward   : Vec3::init(0.0, 0.0, 1.0),
            up        : Vec3::init(0.0, 1.0, 0.0),
            left      : Vec3::init(-1.0, 0.0, 0.0)
        }
    }

    pub fn resize(&mut self, width: f32, height: f32)
    {
        self.viewport = Rect::new(width, height);
    }
}