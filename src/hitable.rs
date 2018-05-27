use material::{Lambertian, Material};
use ray::Ray;
use vector::Vec3;

pub trait Hitable {
    // TODO Think about returning an Intersected enum to indicate intersection
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<Material>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Box::new(Lambertian::new(Vec3::new(0.0, 0.0, 0.0))),
        }
    }
}
