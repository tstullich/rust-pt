use std::fmt;

use aabb::AABB;
use material::Material;
use ray::Ray;
use vector::Vec3;

pub trait Hitable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
}

impl fmt::Debug for Hitable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hitable {{ aabb: {:?} }}", self.bounding_box())
    }
}

/// A data structure that holds some info about the object that was
/// intersected in the scene
#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Material::Lambertian(Vec3::new(0.0, 0.0, 0.0)),
        }
    }
}
