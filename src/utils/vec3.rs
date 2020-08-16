use std::ops::{ Add, AddAssign, Mul, MulAssign, Div, Sub, Neg };

#[derive(Copy, Clone, Debug)]
pub struct Vec3
{
    data: [f32; 3]
}

impl Vec3
{
    pub fn zero() -> Self { Self{ data: [0.0, 0.0, 0.0] } }
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self{ data: [x,y,z] } }

    //pub fn set(&mut self, x: f32, y: f32, z: f32)
    //{
    //    self.data[0] = x;
    //    self.data[1] = y;
    //    self.data[2] = z;
    //}

    pub fn r(&self) -> f32 { self.data[0] }
    pub fn g(&self) -> f32 { self.data[1] }
    pub fn b(&self) -> f32 { self.data[2] }
    pub fn x(&self) -> f32 { self.data[0] }
    pub fn y(&self) -> f32 { self.data[1] }
    pub fn z(&self) -> f32 { self.data[2] }

    pub fn dot(&self, other: Self) -> f32
    {
        return self.x() * other.x() + self.y() * other.y() + self.z() * other.z();
    }

    // TODO: pub fn cross(&self, other: Self) -> Self

    pub fn norm2(&self) -> f32
    {
        return self.x() * self.x() + self.y() * self.y() + self.z() * self.z();
    }

    pub fn norm(&self) -> f32 { return self.norm2().sqrt(); }
    pub fn normalized(&self) -> Self { return *self / self.norm(); }

    pub fn lerp(a: Self, b: Self, t: f32) -> Self
    {
        if      t >= 1.0 { return b; }
        else if t <= 0.0 { return a; }
        else             { return a * (1.0-t) + b * t;}
    }

    pub fn rand(min: f32, max: f32) -> Self
    {
        let x = min + rand::random::<f32>() * (max-min);
        let y = min + rand::random::<f32>() * (max-min);
        let z = min + rand::random::<f32>() * (max-min);

        return Vec3::new(x,y,z);
    }

    pub fn sqrt(&self) -> Self
    {
        let x = self.x().sqrt();
        let y = self.y().sqrt();
        let z = self.z().sqrt();

        return Vec3::new(x,y,z);
    }
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

impl AddAssign for Vec3
{
    fn add_assign(&mut self, other: Vec3) { *self = *self + other; }
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

impl MulAssign<f32> for Vec3
{
    fn mul_assign(&mut self, val: f32) { *self = *self * val; }
}

impl Div<f32> for Vec3
{
    type Output = Vec3;
    fn div(self, val: f32) -> Vec3 { self * (1.0/val) }
}

impl Neg for Vec3
{
    type Output = Vec3;
    fn neg(self) -> Vec3 { Vec3::new(-self.x(), -self.y(), -self.z()) }
}

impl PartialEq for Vec3
{
    fn eq(&self, other: &Vec3) -> bool
    {
        self.x() == other.x() &&
        self.y() == other.y() &&
        self.z() == other.z()
    }
}
impl Eq for Vec3 {}