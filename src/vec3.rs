use std::ops::{ Add, Mul, Div, Sub };

#[derive(Copy, Clone)]
pub struct Vec3
{
    data: [f32; 3]
}

impl Vec3
{
    pub fn new() -> Self { Self{data: [0.0, 0.0, 0.0]} }

    pub fn set(&mut self, x: f32, y: f32, z: f32)
    {
        self.data[0] = x;
        self.data[1] = y;
        self.data[2] = z;
    }

    pub fn r(&self) -> f32 { self.data[0] }
    pub fn g(&self) -> f32 { self.data[1] }
    pub fn b(&self) -> f32 { self.data[2] }
    pub fn x(&self) -> f32 { self.data[0] }
    pub fn y(&self) -> f32 { self.data[1] }
    pub fn z(&self) -> f32 { self.data[2] }
}

impl Add for Vec3
{
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3
    {
        Vec3 { data:
        [
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z()
        ]}
    }
}

impl Sub for Vec3
{
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3
    {
        Vec3 { data:
        [
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z()
        ]}
    }
}

impl Mul<f32> for Vec3
{
    type Output = Vec3;
    fn mul(self, val: f32) -> Vec3
    {
        Vec3 { data:
        [
            self.x() * val,
            self.y() * val,
            self.z() * val
        ]}
    }
}

impl Div<f32> for Vec3
{
    type Output = Vec3;
    fn div(self, val: f32) -> Vec3 { self * (1.0/val) }
}