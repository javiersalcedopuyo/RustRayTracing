use std::ops::{ Add, AddAssign, Mul, MulAssign, Div, Sub, Neg };
use float_cmp::approx_eq;
use super::TAU;

#[derive(Copy, Clone, Debug)]
pub struct Vec3
{
    data: [f32; 3]
}

impl Vec3
{
    pub fn zero() -> Self { Self{ data: [0.0, 0.0, 0.0] } }
    pub fn one()  -> Self { Self{ data: [1.0, 1.0, 1.0] } }
    pub fn debug_color() -> Self { Self{ data: [1.0, 0.0, 1.0] } }
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

    pub fn cross(&self, other: Self) -> Self
    {
        Self{ data:
        [
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x()
        ]}
    }

    pub fn norm2(&self)      -> f32  { return self.dot( *self );   }
    pub fn norm(&self)       -> f32  { return self.norm2().sqrt(); }
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

    pub fn rand_unit() -> Self
    {
        let a = rand::random::<f32>() * (TAU);
        let z = -1.0 + rand::random::<f32>() * 2.0;
        let r = (1.0 - z*z).sqrt();

        return Vec3::new(r*a.cos(), r*a.sin(), z);
    }

    pub fn sqrt(&self) -> Self
    {
        let x = self.x().sqrt();
        let y = self.y().sqrt();
        let z = self.z().sqrt();

        return Vec3::new(x,y,z);
    }

    pub fn reflect(&self, n: Vec3) -> Vec3
    {
        return *self - n * self.dot(n) * 2.0;
    }

    // n: Surface normal
    // eta: Refraction index of the first medium over the second's (ni/nr)
    pub fn refract(&self, n: Vec3, eta: f32) -> Vec3
    {
        let cos_theta    = (-*self).dot(n);
        let r_orthogonal = (*self + n * cos_theta) * eta;
        let r_parallel   = n * -(1.0 - r_orthogonal.norm2()).abs().sqrt();

        return r_orthogonal + r_parallel;
    }
}

// Override operators
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

impl Mul<Vec3> for Vec3
{
    type Output = Vec3;
    fn mul(self, val: Vec3) -> Vec3
    {
        Vec3 { data:
        [
            self.x() * val.x(),
            self.y() * val.y(),
            self.z() * val.z()
        ]}
    }
}

impl MulAssign<f32> for Vec3
{
    fn mul_assign(&mut self, val: f32) { *self = *self * val }
}

impl MulAssign<Vec3> for Vec3
{
    fn mul_assign(&mut self, other: Vec3) { *self = *self * other }
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
        approx_eq!(f32, self.x(), other.x()) &&
        approx_eq!(f32, self.y(), other.y()) &&
        approx_eq!(f32, self.z(), other.z())
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn normalize()
    {
        let norm = Vec3::rand(0.0, 5.0)
                        .normalized()
                        .norm();

        assert!( approx_eq!( f32, norm, 1.0 ) );
    }

    #[test]
    fn dot_product()
    {
        let vec_x = Vec3::new(1.0, 0.0, 0.0);
        let vec_y = Vec3::new(0.0, 1.0, 0.0);

        assert!( approx_eq!(f32, vec_x.dot( vec_y ), 0.0) );
        assert!( approx_eq!(f32, vec_x.dot( vec_x ), 1.0) );
        assert!( approx_eq!(f32, vec_x.dot( -vec_x ), -1.0) );
    }

    #[test]
    fn cross_product()
    {
        let vec_x = Vec3::new(1.0, 0.0, 0.0);
        let vec_y = Vec3::new(0.0, 1.0, 0.0);
        let vec_z = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!( vec_x.cross( vec_y ), vec_z );
    }

    #[test]
    fn random_unit_vector()
    {
        let vector = Vec3::rand_unit();
        assert!( approx_eq!(f32, vector.norm(), 1.0) );
    }

    #[test]
    fn reflect()
    {
        let vec_x = Vec3::new(1.0, 0.0, 0.0);
        let vec_y = Vec3::new(0.0, 1.0, 0.0);
        let plane_norm_at_45_degrees = Vec3::new(-1.0, 1.0, 0.0)
                                        .normalized();

        assert_eq!( vec_x.reflect( -vec_x ), -vec_x );
        assert_eq!( vec_x.reflect( plane_norm_at_45_degrees ), vec_y );
    }

    #[test]
    #[ignore] // FIXME
    fn refract()
    {
        let normal   = Vec3::new(-1.0, 0.0, 0.0);
        let inciding = Vec3::new(1.0, 1.0, 0.0)
                        .normalized();

        let eta_1 = 1.0;
        let eta_2 = 1.0 / 1.5;

        // No refraction
        assert_eq!( inciding.refract( normal, eta_1 ), inciding );
        // 28.1255 degree refraction
        let refracted = inciding.refract( normal, eta_2 )
                                .normalized();

        let cos_angle = inciding.dot( refracted );
        let expected  = (28.1255 as f32).cos();

        println!("TRACE! Actual: {} vs Expected: {}", cos_angle, expected);
        assert!( approx_eq!( f32, cos_angle, expected ) );
    }
}
