use hitable::HitRecord;
use ray::Ray;
use vector::Vec3;

pub trait Material {
    fn scatter(ray_in: &Ray, rec: &HitRecord, attenuation: &Vec3, scattered: &mut Ray);
}
