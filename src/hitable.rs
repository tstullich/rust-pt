use ray::Ray;
use vector::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    // TODO Think about returning an Intersected enum to indicate intersection
    fn hit(self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}
