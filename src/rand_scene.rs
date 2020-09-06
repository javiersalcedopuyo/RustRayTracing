use super::utils::{vec3::Vec3};
use super::hittables::{ Intersectionable, sphere::Sphere};
use super::materials::{Material,
                       lambertian::LambertianMat,
                       metallic::MetallicMat,
                       dielectric::DielectricMat};

use std::rc::Rc;

pub fn new() -> Vec<Intersectionable>
{
    let mut result: Vec<Intersectionable> = Vec::new();

    let ground_mat = Rc::new( LambertianMat{albedo: Vec3::new(0.5, 0.75, 0.0)} );
    result.push( Intersectionable::Sphere( Sphere::new( 1000.0,
                                                        Vec3::new(0.0, -1000.0, 0.0),
                                                        ground_mat.clone()) ) );

    for a in -11..11 {
        for b in -11..11
        {
            let center = Vec3::new(a as f32 + 0.9 * rand::random::<f32>(),
                                   0.2,
                                   b as f32 + 0.9 * rand::random::<f32>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).norm() <= 0.9 { continue; }

            let albedo:  Vec3;
            let new_mat: Rc<dyn Material>;

            let dice = rand::random::<f32>();

            if dice < 0.8
            { // Diffuse
                albedo  = Vec3::rand(0.0, 1.0) * Vec3::rand(0.0, 1.0);
                new_mat = Rc::new( LambertianMat{ albedo } );
            }
            else if dice < 0.95
            { // Metal
                let rgh = super::utils::rand_f32_in_range(0.0, 0.5);
                albedo  = Vec3::rand(0.5, 1.0);
                new_mat = Rc::new( MetallicMat::new(rgh, albedo) );
            }
            else
            { // Glass
                albedo  = Vec3::one();
                new_mat = Rc::new( DielectricMat::new(1.5, albedo) );
            }

            result.push( Intersectionable::Sphere( Sphere::new(0.2, center, new_mat) ) );
        }
    }

    let material1 = Rc::new( DielectricMat::new( 1.5, Vec3::one()) );
    result.push( Intersectionable::Sphere( Sphere::new(1.0, Vec3::new(0.0, 1.0, 0.0), material1) ) );

    let material2 = Rc::new( LambertianMat{ albedo: Vec3::new(0.4, 0.2, 0.1) } );
    result.push( Intersectionable::Sphere( Sphere::new(1.0, Vec3::new(-4.0, 1.0, 0.0), material2) ) );

    let material3 = Rc::new( MetallicMat::new(0.0, Vec3::new(0.7, 0.6, 0.5) ) );
    result.push( Intersectionable::Sphere( Sphere::new(1.0, Vec3::new(4.0, 1.0, 0.0), material3) ) );

    return result;
}